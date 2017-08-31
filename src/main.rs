extern crate sdl2;

use std::process;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {

  let windowWidth = 640;
  let windowHeight = 480;

  let ctx = sdl2::init().unwrap();
  let video_ctx = ctx.video().unwrap();

  let window  = match video_ctx
    .window("rust_to_js", windowWidth, windowHeight)
    .position_centered()
    .opengl()
    .build() {
      Ok(window) => window,
      Err(err)   => panic!("failed to create window: {}", err)
    };

  let mut renderer = match window
    .renderer()
    .build() {
      Ok(renderer) => renderer,
      Err(err) => panic!("failed to create renderer: {}", err)
    };

  let rect2Startx = windowWidth - 20;
  let rect2Starty = windowHeight - 20;

  let mut rect = Rect::new(10, 10, 10, 10);
  let mut rect2 = Rect::new(rect2Startx as i32, rect2Starty as i32, 10, 10);
  let mut bullets = Vec::new();

  let white = sdl2::pixels::Color::RGB(255, 255, 255);
  let black = sdl2::pixels::Color::RGB(0, 0, 0);
  let blue = sdl2::pixels::Color::RGB(0, 0, 255);
  let red = sdl2::pixels::Color::RGB(255, 0, 0);

  let mut p1Color = blue;
  let mut p2Color = red;

  let mut events = ctx.event_pump().unwrap();

  let mut main_loop = || {
    for event in events.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
          process::exit(1);
        },

        // Player 1
        Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
          rect.x -= 10;
        },
        Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
          rect.x += 10;
        },
        Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
          rect.y -= 10;
        },
        Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
          rect.y += 10;
        },
        Event::KeyDown { keycode: Some(Keycode::Slash), ..} => {
          bullets.push(Rect::new(rect.x, rect.y, 5, 5));
        },

        // Player 2
        Event::KeyDown { keycode: Some(Keycode::A), ..} => {
          rect2.x -= 10;
        },
        Event::KeyDown { keycode: Some(Keycode::D), ..} => {
          rect2.x += 10;
        },
        Event::KeyDown { keycode: Some(Keycode::W), ..} => {
          rect2.y -= 10;
        },
        Event::KeyDown { keycode: Some(Keycode::S), ..} => {
          rect2.y += 10;
        },
        Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
          bullets.push(Rect::new(rect2.x, rect2.y, 5, 5));
        },
        _ => {}
      }
    }

    let _ = renderer.set_draw_color(black);
    let _ = renderer.clear();
    let _ = renderer.set_draw_color(p1Color);
    let _ = renderer.fill_rect(rect);
    let _ = renderer.set_draw_color(p2Color);
    let _ = renderer.fill_rect(rect2);
    let _ = renderer.set_draw_color(white);
    for bullet in &bullets {
      let _ = renderer.fill_rect(*bullet);
    }
    let _ = renderer.present();
  };

  #[cfg(target_os = "emscripten")]
  use emscripten::{emscripten};

  #[cfg(target_os = "emscripten")]
  emscripten::set_main_loop_callback(main_loop);

  #[cfg(not(target_os = "emscripten"))]
  loop { main_loop(); }
}
