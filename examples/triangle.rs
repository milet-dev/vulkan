use vulkan::prelude::*;

use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowButtons},
};

const TITLE: &str = "vulkan";

fn initialize_vulkan(handle: &impl raw_window_handle::HasRawWindowHandle) -> anyhow::Result<()> {
    let instance = Instance::new()?;
    let surface = Surface::new(handle, &instance)?;
    let physical_device = instance
        .physical_devices()?
        .into_iter()
        .find(|physical_device| {
            physical_device
                .properties
                .device_type
                .eq(&vk::PhysicalDeviceType::DISCRETE_GPU)
        })
        .unwrap();
    let device = Device::new(&instance, &surface, &physical_device)?;
    let swapchain = Swapchain::new(&instance, &surface, &physical_device, &device)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(TITLE)
        .with_resizable(false)
        .with_enabled_buttons(WindowButtons::MINIMIZE | WindowButtons::CLOSE)
        .build(&event_loop)?;

    initialize_vulkan(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            }
            | Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
            } => {
                if window_id == window.id() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
