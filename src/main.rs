use raylib_ffi::*;

mod setup;
fn main() {
    setup::setup_stuff(1440, 720, "Rust Raylib window".to_string(), 4, false);
    setup::set_monitor_and_fps(1);
    unsafe {
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(raylib_ffi::colors::BLACK);
            DrawText(
                TextFormat(rl_str!(GetFPS())),
                20,
                20,
                20,
                raylib_ffi::colors::WHITE,
            );
            EndDrawing();
        }
    }
}
