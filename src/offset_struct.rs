use toy_arms::internal::Module;
use crate::cxx::CxxString;

use crate::structs::Vector2;
use crate::structs::Vector3;
use crate::structs::Vector4;

// Rebase adress to roblox
fn rebase_adress(adress: usize) -> usize {
    adress -
        0x00400000 +
        Module::from_module_name("RobloxPlayerBeta.exe").unwrap().module_base_address
}

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
            name_addon: 0x2c,

            // Children
            children_start: 0x30,
            children_end: 0x4,

            // Player
            character: 0x80,

            // Insatnce
            def_start: 0xc,
            class_name: 0x4,

            //Part
            primitive: 0xc4,
            position: 0xfc,

            // Other
            roblox_printaddy: 0x10e1c00,
            task_scheduler: 0xb26840,

            // Viewmatrix
            viewmatrix: 0x170,

            fireprox: 0x146ea30,
        }
    }
}

impl Offsets {
    //== With Base ==//
    pub unsafe fn with_base_adress(&mut self, adress: usize) -> usize {
        Module::from_module_name("RobloxPlayerBeta.exe").unwrap().module_base_address + adress
    }

    //== Get Position ==//
    pub unsafe fn get_position(&mut self, part: usize) -> &Vector3 {
        let primitiv = *crate::cast!(part + self.primitive, usize);
        if *crate::cast!(primitiv, i32) != 0 {
            let position = &*crate::cast!(primitiv + self.position, Vector3);
            return position;
        } else {
            let gay = &(Vector3 { x: 0.0, y: 0.0, z: 0.0 });
            return gay;
        }
    }

    //== Get Jobs ==//
    pub unsafe fn get_jobs(&mut self) -> Vec<usize> {
        let mut jobs = Vec::new();

        let mut current_job = *crate::cast!(self.get_task() + 0x134, usize);

        // Loop through jobs
        while current_job != *crate::cast!((self.get_task() as usize) + 0x138, usize) {
            jobs.push(*(current_job as *const usize));
            current_job += 8;
        }

        jobs // return it
    }

    //== Get Job ==//
    pub unsafe fn get_job(&mut self, to_find: &str) -> usize {
        let mut to_return = 0;
        for i in self.get_jobs() {
            let job_name_location = i + 0x10;
            let defre_job_name_location = job_name_location as *const usize;
            let job_name = crate::CStr
                ::from_ptr(job_name_location as *const crate::c_char)
                .to_str();

            if job_name.is_err() {
                let job_name2 = crate::CStr
                    ::from_ptr(*defre_job_name_location as *const crate::c_char)
                    .to_str();
                if job_name2.unwrap() == to_find {
                    to_return = i;
                    break;
                }
            } else {
                if job_name.unwrap() == to_find {
                    to_return = i;
                    break;
                }
            }
        }
        to_return
    }

    pub unsafe fn getvisualengine(&mut self) -> usize {
        let render = self.get_job("Render");
        let renderview = *((render + 0x148) as *const usize);
        let visualengine = *((renderview + 0x8) as *const usize);

        return visualengine;
    }

    pub unsafe fn getscreendim(&mut self) -> Vector2 {
        let width: f32 = *crate::cast!(self.getvisualengine() + 0x6fc, f32);
        let height: f32 = *crate::cast!(self.getvisualengine() + 0x6fc + 0x4, f32);

        Vector2 {
            x: width,
            y: height,
        }
    }

    pub unsafe fn get_viewmatrix(&mut self) -> Matrix4f32 {
        let position = crate::cast!(self.getvisualengine() + self.viewmatrix, Matrix4f32);

        *position
    }

    pub unsafe fn world2screen(&mut self, position: &Vector3) -> Vector2 {
        let view_matrix = self.get_viewmatrix();

        let clip_coords = Vector4 {
            x: position.x * view_matrix[0] +
            position.y * view_matrix[1] +
            position.z * view_matrix[2] +
            view_matrix[3],
            y: position.x * view_matrix[4] +
            position.y * view_matrix[5] +
            position.z * view_matrix[6] +
            view_matrix[7],
            z: position.x * view_matrix[8] +
            position.y * view_matrix[9] +
            position.z * view_matrix[10] +
            view_matrix[11],
            w: position.x * view_matrix[12] +
            position.y * view_matrix[13] +
            position.z * view_matrix[14] +
            view_matrix[15],
        };

        let screendim = self.getscreendim();

        if clip_coords.w < 0.1 {
            return Vector2 { x: -1.0, y: -1.0 }; // Off screen.
        }

        let cool = Vector3 {
            x: clip_coords.x / clip_coords.w,
            y: clip_coords.y / clip_coords.w,
            z: clip_coords.z / clip_coords.w,
        };

        let out = Vector2 {
            x: (screendim.x / 2.0) * cool.x + (cool.x + screendim.x / 2.0),
            y: -((screendim.y / 2.0) * cool.y) + (cool.y + screendim.y / 2.0),
        };

        if out.x < 0.0 || out.y < 0.0 {
            return Vector2 { x: -1.0, y: -1.0 };
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
        crate::make_fn!(rebase_adress(self.task_scheduler as usize), usize)()
    }

    pub unsafe fn fireproxi(&mut self, instance: usize) {
        crate::make_fn!(rebase_adress(self.fireprox as usize), (), usize, i32)(instance, 0);
    }

    //== Get Parent ==//
    pub unsafe fn get_parent(&mut self, instance: usize) -> usize {
        return *crate::cast!(instance + self.parent_addon, usize);
    }

    pub unsafe fn get_character(&mut self, instance: usize) -> usize {
        return *crate::cast!(instance + self.character, usize);
    }

    pub unsafe fn get_localplayer(&mut self, instance: usize) -> usize {
        *crate::cast!(instance + self.character, usize)
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
        let name_location = *&*crate::cast!(
            *crate::cast!(instance + self.def_start, usize) + self.class_name,
            usize
        ) as *const usize;
        let name = &*crate::cast!(name_location, CxxString);
        name.to_str().unwrap().to_string()
    }

    //== Find First Child ==//
    pub unsafe fn find_first_child(&mut self, instance: usize, child_name: &str) -> usize {
        let mut return_value: usize = 0;

        for i in self.get_children(instance) {
            if self.get_name(i).to_string().as_str() == child_name {
                return_value = i;
            }
        }

        return_value
    }

    //== Find First Child Of Type ==//
    pub unsafe fn find_first_child_of_class(&mut self, instance: usize, child_name: &str) -> usize {
        let return_value: usize = 0;

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

        if instance == (0 as usize) {
            return children;
        }

        let start = *crate::cast!(instance + self.children_start, usize);
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

    //== Get Children ==//
    pub unsafe fn get_every_other_player(&mut self, instance: usize) -> Vec<usize> {
        let mut children = Vec::new();

        if instance == (0 as usize) {
            return children;
        }

        let start = *crate::cast!(instance + self.children_start, usize);
        let end = *crate::cast!(start + self.children_end, usize);

        let mut child = *crate::cast!(start, usize) + 0x8;
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

        let print_func = crate::make_fn!(
            rebase_adress(self.roblox_printaddy as usize),
            (),
            i32,
            crate::CString
        );
        print_func(0, crate::CString::new(var).expect("roblox_print: CString::new Failed!"));
    }
}