use std::ffi::{CString};
use std::slice;
use toy_arms::internal::Module;
use chiter::make_fn;
use nalgebra::{Matrix4};

fn addy(adress: usize) -> usize {
    adress - 0x00400000 + Module::from_module_name("RobloxPlayerBeta.exe").unwrap().module_base_address 
}

//== Vector3 Struct ==//
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Vector4 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}

pub type vecf32 = Vec<f32>;
pub type Matrix4f32 = [f32; 16];
//== Offsets Struct ==//
pub struct Offsets {
    //Wait for scripts
        waitscriptdata: usize,

    // Addons
    parent_addon: usize,
    name_addon: usize,

    // Children
    children_start: usize,
    children_end: usize,

    // Player
    character: usize,

    // Insatnce
    def_start: usize,
    class_name: usize,

    //Part
    primitive: usize,
    position: usize,

    // Other
    roblox_printaddy: usize,
    task_scheduler: usize,

    // Viewmatrix
    viewmatrix: usize,


    fireprox: usize,
    
}


//== Defaults ==//
impl const Default for Offsets {
    fn default() -> Self {
        Self {
            //Wait for scripts
            waitscriptdata: 0x28,

            // Addons
            parent_addon: 0x38,
            name_addon: 0x2C,

            // Children
            children_start: 0x30,
            children_end: 0x4,

            // Player
            character: 0x80,

            // Insatnce
            def_start: 0xC,
            class_name: 0x4,

            //Part
            primitive: 0xC4,
            position: 0xFC,

            // Other
            roblox_printaddy: 0x10E1C00,
            task_scheduler: 0xB26840,

            // Viewmatrix
            viewmatrix: 0x170,

            fireprox: 0x146EA30,
        }
    }
}


use crate::cxx::CxxString;
use crate::cxx::CxxVector;
//== Implement Funcs ==//



impl Offsets {

    //== With Base ==//
    pub unsafe fn with_base_adress(&mut self, adress: usize) -> usize {
        Module::from_module_name("RobloxPlayerBeta.exe").unwrap().module_base_address + adress
    } 

    //== Get Position ==//
    pub unsafe fn get_position(&mut self, part: usize) -> &Vector3 {
        let primitiv = *crate::cast!(part + self.primitive, usize); 
        if *crate::cast!(primitiv, i32) != 0 {
            let position = &*crate::cast!(primitiv + self.position,Vector3);
            return position;
        } else {
            let gay = &Vector3 {x: 0.0,y: 0.0, z: 0.0};
            return gay;
        }
    }

    //== Get Jobs ==//
    pub unsafe fn get_jobs(&mut self) -> Vec<usize> {

        let mut jobs = Vec::new();
    
        let mut current_job = *crate::cast!(self.get_task() + 0x134 , usize); 
        
        // Loop through jobs
        while current_job != *crate::cast!(self.get_task() as usize + 0x138, usize) { 
            
            jobs.push(*(current_job as *const usize));
            current_job += 8;
        } 
    
        jobs // return it
    }

    //== Get Job ==//
    pub unsafe fn get_job(&mut self,to_find: &str) -> usize {
        let mut to_return = 0;
        for i in self.get_jobs() {
            let job_name_location = i + 0x10;
            let defre_job_name_location = job_name_location as *const usize;
            let job_name = crate::CStr::from_ptr(job_name_location as *const crate::c_char).to_str();
            
            if job_name.is_err() {
                let job_name2 = crate::CStr::from_ptr(*defre_job_name_location as *const crate::c_char).to_str();
                if job_name2.unwrap() == to_find {
                    to_return = i;
                    break;
                }
            }
            else {
                if job_name.unwrap() == to_find {
                    to_return = i;
                    break;
                }
            }
         } 
        to_return
    }

    pub unsafe fn getvisualengine(&mut self) -> usize {
        let Render = self.get_job("Render");
        let renderview = *((Render + 0x148) as *const usize);
        let visualengine = *((renderview + 0x8) as *const usize);

        return visualengine;
    }

    pub unsafe fn getscreendim(&mut self) -> Vector2 {
        let width: f32 = *crate::cast!(self.getvisualengine() + 0x6FC, f32);
        let height : f32 = *crate::cast!(self.getvisualengine() + 0x6FC + 0x4, f32);

        Vector2 {
            x:width,
            y:height
        }
    }

    pub unsafe fn get_viewmatrix(&mut self) -> Matrix4f32 {

        let position = crate::cast!(self.getvisualengine() + self.viewmatrix, Matrix4f32);

        *position
    }

    pub unsafe fn world2screen(&mut self,position: &Vector3) -> Vector2 {

        let viewMatrix = self.get_viewmatrix();

        let mut clipCoords = Vector4 {
            X:(position.x * viewMatrix[0]) + (position.y * viewMatrix[1]) + (position.z * viewMatrix[2]) + viewMatrix[3],
            Y:(position.x * viewMatrix[4]) + (position.y * viewMatrix[5]) + (position.z * viewMatrix[6]) + viewMatrix[7],
            Z:(position.x * viewMatrix[8]) + (position.y * viewMatrix[9]) + (position.z * viewMatrix[10]) + viewMatrix[11],
            W:(position.x * viewMatrix[12]) + (position.y * viewMatrix[13]) + (position.z * viewMatrix[14]) + viewMatrix[15]
        };

        let screendim = self.getscreendim();

        if (clipCoords.W < 0.1) {
            return Vector2 { x:-1.0, y:-1.0 }; // Off screen.
        }
       
        let mut cool = Vector3 {
            x:clipCoords.X / clipCoords.W,
            y:clipCoords.Y / clipCoords.W,
            z:clipCoords.Z / clipCoords.W
        };

        let out = Vector2 {
            x: (screendim.x / 2.0 * cool.x) + (cool.x + screendim.x / 2.0),
            y: -(screendim.y / 2.0 * cool.y) + (cool.y + screendim.y / 2.0)
        };

        if (out.x < 0.0) || (out.y < 0.0) {
            return Vector2 {x:-1.0,y:-1.0};
        }

        return out;
    }

    //== Get Datamodel //
    pub unsafe fn get_datamodel(&mut self) -> usize {
        let waitinghybridscriptsjob = self.get_job("WaitingHybridScriptsJob");
        let datamodel = waitinghybridscriptsjob + self.waitscriptdata;

        *(crate::cast!(datamodel, usize) as *const usize) + 0x4
    }

    //== Get Task ==//
    pub unsafe fn get_task(&mut self) -> usize {
        crate::make_fn!(addy(self.task_scheduler as usize), usize)()
    }

    pub unsafe fn fireproxi(&mut self, instance: usize) {
        crate::make_fn!(addy(self.fireprox as usize), (),usize,i32)(instance,0);
    }

    //== Get Parent ==//
    pub unsafe fn get_parent(&mut self, instance: usize) -> usize {
        return *crate::cast!(instance + self.parent_addon, usize);
    }

    pub unsafe fn get_character(&mut self, instance: usize) -> usize {
        let ply = String::from("Player");
        match self.get_classname(instance) {
            ply => return *crate::cast!(instance + self.character, usize),
            _ => return 0,
        }
    }

    pub unsafe fn get_localplayer(&mut self, instance: usize) -> usize {
        let plys = String::from("Players");

        match self.get_classname(instance) {
            plys => return *crate::cast!(instance + self.character, usize),
            _ => return 0,
        }
    }

    //== Get Name ==//
    pub unsafe fn get_name(&mut self, instance: usize) -> String {
        let name_location = instance + self.name_addon;
        let defre_name_location = *(name_location as *const usize) as *const usize;
        let name = &*crate::cast!(defre_name_location, CxxString);
        name.to_str().unwrap().to_string()
    }

    //== Get Class Name ==//
    pub unsafe fn get_classname(&mut self, instance: usize) -> String {
        let name_location = *(&*crate::cast!(*crate::cast!(instance + self.def_start,usize) + self.class_name, usize)) as *const usize;
        let name = &*crate::cast!(name_location, CxxString);
        name.to_str().unwrap().to_string()
    }

    //== Find First Child ==//
    pub unsafe fn  find_first_child(&mut self, instance: usize, child_name: &str) -> usize {
        let mut return_value: usize = 0;

        for i in self.get_children(instance) {
            if self.get_name(i).to_string().as_str() == child_name {
                return_value = i;
            }
        }

        return_value
    }

    //== Find First Child Of Type ==//
    pub unsafe fn  find_first_child_of_class(&mut self, instance: usize, child_name: &str) -> usize {
        let mut return_value: usize = 0;

        for i in self.get_children(instance) {
            if self.get_name(i).to_string().as_str() == child_name {
                return i;
            }
        }

        return_value
    }

    //== Get Children ==//
    pub unsafe fn get_children(&mut self, instance: usize) -> Vec<usize> {

        let mut children = Vec::new();

        if instance == 0 as usize {return children;}

        let start  = *crate::cast!(instance + self.children_start, usize);
        let end = *crate::cast!(start + self.children_end, usize);

        let mut child = *crate::cast!(start, usize); 
        // Loop through children
        while child < end { 
            if !(self.get_name(*crate::cast!(child, usize)).chars().count() == 0) {
                children.push(*crate::cast!(child, usize)); 
            }

            child += 0x8; 
        } 
        
        children
    }

    //== Roblox Print ==//
    pub unsafe fn roblox_print(&mut self, text: &str) {
        let var = text.replace(|c: char| !c.is_ascii(), "");

        let print_func = crate::make_fn!(addy(self.roblox_printaddy as usize), (), i32, crate::CString);
        print_func(0, crate::CString::new(var).expect("roblox_print: CString::new Failed!"));
    }
}
