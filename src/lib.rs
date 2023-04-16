#![feature(cstr_from_bytes_until_nul)]
#![feature(const_trait_impl)]
use std::env;
extern crate winapi;
extern crate chiter;
extern crate winconsole;
extern crate cxx;
extern crate libc;
use std::{ffi::{CString, CStr, c_char}, fmt::format};
use shroud::directx::directx11;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};
extern crate toy_arms;
use chiter::make_fn;

use toy_arms::cast;
use std::{thread, time, str::Chars};
pub type uintptr_t = usize;

use toy_arms::internal::Module;

// Defining Struct
mod offsets;
use offsets::Offsets;

use offsets::Vector2;
use offsets::Vector3;
mod errorhandling;
/* unsafe fn setfpslimit(limit: i32) {
  let fps = cast!(mut gettask() + 0x118,f64);
  *fps = 1f64 / limit as f64;
} */

use std::ffi::c_void;

pub static mut offsets: Offsets = Offsets::default();

pub struct Addresses {
  datamodel: usize,
  players: usize,
  localplayer: usize,
}


pub static mut ADDRESSES: Addresses = Addresses {datamodel:0,players:0,localplayer:0};

use xorstring;
use xorstring::xorstring_procmacro;

macro_rules! xorstr {
  ($str_lit:literal) => {
    $crate::XorString::new(xorstring_procmacro::xorstring!($str_lit).0).decrypt()
  };
  ($str_var:expr) => {
    $crate::XorString::new(xorstring_procmacro::xorstring!($str_var).0).decrypt()
  };
}

mod gay;
mod bruteforce;
mod ui;


fn main(_hinst: usize) {
   unsafe { errorhandling::init_errorhandler(); }
  // println!("{}",xorstr!("Cool?55"));

  env::set_var("RUST_BACKTRACE", "full");
  unsafe { winapi::um::consoleapi::AllocConsole();  }

  unsafe {
      //let temp = crate::Module::from_module_name("RobloxPlayerBeta.exe").unwrap().module_base_address;
      //let mut offsets = Offsets::default();
    
      let datamodel = offsets.get_datamodel();
      let players = offsets.find_first_child(datamodel, "Players");
      let localplayer = offsets.get_localplayer(players);

      ADDRESSES = Addresses {
        datamodel: datamodel,
        players: players,
        localplayer: localplayer,
      };

       println!("{:x}",offsets.get_task());


       let cool = offsets.getscreendim();
       println!("{} {}",cool.x,cool.y);

/*       let cool = offsets.world2screen(Vector3 { x: 1.1, y: 2.2, z: 3.3 });

       println!("{} {}",cool.x,cool.y);
        */
      // offsets.roblox_print("Test");
      
/*        let datamodel = offsets.get_datamodel();

      let render = offsets.get_job("Render");
      let viewmatrix = offsets.get_viewmatrix();
      println!("{:?}",viewmatrix);
      offsets.roblox_print(format!("Render: {:x}",render).as_str());
      // println!("Datamodel: {:x} ",datamodel);
      // offsets.roblox_print(format!("Datamodel: {:x} ",datamodel).as_str());
      let Workspace = offsets.find_first_child(datamodel,"Workspace");

      let Cool = offsets.find_first_child(Workspace, "Cool");
      let Prox = offsets.find_first_child(Cool, "ProximityPrompt");

      offsets.fireproxi(Prox);
      // offsets.roblox_print(format!("Workspace {:x}",datamodel).as_str());
      for i in offsets.get_children(Cool) {
        let instancename = offsets.get_name(i);
        let parent = offsets.get_parent(i);
        let parentname = offsets.get_name(parent);
        let classname = offsets.get_classname(i);
        offsets.roblox_print(format!("Name: {} | Parent: {} | ClassName: {} | address: {:x}",instancename,parentname,classname,i).as_str());
      }
       */



/*       bruteforce::GetCharacterOffset();
      bruteforce::GetLocalPlayerOffset();
      bruteforce::GetUserIDOffset(); */
        // bruteforce::GetPlaceIDOffset(); 
       gay::main_thread(_hinst);

    }
}


#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32, _reserved: *mut ()) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || unsafe { main(hinst) });
    }

    1
}

/*
supg's house

        | | 
   /-------\    
  /         \ 
 /           \
/ print("hi") \
|             |
|    _____    |
|    |   |    |
|    |   |    |
|    |   |    |
===============

*/
