use std::io;
use io::prelude::*;

extern crate getch;
use getch::Getch;

pub fn pause() {
   let mut stdout = io::stdout();
   match write!(stdout, "Press any key to continue...").or(stdout.flush()) {
      Err(_) => std::process::exit(5),
      _ => (),
   }

   Getch::new().getch().expect("win-options-time::pause internal error: could not getch.");
}
