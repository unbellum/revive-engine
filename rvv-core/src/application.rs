pub struct Application
{
    pub name: String
}

pub fn run(app: Application)
{
    println!("Running application: {}", app.name);

    // Taken straight from the egui_glium examples. For now get something working.
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder.with_title(app.name), context_builder, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow|
    {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event
        {
            glutin::event::Event::WindowEvent { event, .. } => match event
            {
                glutin::event::WindowEvent::CloseRequested =>
                {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause
            {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.4, 0.6, 0.8, 1.0);
        target.finish().unwrap();
    });
}