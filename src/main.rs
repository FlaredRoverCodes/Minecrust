use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const TARGET_FPS: u32 = 60;

fn main()
{
    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Minecrust")
        .vsync()
        .build();

    raylib_handle.set_target_fps(TARGET_FPS);

    while !raylib_handle.window_should_close()
    {
        let mut draw_handle = raylib_handle.begin_drawing(&raylib_thread);
        draw_handle.clear_background(Color::BLACK);
        draw_handle.draw_text("Welcome to Minecrust", 10, 10, 20, Color::WHITE);
    }
}