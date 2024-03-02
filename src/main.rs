use raylib_ffi::*;
use setup::BLOCK_SIZE;

mod setup;

fn main() {
    const SCREEN_WIDTH: i32 = 1440;
    const SCREEN_HEIGHT: i32 = 720;
    setup::setup_stuff(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "Rust Raylib window".to_string(),
        4,
        false,
    );
    const ROWS: usize = (SCREEN_HEIGHT / BLOCK_SIZE) as usize;
    const COLS: usize = (SCREEN_WIDTH / BLOCK_SIZE) as usize;

    setup::set_monitor_and_fps(1);
    let brush_mode: bool = true;
    let mut grid: Vec<Vec<setup::Cell>> = vec![vec![setup::Cell::default(); ROWS]; COLS];
    unsafe {
        while !WindowShouldClose() {
            if IsMouseButtonDown(0) {
                // Left Mouse Button
                let mx = GetMouseX();
                let my = GetMouseY();
                let grid_x = (mx / BLOCK_SIZE) as usize;
                let grid_y = (my / BLOCK_SIZE) as usize;
                if grid_x < COLS && grid_y < ROWS {
                    setup::spawn_sand_brush(
                        &mut grid,
                        mx,
                        my,
                        10,
                        setup::Material::Sand, // Set material to Sand (assuming Sand corresponds to material 1)
                        brush_mode,
                    );
                }
            }

            BeginDrawing();
            ClearBackground(raylib_ffi::colors::BLACK);
            for i in 0..COLS {
                for j in 0..ROWS {
                    if grid[i][j].material != setup::Material::Empty {
                        let color = grid[i][j].color;
                        DrawRectangle(
                            (i as i32 * BLOCK_SIZE) as i32,
                            (j as i32 * BLOCK_SIZE) as i32,
                            BLOCK_SIZE as i32,
                            BLOCK_SIZE as i32,
                            color.into(),
                        );
                    }
                }
            }

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
