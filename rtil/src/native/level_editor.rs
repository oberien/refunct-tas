use std::sync::Mutex;
use crate::native::reflection::{GlobalObjectArrayWrapper, ActorWrapper, AActor, StructWrapper, UeObjectWrapper, ClassWrapper};

pub static LEVELS: Mutex<Vec<LevelWrapper>> = Mutex::new(Vec::new());

#[derive(Debug, Clone)]
pub struct LevelWrapper<'a> {
    level: ActorWrapper<'a>,
}
// WARNING: somewhat unsound - see AMyCharacter
unsafe impl<'a> Send for LevelWrapper<'a> {}
unsafe impl<'a> UeObjectWrapper for LevelWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_LevelRoot_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        LevelWrapper::new(ActorWrapper::new(ptr))
    }
}

impl<'a> LevelWrapper<'a> {
    pub fn new(level: ActorWrapper<'a>) -> LevelWrapper<'a> {
        assert_eq!(level.as_object().class().name(), "BP_LevelRoot_C");
        LevelWrapper { level }
    }
    pub fn as_actor(&self) -> ActorWrapper<'a> {
        self.level.clone()
    }
    pub fn platforms(&self) -> impl Iterator<Item = PlatformWrapper<'a>> + '_ {
        self.level.as_object().get_field("FertileLands").unwrap_array()
            .into_iter()
    }
    pub fn platform(&self, index: usize) -> PlatformWrapper<'a> {
        let array = self.level.as_object().get_field("FertileLands").unwrap_array();
        array.get(index)
    }
    pub fn speed(&self) -> f32 {
        *self.level.as_object().get_field("Speed").unwrap_float()
    }
    pub fn set_speed(&self, speed: f32) {
        *self.level.as_object().get_field("Speed").unwrap_float() = speed
    }
}

#[derive(Debug, Clone)]
pub struct PlatformWrapper<'a> {
    platform: ActorWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper for PlatformWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_IslandChunk_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        PlatformWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> PlatformWrapper<'a> {
    pub fn new(platform: ActorWrapper<'a>) -> PlatformWrapper<'a> {
        assert_eq!(platform.as_object().class().name(), "BP_IslandChunk_C");
        PlatformWrapper { platform }
    }
    pub fn as_actor(&self) -> ActorWrapper<'a> {
        self.platform.clone()
    }
}

pub fn init() {
    for item in unsafe { GlobalObjectArrayWrapper::get().object_array().iter_elements() } {
        let object = item.object();
        let name = object.name();
        let class_name = object.class().name();
        fn print_children(depth: usize, class: StructWrapper) {
            if let Some(super_class) = class.super_class() {
                log!("{}SuperClass: {}", "    ".repeat(depth), class.name());
                print_children(depth+1, super_class);
            }
            for property in class.iter_properties() {
                let class_name = property.as_object().class().name();
                log!("{}{property}", "    ".repeat(depth));
                if class_name == "ObjectProperty" {
                    let class = unsafe { ClassWrapper::new((*(property.as_uobjectproperty())).property_class) };
                    log!("{}going into {}", "    ".repeat(depth), class.name());
                    // print_children(depth+1, class);
                }
            }
        }
        log!("{:?} {:?} ({object:p})", class_name, name);
        // print_children(1, class);

        if class_name == "BP_LevelRoot_C" && name != "Default__BP_LevelRoot_C" {
            let level: LevelWrapper = object.upcast();
            LEVELS.lock().unwrap().push(level.clone());
            level.set_speed(f32::INFINITY);
            let (lx, ly, lz) = level.as_actor().absolute_location();
            for platform in level.platforms() {
                log!("bloc: {:?}", platform.as_actor().absolute_location());
                platform.as_actor().set_relative_location(0. - lx, 0. - ly, 0. - lz);
                platform.as_actor().set_relative_rotation(0., 0., 0.);
                log!("aloc: {:?}", platform.as_actor().absolute_location());
                log!("arot: {:?}", platform.as_actor().absolute_rotation());
                log!("rrot: {:?}", platform.as_actor().relative_rotation());
            }
        }
    }
}
