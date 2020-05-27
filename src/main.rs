extern crate winreg;

use winreg::enums::*;
use winreg::RegKey;

mod pause;

mod consts;
use consts::*;

mod die;
use die::*;

mod gotch;

use gotch::getch;

use std::io;

impl <T> DieAble<T> for io::Result<T> {
   fn die_or(self, what: &str) -> T {
      match self {
         Ok(r) => r,
         Err(e) => die(&["Could not ", what].concat(), &e),
      }
   }

   fn c_die_or(self, what: &str) -> T {
      match self {
         Ok(r) => r,
         Err(e) => c_die(&["Could not ", what].concat(), &e),
      }
   }
}

impl Dead for io::Error {
   fn cannot(&self, what: &str) -> ! {
      die(&["Could not ", what].concat(), self);
   }

   fn c_cannot(&self, what: &str) -> ! {
      c_die(&["Could not ", what].concat(), self);
   }
}

#[inline(always)]
fn install_orchestrator(storage: RegKey, i10n: RegKey) {
   println!("Installing...");
   // --- PREINSTALL ---
   let am_val: String = i10n.get_value(AM_KEY).die_or("get the AM key");

   let pm_val: String = i10n.get_value(PM_KEY).die_or("get the PM key");

   let time_format: String = i10n.get_value(TIME_KEY).die_or("get the time format");

   match storage.set_value(AM_KEY, &am_val) {
      Ok(_) => println!("Saved the original AM key"),
      Err(e) => die("Could not save the ", &e),
   };

   match storage.set_value(PM_KEY, &pm_val) {
      Ok(_) => println!("Saved original PM key"),
      Err(e) => e.cannot("save the original PM key"),
   };

   match storage.set_value(TIME_KEY, &time_format) {
      Ok(_) => println!("Saved original time format"),
      Err(e) => e.cannot("save the original time format"),
   };
   // --- INSTALL ---
   match i10n.set_value(AM_KEY, &"c") {
      Ok(_) => println!("{} is now c", am_val),
      Err(e) => e.c_cannot("set the AM symbol"),
   };

   match i10n.set_value(PM_KEY, &"p") {
      Ok(_) => println!("{} is now p", pm_val),
      Err(e) => e.c_cannot("set the PM symbol"),
   };

   match i10n.set_value(TIME_KEY, &"$SPY hmmtt") {
      Ok(_) => println!("Time format set"),
      Err(e) => e.c_cannot("set the time format"),
   }
   // --- POST INSTALL ---
   match storage.set_value(INSTALLED_KEY, &1u32) {
      Err(_) => eprintln!("Unable to record installation.\nNothing is broken but uninstallation might not work"),
      _ => (),
   };

   println!("
Finished installing.
It will take a literal minute for everything to update.
You can delete win-options-time.exe if you want.
   ");
}

#[inline(always)]
fn uninstall(hkcu: RegKey, storage: RegKey, i10n: RegKey) {
   println!("Uninstalling...");
   let ori_am_val: String = storage.get_value(AM_KEY).die_or("get the original AM key");

   let ori_pm_val: String = storage.get_value(PM_KEY).die_or("get the original PM key");

   let ori_time_format: String = storage.get_value(TIME_KEY).die_or("get the original time format");

   match i10n.set_value(AM_KEY, &ori_am_val) {
      Ok(_) => println!("c is now {}", ori_am_val),
      Err(e) => e.c_cannot("set the AM symbol"),
   };

   match i10n.set_value(PM_KEY, &ori_pm_val) {
      Ok(_) => println!("p is now {}", ori_pm_val),
      Err(e) => e.c_cannot("set the PM symbol"),
   };

   match i10n.set_value(TIME_KEY, &ori_time_format) {
      Ok(_) => println!("Time format restored"),
      Err(e) => e.c_cannot("set the time format"),
   }
   std::mem::drop(storage);
   std::mem::drop(i10n);
   match hkcu.delete_subkey_all(STORAGE_PATH) {
      Ok(_) => println!("Deleted own registry entries"),
      Err(e) => e.c_cannot("completely uninstall"),
   };
}

fn main() {
   let hkcu = RegKey::predef(HKEY_CURRENT_USER);

   let (storage, what_happened) =
   hkcu.create_subkey(STORAGE_PATH).die_or("open the storage key");

   if what_happened == REG_CREATED_NEW_KEY {
      println!("Created {}", STORAGE_PATH);
   } else {
      println!("Opened {}", STORAGE_PATH);
   }

   let installed = storage.get_value(INSTALLED_KEY).unwrap_or(0u32) == 1;

   let i10n = match hkcu.open_subkey_with_flags(I10N_PATH, KEY_ALL_ACCESS) {
      Ok(key) => { println!(r"Opened HKEY_CURRENT_USER\{}", I10N_PATH); key },
      Err(e) => die(&[r"Could not open HKEY_CURRENT_USER\", I10N_PATH].concat(), &e),
   };

   print!("win-options-time:\n   Press `i` to install or `u` to uninstall\n");
   let c = char::from(getch());
   if c == 'i' {
      if installed {
         eprintln!("Already installed!");
      } else {
         install_orchestrator(storage, i10n);
      }
   } else if c == 'u' {
      if installed {
         uninstall(hkcu, storage, i10n);
      } else {
         eprintln!("win-option-time is not installed.");
      }
   } else {
      eprintln!("Unknown command '{}'", c);
   }
   pause::pause();
}
