extern crate sdl2;

use std::collections::HashMap;
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

  let mut p1Rect = Rect::new(10, 10, 10, 10);
  let mut p2Rect = Rect::new(rect2Startx as i32, rect2Starty as i32, 10, 10);
  let mut p1Bullets :Vec<Rect> = Vec::new();
  let mut p2Bullets :Vec<Rect> = Vec::new();

  let white = sdl2::pixels::Color::RGB(255, 255, 255);
  let black = sdl2::pixels::Color::RGB(0, 0, 0);
  let blue = sdl2::pixels::Color::RGB(0, 0, 255);
  let lightBlue = sdl2::pixels::Color::RGB(120, 120, 255);
  let red = sdl2::pixels::Color::RGB(255, 0, 0);
  let lightRed = sdl2::pixels::Color::RGB(255, 120, 120);

  let mut keyState = HashMap::new();

  let mut p1Color = blue;
  let mut p2Color = red;

  let mut p1Speed = 3.0;
  let mut p2Speed = 3.0;

  struct Point {
    x: f32,
    y: f32,
  }

  let mut p1Pos: Point = Point { x: 10f32, y: 10f32 };
  let mut p2Pos: Point = Point { x: rect2Startx as f32, y: rect2Starty as f32 };

  let mut p1Charge: f32 = 1.0; // when extended, should default to 0
  let mut p2Charge: f32 = 1.0;

  let chargeRate: f32 = 0.01;
  let maxCharge: f32 = 100.0;

  let mut events = ctx.event_pump().unwrap();

  let mut main_loop = || {
    for event in events.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
          process::exit(1);
        },

        { // KEYDOWN
          // Player 1
          Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
            keyState.insert(Keycode::Left, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
            keyState.insert(Keycode::Right, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
            keyState.insert(Keycode::Up, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
            keyState.insert(Keycode::Down, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Slash), ..} => {
            keyState.insert(Keycode::Slash, true);
          },

          // Player 2
          Event::KeyDown { keycode: Some(Keycode::A), ..} => {
            keyState.insert(Keycode::A, true);
          },
          Event::KeyDown { keycode: Some(Keycode::D), ..} => {
            keyState.insert(Keycode::D, true);
          },
          Event::KeyDown { keycode: Some(Keycode::W), ..} => {
            keyState.insert(Keycode::W, true);
          },
          Event::KeyDown { keycode: Some(Keycode::S), ..} => {
            keyState.insert(Keycode::S, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
            keyState.insert(Keycode::Z, true);
          },
        }

        { // KEYUP
          // Player 1
          Event::KeyUp { keycode: Some(Keycode::Left), ..} => {
            keyState.insert(Keycode::Left, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Right), ..} => {
            keyState.insert(Keycode::Right, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Up), ..} => {
            keyState.insert(Keycode::Up, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Down), ..} => {
            keyState.insert(Keycode::Down, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Slash), ..} => {
            keyState.insert(Keycode::Slash, false);
          },

          // Player 2
          Event::KeyUp { keycode: Some(Keycode::A), ..} => {
            keyState.insert(Keycode::A, false);
          },
          Event::KeyUp { keycode: Some(Keycode::D), ..} => {
            keyState.insert(Keycode::D, false);
          },
          Event::KeyUp { keycode: Some(Keycode::W), ..} => {
            keyState.insert(Keycode::W, false);
          },
          Event::KeyUp { keycode: Some(Keycode::S), ..} => {
            keyState.insert(Keycode::S, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Z), ..} => {
            keyState.insert(Keycode::Z, false);
          },
        }
        _ => {}
      }
    }

    // Update
    {
      { // Player 1
        match keyState.get(&Keycode::Left) {
          Some(&true) => p1Pos.x -= p1Speed,
          _ => {}
        }
        match keyState.get(&Keycode::Right) {
          Some(&true) => p1Pos.x += p1Speed,
          _ => {}
        }
        match keyState.get(&Keycode::Up) {
          Some(&true) => p1Pos.y -= p1Speed,
          _ => {}
        }
        match keyState.get(&Keycode::Down) {
          Some(&true) => p1Pos.y += p1Speed,
          _ => {}
        }
        match keyState.get(&Keycode::Slash) {
          Some(&true) => p1Bullets.push(Rect::new(p1Pos.x as i32, p1Pos.y as i32, 5, 5)),
          _ => {}
        }
      }

      { // Player 2
        match keyState.get(&Keycode::A) {
          Some(&true) => p2Pos.x -= p2Speed,
          _ => {}
        }
        match keyState.get(&Keycode::D) {
          Some(&true) => p2Pos.x += p2Speed,
          _ => {}
        }
        match keyState.get(&Keycode::W) {
          Some(&true) => p2Pos.y -= p2Speed,
          _ => {}
        }
        match keyState.get(&Keycode::S) {
          Some(&true) => p2Pos.y += p2Speed,
          _ => {}
        }
        match keyState.get(&Keycode::Z) {
          Some(&true) => p2Bullets.push(Rect::new(p2Pos.x as i32, p2Pos.y as i32, 5, 5)),
          _ => {}
        }
      }

      p1Rect.x = p1Pos.x as i32;
      p1Rect.y = p1Pos.y as i32;

      p2Rect.x = p2Pos.x as i32;
      p2Rect.y = p2Pos.y as i32;

      for mut bullet in p1Bullets.iter_mut() {
        bullet.x += p1Charge as i32;
      }
      for mut bullet in p2Bullets.iter_mut() {
        bullet.x -= p2Charge as i32;
      }
    }

    { // Draw
      let _ = renderer.set_draw_color(black);
      let _ = renderer.clear();
      let _ = renderer.set_draw_color(p1Color);
      let _ = renderer.fill_rect(p1Rect);
      let _ = renderer.set_draw_color(p2Color);
      let _ = renderer.fill_rect(p2Rect);
      let _ = renderer.set_draw_color(lightBlue);
      for bullet in &p1Bullets {
        let _ = renderer.fill_rect(*bullet);
      }
      let _ = renderer.set_draw_color(lightRed);
      for bullet in &p2Bullets {
        let _ = renderer.fill_rect(*bullet);
      }
      let _ = renderer.present();
    }
  };

  #[cfg(target_os = "emscripten")]
  use emscripten::{emscripten};

  #[cfg(target_os = "emscripten")]
  emscripten::set_main_loop_callback(main_loop);

  #[cfg(not(target_os = "emscripten"))]
  loop { main_loop(); }
}
