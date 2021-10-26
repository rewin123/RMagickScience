use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct Render {
    instance : wgpu::Instance,
    surface : wgpu::Surface,
    adapter : wgpu::Adapter,
    device : wgpu::Device,
    queue : wgpu::Queue,
    swapchain_format : wgpu::TextureFormat
}


impl Render {
    pub fn new(
        window : &Window
    ) -> Render {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe {instance.create_surface(window)};

        let adapter = futures::executor::block_on(instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })).expect("Failed to find an appropriate adapter");
        
        let (device, queue) = futures::executor::block_on(adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        ))
        .expect("Failed to create device");

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &config);
    
        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swapchain_format
        }
    }
}