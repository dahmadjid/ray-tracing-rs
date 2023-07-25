use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::scene::Scene;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
}

impl Window {
    pub fn render_loop(&self, scene: Scene) {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(self.width, self.height);
            WindowBuilder::new()
                .with_title(self.title) 
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(self.width, self.height, surface_texture).unwrap()
        };
        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                let image = scene.render();
                for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                    pixel[0] = image[i].x;
                    pixel[1] = image[i].y;
                    pixel[2] = image[i].z;
                    pixel[3] = 0xff;
                }
                if let Err(err) = pixels.render() {
                    println!("pixels.render erorr: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
    
            // Resize the window
            if input.update(&event) {
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("pixels.resize_surface error: {}", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
        
                if input.key_pressed(VirtualKeyCode::Up) {
                }
        
                if input.key_pressed(VirtualKeyCode::Down) {
                }
            }
            window.request_redraw();
        });
    }
}