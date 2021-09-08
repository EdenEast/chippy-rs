#![allow(unused_variables)]

use chippy::emu::{self, vm::Vm};
use emu::gpu;
use eyre::{eyre, Result, WrapErr};
use log::error;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const PIXEL_SIZE: u32 = 16;

fn update_buffer(gpu: &gpu::Gpu, frame: &mut [u8]) {
    let mut index = 0;
    let width = gpu::SCREEN_WIDTH * PIXEL_SIZE as usize;
    for pixel in frame.chunks_exact_mut(4) {
        let x = (index % width) / 16;
        let y = (index / width) / 16;
        let state = gpu.get(x, y);

        let value = match state {
            true => [0xCD, 0xCE, 0xCF, 0xFF],
            false => [0x19, 0x23, 0x30, 0xFF],
        };

        pixel.copy_from_slice(&value);
        index += 1;
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let size = PhysicalSize::new(
        gpu::SCREEN_WIDTH as u32 * PIXEL_SIZE,
        gpu::SCREEN_HEIGHT as u32 * PIXEL_SIZE,
    );

    let scale_factor = 1.0;

    let romfile = std::env::args()
        .nth(1)
        .ok_or(eyre!("Missing rom file in arguments"))?;
    let bytes = std::fs::read(romfile).wrap_err("Failed to open c8 file")?;
    let mut vm = Vm::new();
    vm.load(bytes);

    println!("{:#?}", size);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(size.to_logical::<f64>(1.0))
        .with_title("Chippy")
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = pixels::SurfaceTexture::new(size.width, size.height, &window);
        pixels::Pixels::new(size.width, size.height, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            }
            | Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                // Handle keystate
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                pixels.resize_surface(size.width, size.height);
            }
            // Event::WindowEvent {
            //     event:
            //         WindowEvent::ScaleFactorChanged {
            //             scale_factor,
            //             new_inner_size,
            //         },
            //     ..
            // } => {
            //     pixels.resize_surface(new_inner_size .width, new_inner_size .height);
            // }
            Event::MainEventsCleared => {
                match vm.cycle() {
                    emu::vm::ProgramState::Continue => {}
                    emu::vm::ProgramState::Stop => *control_flow = ControlFlow::Exit,
                }

                window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                update_buffer(&vm.gpu, pixels.get_frame());

                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }
    });
}
