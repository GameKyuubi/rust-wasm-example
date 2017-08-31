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
  let mut bullets :Vec<Rect> = Vec::new();

  let white = sdl2::pixels::Color::RGB(255, 255, 255);
  let black = sdl2::pixels::Color::RGB(0, 0, 0);
  let blue = sdl2::pixels::Color::RGB(0, 0, 255);
  let red = sdl2::pixels::Color::RGB(255, 0, 0);

  let mut keyState = HashMap::new();

  let mut p1Color = blue;
  let mut p2Color = red;

  let mut p1Speed = 0.5;
  let mut p2Speed = 0.5;

  let mut p1Pos: [f32; 2] = [10.0, 10.0];
  let mut p2Pos: [f32; 2] = [rect2Startx as f32, rect2Starty as f32];

  let mut events = ctx.event_pump().unwrap();

  let mut main_loop = || {
    for event in events.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
          process::exit(1);
        },

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

        // // Player 1
        // Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
        //   rect.x -= 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
        //   rect.x += 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
        //   rect.y -= 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
        //   rect.y += 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::Slash), ..} => {
        //   bullets.push(Rect::new(rect.x, rect.y, 5, 5));
        // },

        // // Player 2
        // Event::KeyDown { keycode: Some(Keycode::A), ..} => {
        //   rect2.x -= 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::D), ..} => {
        //   rect2.x += 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::W), ..} => {
        //   rect2.y -= 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::S), ..} => {
        //   rect2.y += 10;
        // },
        // Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
        //   bullets.push(Rect::new(rect2.x, rect2.y, 5, 5));
        // },
        _ => {}
      }
    }

    if Some(&true) == keyState.get(&Keycode::Left) {
      p1Pos[0] -= p1Speed;
    }
    if Some(&true) == keyState.get(&Keycode::Right) {
      p1Pos[0] += p1Speed;
    }

    p1Rect.x = p1Pos[0] as i32;
    p1Rect.y = p1Pos[1] as i32;

    let _ = renderer.set_draw_color(black);
    let _ = renderer.clear();
    let _ = renderer.set_draw_color(p1Color);
    let _ = renderer.fill_rect(p1Rect);
    let _ = renderer.set_draw_color(p2Color);
    let _ = renderer.fill_rect(p2Rect);
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
