use hoyer_chip_8::{
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    display::window::create_window,
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Hoyer's Chip-8 Interpreter", &event_loop);
    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap();
    let mut world = World::new();
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                println!("{}, {}", size.width, size.height);
                if size.height != size.width / 2 {
                    window.set_inner_size(PhysicalSize::new(size.width, size.width / 2));
                    pixels.resize_surface(size.width, size.width / 2);
                }
            }
            world.update();
            window.request_redraw();
        }
    });
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + 4 > WINDOW_WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + 4 > WINDOW_HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WINDOW_WIDTH as usize) as i16;
            let y = (i / WINDOW_WIDTH as usize) as i16;

            let inside_the_box =
                x >= self.box_x && x < self.box_x + 4 && y >= self.box_y && y < self.box_y + 4;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
