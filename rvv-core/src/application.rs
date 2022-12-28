
use crate::{scene::Scene};
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

    // Setup egui?
    //let mut egui_glium = egui_glium::EguiGlium::new(&display, &event_loop);

    // TODO: Before moving on need to learn:
    //
    // 1. Pattern matching
    // 2. Closures
    // 3. How egui_glium works
    // 4. How to use egui directly once egui_glium is setup
    //
    // Post learning:
    //
    // 1. Setup egui (Panel for the scene, panel for component editing, viewport window)
    // 2. Render a dummy scene (rotating cube + ground grid?)
    //      - Default world object that implements a cube
    //      - Scene camera setup to view cube
    //      - Dumb shaders with no lighting
    // 3. Add a scene module, consider if ecs is what we want to use out of the box (ehh.... Hardcoded object types + vectors might be better short term)

    let mut scene: Scene = Scene::new();
    scene.load_basic_scene(&display);

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