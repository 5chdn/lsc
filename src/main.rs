extern crate rand;
extern crate termion;
extern crate termsize;

use std::env;
use std::io::{Write, stdout};

use rand::Rng;
use termion::color;
use termion::raw::IntoRawMode;

fn main() {

  env::set_var("RUST_BACKTRACE", "1");
  let mut stdout = stdout().into_raw_mode().unwrap();

  let cols : u16 = termsize::get().unwrap().cols;
  let rows : u16 = termsize::get().unwrap().rows - 1;

  for y in 0 .. rows {
    for x in 0..cols {
      if y == 0 && x == 0 {
        write!(stdout, "{}╔", color::Fg(color::Yellow)).unwrap();
      } else if y == 0 && x == cols - 1 {
        write!(stdout, "{}╗", color::Fg(color::Yellow)).unwrap();
      } else if y == rows - 1 && x == 0 {
        write!(stdout, "{}╚", color::Fg(color::Yellow)).unwrap();
      } else if y == rows - 1 && x == cols - 1 {
        write!(stdout, "{}╝", color::Fg(color::Yellow)).unwrap();
      } else if x == 0 || x == cols - 1 {
        write!(stdout, "{}║", color::Fg(color::Yellow)).unwrap();
      } else if y == 0 || y == rows - 1 {
        write!(stdout, "{}═", color::Fg(color::Yellow)).unwrap();
      } else {
        let mut rng = rand::thread_rng();
        if rng.gen() {
          write!(stdout, "{}░", color::Fg(color::LightBlue)).unwrap();
        } else {
          if rng.gen() {
            write!(stdout, "{}▒", color::Fg(color::LightYellow)).unwrap();
          } else {
            write!(stdout, "{}▓", color::Fg(color::LightGreen)).unwrap();
          }
        }
      }
    }
  }
}
