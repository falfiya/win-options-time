extern crate winreg;

use winreg::enums::*;
use winreg::RegKey;

mod pause;

mod consts;
mod die;
use die::die;

fn main() {
   let hkcu = RegKey::predef(HKEY_CURRENT_USER);
   let i10n = match hkcu.open_subkey_with_flags(consts::I10N_PATH, KEY_ALL_ACCESS) {
      Ok(key) => { eprintln!(r"Opened HKEY_CURRENT_USER\{}", consts::I10N_PATH); key },
      Err(e) => die(&[consts::I10N_PATH, " could not be opened"].concat(), &e),
   };

   match i10n.set_value(consts::AM_KEY, &"c") {
      Ok(_) => eprintln!("AM is now c."),
      Err(e) => die("Could not set the AM symbol", &e),
   };

   match i10n.set_value(consts::PM_KEY, &"p") {
      Ok(_) => eprintln!("PM is now p."),
      Err(e) => die("Could not set the PM symbol", &e),
   };

   match i10n.set_value(consts::TIME_KEY, &"$SPY hmmtt") {
      Ok(_) => eprintln!("Time format set."),
      Err(e) => die("Could not set the time format", &e),
   }

   println!(
"Finished.
You can close this console window.
Be patient and wait for the time to change.
It'll take a minute for it to update on the taskbar."
   );
   pause::pause();
}
