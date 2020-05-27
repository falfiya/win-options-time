use std::io;
use io::prelude::*;

pub fn pause() {
   let mut stdout = io::stdout();
   match write!(stdout, "Press any key to continue...").or(stdout.flush()) {
      Err(_) => std::process::exit(5),
      _ => (),
   }

   super::getch::getch();
}
