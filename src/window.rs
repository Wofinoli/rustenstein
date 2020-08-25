use winit::{
    event::{KeyboardInput, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
    error::OsError,
};

#[derive(Debug)]
pub struct WindowState {
    pub window: Window,

    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    size: winit::dpi::PhysicalSize<u32>,
}

pub const WINDOW_NAME: &str = "Rustenstein";

impl WindowState {
    /// Constructs a new `EventsLoop` and `Window` pair.
    ///
    /// The specified title and size are used, other elements are default.
    /// ## Failure
    /// Window creating may fail, but this is unlikely
    pub async fn new<T: Into<String>>(title: T, size: LogicalSize<f64>) 
            -> Result<(EventLoop<()>, WindowState), OsError> {
        let event_loop = EventLoop::new();
        let output = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(&event_loop);
        
        let window = output.unwrap();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY); // Vulkan + Metal + DX12 + Browser WebGPU
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
        ).await.unwrap();


        let size = window.inner_size();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::DEPTH_CLAMPING,
                limits: Default::default(),
                shader_validation: false,
            },
            None,
        ).await.expect("Device request failed.");

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Ok( (event_loop, 
            Self {
                window,
                instance,
                surface,
                adapter,
                device,
                queue,
                sc_desc,
                swap_chain,
                size,
            } ) )
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    // input() won't deal with GPU code, so it can be synchronous
    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
    }

    pub fn render(&mut self) {
        let frame = self.swap_chain.get_current_frame()
        .expect("Timeout getting texture");

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations::<wgpu::Color> {
                            load: wgpu::LoadOp::Clear( wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });
        }
    
        let buffer = &[encoder.finish()];
        self.queue.submit(buffer);

    }


    /// Takes keyboard input and directs it to the
    /// appropriate place.
    pub fn handle_key_input(input: KeyboardInput) {
        println!("{:?}", input);
    }

    /// Makes an 640x480 windwow with `WINDOW_NAME` as the title.
    /// ## Panics
    /// If an `OsError` occurs.
    pub async fn default() -> Result<(EventLoop<()>, WindowState ), OsError> {
        WindowState::new(
            WINDOW_NAME,
            LogicalSize {
                width: 640.0,
                height: 480.0,
            },
        ).await
    }
}
