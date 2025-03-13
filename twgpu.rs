use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    platform::wayland::EventLoopWindowTargetExtWayland,
    window::WindowBuilder,
};

pub fn run() -> Result<(), winit::error::EventLoopError> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("wtf")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                _ => {}
            },
            _ => {}
        };
        println!("{:?}", control_flow.primary_monitor());
        println!("{:?}", event);
        println!("{}", control_flow.is_wayland());
    })
}
