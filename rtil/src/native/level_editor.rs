use std::sync::Mutex;
use crate::native::reflection::{ClassWrapper, GlobalObjectArrayWrapper, ObjectWrapper};
use crate::native::ue::TArray;

pub static LEVELS: Mutex<Vec<LevelWrapper>> = Mutex::new(Vec::new());

pub struct LevelWrapper {

}

pub fn init() {
    // debug
    unsafe {
        // ::std::thread::sleep(::std::time::Duration::from_secs(10));

        for item in GlobalObjectArrayWrapper::get().object_array().iter_elements() {
            let object = item.object();
            let name = object.name();
            let class_name = object.class().name();
            log!("{:?} {:?} ({object:p})", class_name, name);
            unsafe fn print_children(depth: usize, class: ClassWrapper) {
                if let Some(super_class) = class.super_class() {
                    log!("{}SuperClass: {}", "    ".repeat(depth), class.name());
                    print_children(depth+1, super_class);
                }
                for property in class.iter_properties() {
                    let class_name = property.as_object().class().name();
                    let name = property.name();
                    let offset = property.offset();
                    log!("{}{} {} {:#x} ({property:p})", "    ".repeat(depth), class_name, name, offset);
                    if class_name == "ObjectProperty" {
                        let class = (*(property.as_uobjectproperty())).property_class;
                        log!("{}going into {}", "    ".repeat(depth), (*class).base_ustruct.base_ufield.base_uobject.name.to_string_lossy());
                        // print_children(depth+1, class);
                    }
                }
            }
            // print_children(1, class);

            if class_name == "BP_LevelRoot_C" && name != "Default__BP_LevelRoot_C" {
                let array = object.get_field("FertileLands").unwrap_array() as *mut TArray<ObjectWrapper>;
                log!("{}/{}", (&*array).len(), (&*array).capacity());
                for e in &*array {
                    let root_component = e.get_field("RootComponent").unwrap_object();
                    let absolute_location = root_component.get_field("AbsoluteLocation").unwrap_struct();
                    let x = absolute_location.get_field("X").unwrap_float();
                    let y = absolute_location.get_field("Y").unwrap_float();
                    let z = absolute_location.get_field("Z").unwrap_float();
                    log!("{} {} {}", *x, *y, *z);
                }
            }
        }
    }
    // end debug

}
