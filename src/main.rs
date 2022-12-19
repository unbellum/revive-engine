// TODO:
//
// 1. Want a separation between a game project and the engine itself
// 2. Setup a workflow for the client project to funnel data into the engine (replacement for the application class we would use in C++)
// 3. Setup a window with the rust equivalent to ImGui
// 4. Add a game loop
// 5. Setup a OpenGL rendering viewport
// 6. Setup input handlers

use rvv_core;

fn main() {
    let app: rvv_core::application::Application = rvv_core::application::Application{name: "Hello".to_string()};
    rvv_core::application::run(app);
}
