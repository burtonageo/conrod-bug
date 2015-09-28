#![feature(plugin)]

// Actually using the following lints causes an ICE. Hopefully, it should
// go away in a future rustc version, so keep them around as a reminder.
//#![plugin(clippy, sorty)]

extern crate apply;
extern crate ai_behavior as ai;
extern crate conrod;
extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate nalgebra as na;
extern crate opengl_graphics;
extern crate piston;
extern crate rebind;
extern crate sprite;
extern crate viewport;

mod game;
mod screens;

use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::window::WindowSettings;

use game::Game;

fn main() {
    const OPENGL: OpenGL = OpenGL::V3_2;
    let window = WindowSettings::new("cargo-bug", (800, 600))
        .exit_on_esc(true)
        .fullscreen(false)
        .opengl(OPENGL)
        .vsync(true)
        .build::<GlutinWindow>()
        .expect("Could not create window");

    Game::new(OPENGL, window).run_loop();
}