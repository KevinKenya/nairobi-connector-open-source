use wgpu::*;

pub struct HeadlessContext {
    pub device: Device,
    pub queue: Queue,
    pub instance: Instance,
    pub adapter: Adapter,
}

impl HeadlessContext {
    pub async fn new() -> Self {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let mut adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await;

        if adapter.is_none() {
            log::info!("HighPerformance adapter not found, trying software fallback");
            adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::None,
                    compatible_surface: None,
                    force_fallback_adapter: true,
                })
                .await;
        }

        let adapter = adapter.expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Headless Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        Self {
            device,
            queue,
            instance,
            adapter,
        }
    }

    pub fn create_offscreen_texture(&self, width: u32, height: u32) -> Texture {
        self.device.create_texture(&TextureDescriptor {
            label: Some("Offscreen Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    }

    pub fn create_staging_buffer(&self, width: u32, height: u32) -> (Buffer, u32) {
        let bytes_per_pixel = 4;
        let unaligned_bytes_per_row = width * bytes_per_pixel;
        let align = COPY_BYTES_PER_ROW_ALIGNMENT;
        let aligned_bytes_per_row = (unaligned_bytes_per_row + align - 1) & !(align - 1);
        
        let buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Staging Buffer"),
            size: aligned_bytes_per_row as u64 * height as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        (buffer, aligned_bytes_per_row)
    }
}
