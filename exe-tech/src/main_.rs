use anyhow::Result;
use wgpu::TextureUsages;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

#[allow(clippy::print_stdout)]
#[tokio::main]
pub async fn main() -> Result<()> {
  println!("Context intialized");

  env_logger::builder()
    .filter(Some(module_path!()), log::LevelFilter::Info)
    .parse_default_env()
    .init();

  let event_loop = EventLoop::new().unwrap();

  let window = WindowBuilder::new().build(&event_loop).unwrap();

  let instance = wgpu::Instance::default();

  let surface = instance.create_surface(&window).unwrap();

  log::info!("Available adapters:");
  for a in instance.enumerate_adapters(wgpu::Backends::all()) {
    log::info!("    {:?}", a.get_info());
  }

  let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::HighPerformance,
      force_fallback_adapter: false,
      compatible_surface: Some(&surface),
    })
    .await
    .unwrap();

  log::info!("Selected adapter: {:?}", adapter.get_info());

  let (device, _queue) = adapter
    .request_device(
      &wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::downlevel_webgl2_defaults()
          .using_resolution(adapter.limits()),
        memory_hints: wgpu::MemoryHints::MemoryUsage,
      },
      None,
    )
    .await
    .expect("Failed to create device");

  //let swapchain_capabilities = surface.get_capabilities(&adapter);
  //let swapchain_format = swapchain_capabilities.formats[0];

  let mut size = window.inner_size();
  size.width = size.width.max(1);
  size.height = size.height.max(1);

  let mut config = surface
    .get_default_config(&adapter, size.width, size.height)
    .unwrap();
  config.usage = TextureUsages::TEXTURE_BINDING;
  surface.configure(&device, &config);

  let window = &window;
  event_loop.run(move |event, target| {
    let _ = (&instance, &adapter);

    if let Event::WindowEvent {
      window_id: _,
      event,
    } = event
    {
      match event {
        WindowEvent::Resized(new_size) => {
          // Reconfigure the surface with the new size
          config.width = new_size.width.max(1);
          config.height = new_size.height.max(1);
          surface.configure(&device, &config);
          // On macos the window needs to be redrawn manually after resizing
          window.request_redraw();
        }
        WindowEvent::RedrawRequested => {
          let surface_texture = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

          log::info!("Surface texture: {:?}", surface_texture);
          surface_texture.present();
        }
        WindowEvent::CloseRequested => target.exit(),
        _ => {}
      }
    }
  })?;

  Ok(())
}
