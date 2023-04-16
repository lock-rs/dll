// Defining Struct
use crate::offsets;
use xorstring::XorString;

pub unsafe fn GetCharacterOffset() -> usize {
    let datamodel = offsets.get_datamodel();
    let players = offsets.find_first_child(datamodel,"Players");
    let localplayer = offsets.get_children(players)[0];
    let localplayername = offsets.get_name(localplayer);

    let workspace = offsets.find_first_child(datamodel,"Workspace");
    let localplayerws = offsets.find_first_child(workspace,&localplayername);

    // Loop through jobs
    let mut off = 0x0 as usize;
    while *crate::cast!(localplayer + off, usize) != localplayerws { 
        off += 4;
    } 

    offsets.roblox_print(format!("Player Character Offset {:x}",off).as_str());

    off
}

pub unsafe fn GetLocalPlayerOffset() -> usize {
    let datamodel = offsets.get_datamodel();
    let players = offsets.find_first_child(datamodel,"Players");
    let localplayer = offsets.get_children(players)[0];

    // Loop through jobs
    let mut off = 0x0 as usize;
    while *crate::cast!(players + off, usize) != localplayer { 
        off += 4;
    } 

    offsets.roblox_print(format!("Players LocalPlayer Offset {:x}",off).as_str());

    off
}

pub unsafe fn GetUserIDOffset() -> usize {
    let datamodel = offsets.get_datamodel();
    let players = offsets.find_first_child(datamodel,"Players");
    let localplayer = offsets.get_children(players)[0];
    
    // Loop through jobs
    let mut off = 0x0 as usize;
    while *crate::cast!(localplayer + off, u32) != 577625010 { 
        off += 4;
    } 

    offsets.roblox_print(format!("Players UserID Offset {:x}",off).as_str());

    off
}


// https://www.roblox.com/games/12109643/Fencing
pub unsafe fn GetPlaceIDOffset() -> usize {
    let datamodel = offsets.get_datamodel();
    
    // Loop through jobs
    let mut off = 0x0 as usize;
    while *crate::cast!(datamodel + off, i32) != 12109643 { 
        off += 4;
    } 

    offsets.roblox_print(format!("Players PlaceID Offset {:x}",off).as_str());

    off
}