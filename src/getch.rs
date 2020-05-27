extern crate getch;
use getch::Getch;

let getcher = Getch::new();
pub fn getch() {
   getcher.getch().expect("win-options-time::pause internal error: could not getch.");
}
