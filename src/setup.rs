use raylib_ffi::*;

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
