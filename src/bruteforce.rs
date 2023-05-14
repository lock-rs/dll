// Defining Struct
use crate::OFFSETS;


pub unsafe fn get_character_offset() -> usize {
    let datamodel = OFFSETS.get_datamodel();
    let players = OFFSETS.find_first_child(datamodel,"Players");
    let localplayer = OFFSETS.get_children(players)[0];
    let localplayername = OFFSETS.get_name(localplayer);

    let workspace = OFFSETS.find_first_child(datamodel,"Workspace");
    let localplayerws = OFFSETS.find_first_child(workspace,&localplayername);

    // Loop through jobs
    let mut off = 0x0 as usize;
    while *crate::cast!(localplayer + off, usize) != localplayerws { 
        off += 4;
    } 

    OFFSETS.roblox_print(format!("Player Character Offset {:x}",off).as_str());

    off
}

pub unsafe fn get_localplayer_offset() -> usize {
    let datamodel = OFFSETS.get_datamodel();
    let players = OFFSETS.find_first_child(datamodel,"Players");
    let localplayer = OFFSETS.get_children(players)[0];

    // Loop through jobs
    let mut off = 0x0 as usize;
    while *crate::cast!(players + off, usize) != localplayer { 
        off += 4;
    } 

    OFFSETS.roblox_print(format!("Players LocalPlayer Offset {:x}",off).as_str());

    off
}