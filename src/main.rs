extern crate cursive;

use std::env;
use cursive::Cursive;
use cursive::views::{Dialog, TextView};

/// lsc main entry point
fn main() {

  // always print backtrace on panic
  env::set_var("RUST_BACKTRACE", "1");

  // initialize cursive application root
  let mut siv = Cursive::new();

  // custom tty style sheet
  let _ = siv.load_theme_file("assets/style.toml");

  // add hello-world dialog with quit button
  siv.add_layer(
    Dialog::around(
      TextView::new("hello, world!1")
    ).title("lsc").button("ok", |s| s.quit())
  );

  // run the cursive event loop
  siv.run();
}
