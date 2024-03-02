use raylib_ffi::{
    enums::{KeyboardKey, MouseButton},
    *,
};
use setup::BLOCK_SIZE;

mod setup;

fn main() {
    let mut mscroll_brush_size: i32 = 10;
    const SCREEN_WIDTH: i32 = 1440;
    const SCREEN_HEIGHT: i32 = 720;
    setup::setup_stuff(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "Rust Raylib window".to_string(),
        4,
        false,
    );
    const ROWS: u32 = (SCREEN_HEIGHT / BLOCK_SIZE) as u32;
    const COLS: u32 = (SCREEN_WIDTH / BLOCK_SIZE) as u32;

    setup::set_monitor_and_fps(1);
    let mut brush_mode: bool = true;
    let mut grid: Vec<Vec<setup::Cell>> = Vec::with_capacity(ROWS as usize);
    for _ in 0..ROWS {
        let row: Vec<setup::Cell> = vec![setup::Cell::default(); COLS as usize];
        grid.push(row);
    }

    let mut draw_mat = setup::Material::Empty;
    unsafe {
        while !WindowShouldClose() {
            if IsMouseButtonDown(MouseButton::Left as i32) {
                // Left Mouse Button
                let mx = GetMouseX();
                let my = GetMouseY();
                let grid_x = (mx / BLOCK_SIZE) as u32;
                let grid_y = (my / BLOCK_SIZE) as u32;
                if grid_x < COLS && grid_y < ROWS {
                    setup::spawn_sand_brush(
                        &mut grid, mx, my, 50,
                        draw_mat, // Set material to Sand (assuming Sand corresponds to material 1)
                        brush_mode,
                    );
                }
            } else if IsMouseButtonDown(MouseButton::Right as i32) {
                // Left Mouse Button
                let mx = GetMouseX();
                let my = GetMouseY();
                let grid_x = (mx / BLOCK_SIZE) as u32;
                let grid_y = (my / BLOCK_SIZE) as u32;
                if grid_x < COLS && grid_y < ROWS {
                    setup::spawn_sand_brush(
                        &mut grid,
                        mx,
                        my,
                        mscroll_brush_size,
                        setup::Material::Empty, // Set material to Sand (assuming Sand corresponds to material 1)
                        false,
                    );
                }
            }
            if IsKeyPressed(KeyboardKey::One as i32) {
                draw_mat = setup::Material::Sand;
            }
            if IsKeyPressed(KeyboardKey::Two as i32) {
                draw_mat = setup::Material::Water;
            }
            if IsKeyPressed(KeyboardKey::Three as i32) {
                draw_mat = setup::Material::Stone;
            }
            if IsKeyPressed(KeyboardKey::B as i32) {
                brush_mode = !brush_mode;
            }
            mscroll_brush_size += GetMouseWheelMove() as i32 * 3;
            if mscroll_brush_size < 1 {
                mscroll_brush_size = 1;
            }
            println!("{}", mscroll_brush_size);

            BeginDrawing();
            ClearBackground(raylib_ffi::colors::BLACK);
            for i in 0..ROWS {
                for j in 0..COLS {
                    if grid[i as usize][j as usize].material != setup::Material::Empty {
                        DrawRectangle(
                            (j as f32 * BLOCK_SIZE as f32) as i32,
                            (i as f32 * BLOCK_SIZE as f32) as i32,
                            BLOCK_SIZE as i32,
                            BLOCK_SIZE as i32,
                            grid[i as usize][j as usize].color,
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
