extern crate sdl2;
extern crate num;

use num::clamp;
use std::collections::HashMap;
use std::process;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {

  let window_width = 640;
  let window_height = 480;

  let ctx = sdl2::init().unwrap();
  let video_ctx = ctx.video().unwrap();

  let window  = match video_ctx
    .window("rust_to_js", window_width, window_height)
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

  let rect2_start_x = window_width - 20;
  let rect2_start_y = window_height - 20;

  let mut p1_rect = Rect::new(10, 10, 10, 10);
  let mut p2_rect = Rect::new(rect2_start_x as i32, rect2_start_y as i32, 10, 10);
  let mut p1_bullets :Vec<Bullet> = Vec::new();
  let mut p2_bullets :Vec<Bullet> = Vec::new();

  let black = sdl2::pixels::Color::RGB(0, 0, 0);
  let light_blue = sdl2::pixels::Color::RGB(120, 120, 255);
  let light_red = sdl2::pixels::Color::RGB(255, 120, 120);

  let mut key_state = HashMap::new();

  let p1_speed = 3.0;
  let p2_speed = 3.0;

  struct Point {
    x: f32,
    y: f32,
  }

  struct Bullet {
    position: Point,
    speed: f32
  }

  let mut p1_pos: Point = Point { x: 10f32, y: 10f32 };
  let mut p2_pos: Point = Point { x: rect2_start_x as f32, y: rect2_start_y as f32 };

  let default_charge: f32 = 3.0;

  let mut p1_charge: f32 = default_charge; // when extended, should default to 0
  let mut p2_charge: f32 = default_charge;

  let charge_rate: f32 = 0.1;

  let mut events = ctx.event_pump().unwrap();

  let mut main_loop = || {
    for event in events.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
          process::exit(1);
        },

        // { // KEYDOWN
          // Player 1
          Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
            key_state.insert(Keycode::Left, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
            key_state.insert(Keycode::Right, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
            key_state.insert(Keycode::Up, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
            key_state.insert(Keycode::Down, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Slash), ..} => {
            key_state.insert(Keycode::Slash, true);
          },

          // Player 2
          Event::KeyDown { keycode: Some(Keycode::F), ..} => {
            key_state.insert(Keycode::F, true);
          },
          Event::KeyDown { keycode: Some(Keycode::H), ..} => {
            key_state.insert(Keycode::H, true);
          },
          Event::KeyDown { keycode: Some(Keycode::T), ..} => {
            key_state.insert(Keycode::T, true);
          },
          Event::KeyDown { keycode: Some(Keycode::G), ..} => {
            key_state.insert(Keycode::G, true);
          },
          Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
            key_state.insert(Keycode::Z, true);
          },
        // }

        // { // KEYUP
          // Player 1
          Event::KeyUp { keycode: Some(Keycode::Left), ..} => {
            key_state.insert(Keycode::Left, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Right), ..} => {
            key_state.insert(Keycode::Right, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Up), ..} => {
            key_state.insert(Keycode::Up, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Down), ..} => {
            key_state.insert(Keycode::Down, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Slash), ..} => {
            key_state.insert(Keycode::Slash, false);
            p1_bullets.push(Bullet {
              position: Point {
                x: p1_pos.x,
                y: p1_pos.y,
              },
              speed: p1_charge,
            });
            p1_charge = default_charge;
          },

          // Player 2
          Event::KeyUp { keycode: Some(Keycode::F), ..} => {
            key_state.insert(Keycode::F, false);
          },
          Event::KeyUp { keycode: Some(Keycode::H), ..} => {
            key_state.insert(Keycode::H, false);
          },
          Event::KeyUp { keycode: Some(Keycode::T), ..} => {
            key_state.insert(Keycode::T, false);
          },
          Event::KeyUp { keycode: Some(Keycode::G), ..} => {
            key_state.insert(Keycode::G, false);
          },
          Event::KeyUp { keycode: Some(Keycode::Z), ..} => {
            key_state.insert(Keycode::Z, false);
            p2_bullets.push(Bullet {
              position: Point {
                x: p2_pos.x,
                y: p2_pos.y,
              },
              speed: p2_charge,
            });
            p2_charge = default_charge;
          },
        // }
        _ => {}
      }
    }

    // Update
    {
      { // Player 1
        match key_state.get(&Keycode::Left) {
          Some(&true) => p1_pos.x -= p1_speed,
          _ => {}
        }
        match key_state.get(&Keycode::Right) {
          Some(&true) => p1_pos.x += p1_speed,
          _ => {}
        }
        match key_state.get(&Keycode::Up) {
          Some(&true) => p1_pos.y -= p1_speed,
          _ => {}
        }
        match key_state.get(&Keycode::Down) {
          Some(&true) => p1_pos.y += p1_speed,
          _ => {}
        }
        match key_state.get(&Keycode::Slash) {
          Some(&true) => p1_charge += charge_rate,
          _ => {}
        }
      }

      { // Player 2
        match key_state.get(&Keycode::F) {
          Some(&true) => p2_pos.x -= p2_speed,
          _ => {}
        }
        match key_state.get(&Keycode::H) {
          Some(&true) => p2_pos.x += p2_speed,
          _ => {}
        }
        match key_state.get(&Keycode::T) {
          Some(&true) => p2_pos.y -= p2_speed,
          _ => {}
        }
        match key_state.get(&Keycode::G) {
          Some(&true) => p2_pos.y += p2_speed,
          _ => {}
        }
        match key_state.get(&Keycode::Z) {
          Some(&true) => p2_charge += charge_rate,
          _ => {}
        }
      }

      p1_rect.x = p1_pos.x as i32;
      p1_rect.y = p1_pos.y as i32;

      p2_rect.x = p2_pos.x as i32;
      p2_rect.y = p2_pos.y as i32;

      for mut bullet in p1_bullets.iter_mut() {
        bullet.position.x += bullet.speed;
      }
      for mut bullet in p2_bullets.iter_mut() {
        bullet.position.x -= bullet.speed;
      }
    }

    for bullet in &p2_bullets {
      let rect = Rect::new(bullet.position.x as i32, bullet.position.y as i32, 5, 5);
      if p1_rect.has_intersection(rect) {
        println!("P1 hit!");
      }
    }
    for bullet in &p1_bullets {
      let rect = Rect::new(bullet.position.x as i32, bullet.position.y as i32, 5, 5);
      if p2_rect.has_intersection(rect) {
        println!("P2 hit!");
      }
    }

    let scale = 15;
    let p1_other_color = 50+(clamp(p1_charge, 0f32, 12f32) as u8)*scale;
    let p2_other_color = 50+(clamp(p2_charge, 0f32, 12f32) as u8)*scale;

    let p1_color = sdl2::pixels::Color::RGB(p1_other_color, p1_other_color, 255);
    let p2_color = sdl2::pixels::Color::RGB(255, p2_other_color, p2_other_color);

    { // Draw
      let _ = renderer.set_draw_color(black);
      let _ = renderer.clear();
      let _ = renderer.set_draw_color(p1_color);
      let _ = renderer.fill_rect(p1_rect);
      let _ = renderer.set_draw_color(p2_color);
      let _ = renderer.fill_rect(p2_rect);
      let _ = renderer.set_draw_color(light_blue);
      for bullet in &p1_bullets {
        let _ = renderer.fill_rect(Rect::new(bullet.position.x as i32, bullet.position.y as i32, 5, 5));
      }
      let _ = renderer.set_draw_color(light_red);
      for bullet in &p2_bullets {
        let _ = renderer.fill_rect(Rect::new(bullet.position.x as i32, bullet.position.y as i32, 5, 5));
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
