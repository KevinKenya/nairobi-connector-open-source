use crate::device::HeadlessContext;
use bytemuck::{Pod, Zeroable};
use egui_wgpu::ScreenDescriptor;
use std::sync::Arc;
use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct LttbPoint {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct LttbParams {
    input_count: u32,
    output_count: u32,
}

pub struct LagosPipeline {
    ctx: Arc<HeadlessContext>,
    compute_pipeline: ComputePipeline,
    egui_renderer: egui_wgpu::Renderer,
    egui_ctx: egui::Context,
}

impl LagosPipeline {
    pub fn new(ctx: Arc<HeadlessContext>) -> Self {
        let shader = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("LTTB Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/lttb.wgsl").into()),
        });

        let compute_pipeline = ctx.device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("LTTB Compute Pipeline"),
            layout: None,
            module: &shader,
            entry_point: "main",
        });

        let egui_renderer = egui_wgpu::Renderer::new(
            &ctx.device,
            TextureFormat::Rgba8UnormSrgb,
            None,
            1,
        );

        Self {
            ctx,
            compute_pipeline,
            egui_renderer,
            egui_ctx: egui::Context::default(),
        }
    }

    pub async fn process_and_render(
        &mut self,
        input_points: &[LttbPoint],
        output_count: u32,
        width: u32,
        height: u32,
        raw_input: egui::RawInput,
        render_ui: impl FnOnce(&egui::Context, &[LttbPoint]),
    ) -> Vec<u8> {
        let device = &self.ctx.device;
        let queue = &self.ctx.queue;

        // 1. Prepare LTTB buffers
        let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: bytemuck::cast_slice(input_points),
            usage: BufferUsages::STORAGE,
        });

        let output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Output Buffer"),
            size: (output_count as usize * std::mem::size_of::<LttbPoint>()) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let params = LttbParams {
            input_count: input_points.len() as u32,
            output_count,
        };
        let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Params Buffer"),
            contents: bytemuck::cast_slice(&[params]),
            usage: BufferUsages::UNIFORM,
        });

        let bind_group_layout = self.compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("LTTB Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: input_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        // 2. Dispatch Compute
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Lagos Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("LTTB Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            let workgroup_count = (output_count + 63) / 64;
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }

        // 3. Read back decimated points for egui (Phase 1 simplicity, in future we might use them in GPU)
        // Note: egui_plot expects points on CPU to build the mesh.
        let staging_output_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Staging Output Buffer"),
            size: (output_count as usize * std::mem::size_of::<LttbPoint>()) as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_output_buffer, 0, staging_output_buffer.size());

        queue.submit(Some(encoder.finish()));

        let (tx, rx) = std::sync::mpsc::channel();
        let buffer_slice = staging_output_buffer.slice(..);
        buffer_slice.map_async(MapMode::Read, move |v| tx.send(v).unwrap());
        device.poll(Maintain::Wait);
        rx.recv().unwrap().expect("Failed to map staging output buffer");

        let decimated_points: Vec<LttbPoint> = {
            let data = buffer_slice.get_mapped_range();
            bytemuck::cast_slice(&data).to_vec()
        };
        staging_output_buffer.unmap();

        // 4. Egui Rendering
        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            render_ui(ctx, &decimated_points);
        });

        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: full_output.pixels_per_point,
        };

        let target_texture = self.ctx.create_offscreen_texture(width, height);
        let target_view = target_texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Egui Render Encoder"),
        });

        // Upload egui textures
        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(device, queue, *id, image_delta);
        }

        self.egui_renderer.update_buffers(
            device,
            queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Egui Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &target_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
        }

        // 5. Copy Texture to Staging Buffer
        let staging_buffer = self.ctx.create_staging_buffer(width, height);
        let bytes_per_row = width * 4;
        encoder.copy_texture_to_buffer(
            ImageCopyTexture {
                texture: &target_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            ImageCopyBuffer {
                buffer: &staging_buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        queue.submit(Some(encoder.finish()));

        // 6. Map and extract
        let (tx, rx) = std::sync::mpsc::channel();
        let buffer_slice = staging_buffer.slice(..);
        buffer_slice.map_async(MapMode::Read, move |v| tx.send(v).unwrap());
        device.poll(Maintain::Wait);
        rx.recv().unwrap().expect("Failed to map staging buffer");

        let data = buffer_slice.get_mapped_range();
        let result = data.to_vec();
        drop(data);
        staging_buffer.unmap();

        // Clean up egui textures
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        result
    }
}

use wgpu::util::DeviceExt;
