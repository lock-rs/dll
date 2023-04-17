#![feature(const_trait_impl)]

//== Crates ==//
extern crate cxx;

//== Mod ==//
mod structs;
mod offset_struct;
mod error_handling;
mod vars;
//== Use ==//
use std::{ ffi::{ CString, CStr, c_char } };
use chiter::make_fn;
use toy_arms::cast;
use std::env;

use offset_struct::Offsets;

/* unsafe fn setfpslimit(limit: i32) {
  let fps = cast!(mut gettask() + 0x118,f64);
  *fps = 1f64 / limit as f64;
} */

pub static mut OFFSETS: Offsets = Offsets::default();

pub struct Addresses {
    datamodel: usize,
    players: usize,
    localplayer: usize,
}

pub static mut ADDRESSES: Addresses = Addresses { datamodel: 0, players: 0, localplayer: 0 };

mod dx11_hook;
mod bruteforce;
mod ui;

fn main(_hinst: usize) {


    env::set_var("RUST_BACKTRACE", "full");
    unsafe {
            winapi::um::consoleapi::AllocConsole();
        let datamodel = OFFSETS.get_datamodel();
        let players = OFFSETS.find_first_child(datamodel, "Players");
        let localplayer = OFFSETS.get_localplayer(players);

        ADDRESSES = Addresses {
          datamodel: datamodel,
          players: players,
          localplayer: localplayer,
        };
    }

    error_handling::init_errorhandler();
    
    dx11_hook::main_thread(_hinst);
}

#[no_mangle]
extern "stdcall" fn DllMain(hinst: usize, reason: u32, _reserved: *mut ()) -> i32 {
    if reason == 1 {
        std::thread::spawn(move || main(hinst) );
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