mod logger;
mod err;
mod instance;
mod files;
mod client;

pub mod common;

pub use common::math;

use {
    winit::{
        event::*,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    err::Error,
};

pub async fn run() -> Result<(), Error> {
    unsafe { logger::Logger::init(log::LevelFilter::Warn)? };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut client = client::Client::new(window).await?;


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == client.window_id() => {
                client.update();
                match client.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => client.resize(client.renderer.state.size.into()),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => log::error!("{:?}", e),
                }
            }

            Event::DeviceEvent { event, .. } => client.device_input(&event),

            Event::MainEventsCleared => {
                client.window.request_redraw();
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == client.window_id() => if !client.window_input(event) {
                match event {

                    WindowEvent::Resized(physical_size) => {
                        client.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        client.resize(**new_inner_size);
                    }

                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
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
        }
    });
}