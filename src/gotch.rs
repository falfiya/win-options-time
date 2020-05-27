extern crate getch;
use getch::Getch;

pub fn getch() -> char {
   char::from(
      Getch::new()
         .getch()
         .expect("win-options-time::pause internal error: could not getch.")
   )
}
