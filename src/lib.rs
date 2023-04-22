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


pub static mut OFFSETS: Offsets = Offsets::default();
#[derive(Debug)]
pub struct Addresses {
    datamodel: usize,
    players: usize,
    localplayer: usize,
    visualengine: usize
}

pub static mut ADDRESSES: Addresses = Addresses { datamodel: 0, players: 0, localplayer: 0, visualengine: 0};
use crate::structs::rbxfunctions;
mod dx11_hook;
mod bruteforce;
mod ui;

fn main(_hinst: usize) {
    error_handling::init_errorhandler();

    env::set_var("RUST_BACKTRACE", "full");
    unsafe {
            winapi::um::consoleapi::AllocConsole();
        let datamodel = OFFSETS.get_datamodel();
        let players = OFFSETS.find_first_child(datamodel, "Players");
        let localplayer = OFFSETS.get_localplayer(players);
        let visualengine = OFFSETS.getvisualengine();

        ADDRESSES = Addresses {
          datamodel: datamodel,
          players: players,
          localplayer: localplayer,
          visualengine: visualengine,
        };
        let wwwww = OFFSETS.find_first_child(datamodel,"Workspace");

        let hum = OFFSETS.find_first_child(wwwww,"Camera");
        
        println!("{:X} {:X} {:X} {:X}",ADDRESSES.datamodel,ADDRESSES.players,ADDRESSES.localplayer,hum);

        let coooool = OFFSETS.get_functions(hum);


        type test = Box<u32>;
        let struct_ptr: test = Box::new(0 as u32);

        let coool = OFFSETS.get_function(hum,"WorldToViewportPoint");
        let name = coool.GetName();
        println!("{}",name);
        let func = coool.GetFunc();
        println!("{:X}",func);
        

        //destroy(hum,&struct_ptr,2.0,2.0,2.0);
        
        
/*         for i in coooool {
            let name = i.GetName();
            let nametmp = name.as_str();

            match nametmp {
                "Destroy" => {
                    println!("{}",nametmp);
                    let func = i.GetFunc();
                    println!("{:X}",func)
                    //make_fn!(func,(),usize)(hum);
                },
                _ => {}
            }
        }        
         */
    }
    
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