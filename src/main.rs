use std::env;

/// lsc main entry point
fn main() {

  // always print backtrace on panic
  env::set_var("RUST_BACKTRACE", "1");
}
