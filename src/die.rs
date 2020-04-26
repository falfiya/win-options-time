pub fn die(msg: &str, err: &dyn std::fmt::Debug) -> ! {
   eprintln!("win-options-time error:\nmessage: {}!\nerr: {:?}", msg, err);
   super::pause::pause();
   std::process::exit(1);
}
