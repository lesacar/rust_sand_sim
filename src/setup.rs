// setup.rs
use rand::Rng;
use raylib_ffi::*;

pub const BLOCK_SIZE: i32 = 4; // Example block size

#[derive(Clone, Copy, PartialEq)] // Add PartialEq here
pub enum Material {
    Empty,
    Sand,
    Water,
    Stone,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub friction: f32,
    pub is_free_falling: bool,
    pub mass: i32,
    pub material: Material, // Assuming Material enum is public
    pub spread_factor: i32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub color: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            friction: 0.0,
            is_free_falling: false,
            mass: 0,
            material: Material::Empty,
            spread_factor: 0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            color: NO_COLOR,
        }
    }
}

const NO_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub fn spawn_sand_brush(
    grid: &mut Vec<Vec<Cell>>,
    mouse_x: i32,
    mouse_y: i32,
    brush_size: i32,
    material: Material,
    brush_mode: bool,
) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let brush_radius = brush_size / 2;

    let start_x = (mouse_x / BLOCK_SIZE) - brush_radius;
    let start_y = (mouse_y / BLOCK_SIZE) - brush_radius;

    let start_x = start_x.max(0);
    let start_y = start_y.max(0);

    let end_x = (start_x + brush_size).min(cols);
    let end_y = (start_y + brush_size).min(rows);

    for i in start_x..end_x {
        for j in start_y..end_y {
            let distance = (((i - start_x).pow(2) + (j - start_y).pow(2)) as f32).sqrt();

            if distance <= brush_radius as f32 {
                if let Some(cell) = grid.get_mut(i as usize) {
                    if let Some(cell) = cell.get_mut(j as usize) {
                        cell.material = material;
                        match material {
                            Material::Sand => {
                                cell.friction = 0.95;
                                cell.color = Color {
                                    r: 201,
                                    g: 170,
                                    b: 127,
                                    a: 255,
                                };
                            }
                            Material::Water => {
                                cell.friction = 0.0;
                                cell.spread_factor = 5;
                                cell.color = Color {
                                    r: 0,
                                    g: 0,
                                    b: 255,
                                    a: 255,
                                };
                            }
                            Material::Stone => {
                                cell.friction = 0.0;
                                cell.spread_factor = 0;
                                cell.color = Color {
                                    r: 51,
                                    g: 83,
                                    b: 69,
                                    a: 255,
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn setup_stuff(
    sc_wi: i32,
    sc_he: i32,
    window_title: String,
    log_lvl: i32,
    fullscreen: bool,
) -> i32 {
    println!("Test from setup.rs");
    unsafe {
        raylib_ffi::SetTraceLogLevel(log_lvl);
        raylib_ffi::InitWindow(sc_wi, sc_he, rl_str!(window_title.to_string()));
    }
    if fullscreen {
        unsafe {
            ToggleFullscreen();
        }
    }
    0
}

pub fn set_monitor_and_fps(monitor: i32) -> i32 {
    {
        unsafe {
            raylib_ffi::SetWindowMonitor(monitor - 1);
            SetTargetFPS(GetMonitorRefreshRate(monitor - 1));
        }
    }
    0
}
