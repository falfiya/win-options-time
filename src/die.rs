use super::pause::pause;
use std::process::exit;

pub fn diep(msg: &str, err: &dyn std::fmt::Debug) -> ! {
   eprintln!("win-options-time error: {}!\nerr: {:?}", msg, err);
   pause();
   exit(1);
}

pub fn die(msg: &str, err: &dyn std::fmt::Debug) -> ! {
   eprint!("win-options-time error: {}!\nerr: {:?}\nNo changes to your system have been made. Exiting...", msg, err);
   exit(1);
}

pub trait DieAble {
   fn die(&self, why: &str) -> !;
   fn diep(&self, why: &str) -> !;
}

impl DieAble for std::io::Error {
   fn die(&self, why: &str) -> ! {
      die(why, self);
   }

   fn diep(&self, why: &str) -> ! {
      diep(why, self);
   }
}
