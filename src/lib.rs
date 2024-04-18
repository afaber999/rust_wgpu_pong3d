

use camera::Camera;
use glam::Mat4;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winit::window::Window;

pub mod math;
pub mod errors;
pub mod buffers;
pub mod geometries;
pub mod renderers;
pub mod texture2d;
pub mod camera;

use geometries::{CubeGeometry, QuadGeometry};


struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    camera : Camera,
    renderer : renderers::unlit_material::UnlitMaterial,
    depth_texture : texture2d::Texture2d,

    //render_pipeline : wgpu::RenderPipeline,
    window: Window,
}


impl State {

    fn camera_mat(width: u32, height: u32) -> Mat4 {
        // let mat = Mat4::orthographic_lh(
        //     -1.0,1.0,
        //     -1.0,1.0,
        //     0.0,1.0 );

        let fov = 90.0_f32.to_radians();
        let z_near = 0.01_f32;
        let z_far = 5_f32;
        let aspect_ratio  = width as f32 / height as f32;

        Mat4::perspective_lh(
            fov, aspect_ratio,
            z_near, z_far)
    }

    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
  
         let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);


        // DEPTH buffer
        let depth_texture = texture2d::Texture2d::create_depth_texture(&device, &config, "depth_texture");


        //let geo = QuadGeometry::new();
        let geo = CubeGeometry::new();


                
        let camera = Camera::new(
            &device,
            Self::camera_mat(size.width, size.height),
            "Main camera" );

        let renderer = renderers::unlit_material::UnlitMaterial::new(
                &device, 
                &queue,
                config.format, 
                geo.positions,
                geo.colors,
                geo.tex_coords,
                geo.indices,
                &camera);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            camera,
            renderer,
            depth_texture,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    // impl State
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.camera.update(
                &self.queue, 
                Self::camera_mat(self.size.width, self.size.height));

            self.surface.configure(&self.device, &self.config);
            self.depth_texture = texture2d::Texture2d::create_depth_texture(
                &self.device, 
                &self.config,
                "depth_texture");
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.renderer.update(&self.queue);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // _render_pass must have limited scope in order to release the encoder
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, // to where we rendering to, in this case a view on the window output texture
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.8,
                            g: 0.8,
                            b: 0.8,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.renderer.draw(&mut render_pass, &self.camera);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())                
    }
}


pub async fn run() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {

        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once unless we manually
            // request it.
            state.window().request_redraw();
        }
        
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id()  => if !state.input(event) {
            
            match event {
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    state.resize(**new_inner_size);
                }

                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,

                _ => {}
            }
        }
        _ => {}
    });
}
