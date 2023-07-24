use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::vec3::Vec3;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
}

impl Window {
    pub fn render_loop<F, G>(&self, mut on_redraw_requested: G, mut on_frame_updated: F)
    where F :'static + FnMut(&WinitInputHelper,  &mut ControlFlow), G:  'static + FnMut() -> Vec<Vec3<u8>> {
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
                let image = on_redraw_requested();
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
    
            if input.update(&event) {
                window.request_redraw();
                on_frame_updated(&input, control_flow);
            }
        });
    }
}