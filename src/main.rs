extern crate winreg;

use winreg::enums::*;
use winreg::RegKey;

mod pause;

mod consts;
use consts::*;

mod die;
use die::*;

fn install() {
   let hkcu = RegKey::predef(HKEY_CURRENT_USER);
   let i10n = match hkcu.open_subkey_with_flags(I10N_PATH, KEY_ALL_ACCESS) {
      Ok(key) => { println!(r"Opened HKEY_CURRENT_USER\{}", I10N_PATH); key },
      Err(e) => diep(&[r"Could not open HKEY_CURRENT_USER\{}", I10N_PATH].concat(), &e),
   };

   let am_val: String = match i10n.get_value(AM_KEY) {
      Ok(key) => key,
      Err(e) => die("Could not get the AM key", &e),
   };

   let pm_val: String = match i10n.get_value(PM_KEY) {
      Ok(key) => key,
      Err(e) => die("Could not get the PM key", &e),
   };

   let time_format: String = match i10n.get_value(TIME_KEY) {
      Ok(key) => key,
      Err(e) => die("Could not get the time format", &e),
   };

   // time to store them all
   let (storage_key, what_happened) = match hkcu.create_subkey(STORAGE_PATH) {
      Ok(res) => res,
      Err(e) => die("Could not open the storage key", &e),
   };

   if what_happened == REG_CREATED_NEW_KEY {
      println!("Created {}", STORAGE_PATH);
   }

   match storage_key.set_value(AM_KEY, &am_val) {
      Ok(_) => println!("Saved the original AM key"),
      Err(e) => die("Could not save the ", &e),
   };
   match storage_key.set_value(PM_KEY, &pm_val) {
      Ok(_) => println!("Saved original PM key"),
      Err(e) => die("Could not save the ", &e),
   };
   match storage_key.set_value(TIME_KEY, &time_format) {
      Ok(_) => println!("Saved original time format"),
      Err(e) => die("Could not save the ", &e),
   };

   

   match i10n.set_value(AM_KEY, &"c") {
      Ok(_) => println!("AM is now c"),
      Err(e) => diep("Could not set the AM symbol", &e),
   };

   match i10n.set_value(PM_KEY, &"p") {
      Ok(_) => println!("PM is now p"),
      Err(e) => diep("Could not set the PM symbol", &e),
   };

   match i10n.set_value(TIME_KEY, &"$SPY hmmtt") {
      Ok(_) => println!("Time format set"),
      Err(e) => diep("Could not set the time format", &e),
   }

   println!("
Finished.
You can close this console window.
Be patient and wait for the time to change.
It'll take a minute for it to update on the taskbar.
   ");
   pause::pause();
}

fn main() {
   
}
