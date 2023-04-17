#![feature(cstr_from_bytes_until_nul)]
#![feature(const_trait_impl)]

//== Crates ==//
extern crate winconsole;
extern crate toy_arms;
extern crate winapi;
extern crate chiter;
extern crate libc;
extern crate cxx;

//== Mod ==//
mod structs;
mod offsets;
mod errorhandling;

//== Use ==//
use windows;
use rlua::{ Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic };
use std::{ ffi::{ CString, CStr, c_char }, fmt::format };
use std::{ thread, time, str::Chars };
use toy_arms::internal::Module;
use shroud::directx::directx11;
use std::ffi::c_void;
use chiter::make_fn;
use toy_arms::cast;
use std::env;

use structs::Vector2;
use structs::Vector3;
use offsets::Offsets;

/* unsafe fn setfpslimit(limit: i32) {
  let fps = cast!(mut gettask() + 0x118,f64);
  *fps = 1f64 / limit as f64;
} */

pub type uintptr_t = usize;
pub static mut offsets: Offsets = Offsets::default();

pub struct Addresses {
    datamodel: usize,
    players: usize,
    localplayer: usize,
}

pub static mut ADDRESSES: Addresses = Addresses { datamodel: 0, players: 0, localplayer: 0 };

mod gay;
mod bruteforce;
mod ui;

fn main(_hinst: usize) {
    // println!("{}",xorstr!("Cool?55"));

    env::set_var("RUST_BACKTRACE", "full");
    unsafe {
        // if is_debug_mode() {
            winapi::um::consoleapi::AllocConsole();
        // }
    }

    unsafe {
        errorhandling::init_errorhandler();
    }

    unsafe {
        //let temp = crate::Module::from_module_name("RobloxPlayerBeta.exe").unwrap().module_base_address;
        //let mut offsets = Offsets::default();

/*         let datamodel = offsets.get_datamodel();
        let players = offsets.find_first_child(datamodel, "Players");
        let localplayer = offsets.get_localplayer(players);

        ADDRESSES = Addresses {
            datamodel: datamodel,
            players: players,
            localplayer: localplayer,
        };

        println!("{:x}", offsets.get_task());

        let cool = offsets.getscreendim();
        println!("{} {}", cool.x, cool.y);

        let v1 = Vector2 { x: 1.0, y: 2.0 };
        let v2 = Vector2 { x: 4.0, y: 5.0 };
        let distance = v1.distance(&v2);
        println!("{:.2}", distance); */

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