use super::pause::pause;
use std::process::exit;

use super::consts::*;

pub fn c_die(msg: &str, err: &dyn std::fmt::Debug) -> ! {
   eprintln!("{}{}!\nerr: {:?}{}", ERROR_MSG_HEADER, msg, err, ERROR_MSG_FOOTER);
   pause();
   exit(1);
}

pub fn die(msg: &str, err: &dyn std::fmt::Debug) -> ! {
   eprintln!("{}{}!\nerr: {:?}{}\nNo changes to your system have been made.", ERROR_MSG_HEADER, msg, err, ERROR_MSG_FOOTER);
   pause();
   exit(1);
}

pub trait DieAble<T> {
   fn die_or(self, what: &str) -> T;
   fn c_die_or(self, what: &str) -> T;
}

pub trait Dead {
   fn cannot(&self, what: &str) -> !;
   fn c_cannot(&self, what: &str) -> !;
}
