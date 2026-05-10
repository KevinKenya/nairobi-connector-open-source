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

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

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

    pub fn create_staging_buffer(&self, width: u32, height: u32) -> Buffer {
        let u32_size = std::mem::size_of::<u32>() as u64;
        self.device.create_buffer(&BufferDescriptor {
            label: Some("Staging Buffer"),
            size: width as u64 * height as u64 * u32_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
