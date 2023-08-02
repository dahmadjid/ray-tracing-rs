use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize};
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
    pub fn render_loop(&self, mut scene: Scene) {
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
                        println!("ERROR: pixels.resize_surface: {}", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                
                if input.mouse_held(1) {
                    if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                        println!("ERROR: set_cursor_grab: {}", err);
                    }
                    let mut needs_recalculating = false;
                    if input.key_held(VirtualKeyCode::A) {
                        scene.camera.update_x_position(-0.1); 
                        // needs_recalculating = true;
                    }
            
                    if input.key_held(VirtualKeyCode::D) {
                        scene.camera.update_x_position(0.1); 
                        // needs_recalculating = true;
                    }

                    if input.key_held(VirtualKeyCode::Q) {
                        scene.camera.update_y_position(0.1); 
                        // needs_recalculating = true;
                    }
                    
                    if input.key_held(VirtualKeyCode::E) {
                        scene.camera.update_y_position(-0.1); 
                        // needs_recalculating = true;
                    }
                    
                    if input.key_held(VirtualKeyCode::W) {
                        scene.camera.update_z_position(0.1); 
                        // needs_recalculating = true;
                    }
                    
                    if input.key_held(VirtualKeyCode::S) {
                        scene.camera.update_z_position(-0.1); 
                        // needs_recalculating = true;
                    }
                    let yaw_angle = input.mouse_diff().0 * 0.001;
                    let pitch_angle = input.mouse_diff().1 * 0.001;
                    if yaw_angle != 0. && pitch_angle != 0. {
                        scene.camera.rotate(pitch_angle as f64, yaw_angle as f64);
                        needs_recalculating = true;
                    }

                    if needs_recalculating {
                        scene.camera.calculate_ray_directions();
                    }
                } else {
                    if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                        println!("ERROR: set_cursor_grab: {}", err);
                    }
                }
                
            } 
            window.request_redraw();
        });
    }
}