use crate::native::Args;
use crate::native::reflection::{ClassWrapper, FUObjectArray, ObjectWrapper, UObject};
use crate::native::ue::TArray;

#[rtil_derive::hook_before(AMyCharacter::ForcedUnCrouch)]
fn new_game(_args: &mut Args) {

    // debug
    unsafe {
        use std::sync::atomic::Ordering;
        use std::ptr;
        use std::collections::HashMap;
        use crate::native::linux::GUOBJECTARRAY;

        // ::std::thread::sleep(::std::time::Duration::from_secs(10));

        let vtable_names: HashMap<_, _> = dynsym::iter(std::env::current_exe().unwrap()).into_iter()
            .filter(|(name, _addr)| name.starts_with("{vtable("))
            .map(|(name, addr)| (addr + 16, name)).collect();

        let guobject_array = GUOBJECTARRAY.load(Ordering::SeqCst) as *mut FUObjectArray;
        let object_array = ptr::addr_of_mut!((*guobject_array).obj_objects);
        for i in 0..(*object_array).num_elements {
            let item = (*object_array).objects.offset(i.try_into().unwrap());
            let object = ObjectWrapper::new((*item).object as *mut UObject);
            let name = object.name();
            let class_name = object.class().name();
            log!("{} {:?} {:?} ({object:p})", vtable_names[&(object.vtable() as usize)], class_name, name);
            unsafe fn print_children(depth: usize, vtable_names: &HashMap<usize, String>, class: ClassWrapper) {
                if let Some(super_class) = class.super_class() {
                    log!("{}SuperClass: {} {}", "    ".repeat(depth), vtable_names[&(super_class.as_object().vtable() as usize)], class.name());
                    print_children(depth+1, vtable_names, super_class);
                }
                for property in class.iter_properties() {
                    let vtable_name = &vtable_names[&(property.as_object().vtable() as usize)];
                    let class_name = property.as_object().class().name();
                    let name = property.name();
                    let offset = property.offset();
                    log!("{}{} {} {} {:#x} ({property:p})", "    ".repeat(depth), vtable_name, class_name, name, offset);
                    if class_name == "ObjectProperty" {
                        let class = (*(property.as_uobjectproperty())).property_class;
                        log!("{}going into {}", "    ".repeat(depth), (*class).base_ustruct.base_ufield.base_uobject.name.to_string_lossy());
                        // print_children(depth+1, vtable_names, class);
                    }
                }
            }
            // print_children(1, &vtable_names, class);

            if class_name == "BP_LevelRoot_C" {
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


    crate::threads::ue::new_game();
}
