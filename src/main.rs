use std::time::Instant;

use hoyer_chip_8::{
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    display::window::create_window,
    interpreter::interpreter::Interpreter,
};
use pixels::{wgpu, Pixels, PixelsBuilder, SurfaceTexture};
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
    let mut pixels = PixelsBuilder::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)
        .present_mode(wgpu::PresentMode::Immediate)
        .request_adapter_options(wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
        })
        .wgpu_backend(wgpu::BackendBit::DX12)
        .build()
        .unwrap();
    let mut interpreter = Interpreter::new("./roms/games/PONG.c8");

    let mut time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            interpreter.draw(pixels.get_frame());
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
                if size.height != size.width / 2 {
                    window.set_inner_size(PhysicalSize::new(size.width, size.width / 2));
                    pixels.resize_surface(size.width, size.width / 2);
                }
            }

            interpreter.update_inputs(&input);
            interpreter.update();
            window.request_redraw();
        }
    });
}
