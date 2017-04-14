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

  // quit on q key press
  siv.add_global_callback('q', |s| s.quit());

  // add hello-world dialog with quit button
  siv.add_layer(
    Dialog::around(
      TextView::new(
"182. it will be objected that the french and russian revolutions were
     failures. but most revolutions have two goals. one is to destroy an
     old form of society and the other is to set up the new form of society
     envisioned by the revolutionaries. the french and russian
     revolutionaries failed (fortunately!) to create the new kind of
     society of which they dreamed, but they were quite successful in
     destroying the existing form of society.")
    ).title("lsc").button("ok", |s| s.quit())
  );

  // run the cursive event loop
  siv.run();
}
