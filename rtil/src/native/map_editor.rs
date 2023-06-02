use std::fmt::{Formatter, Pointer};
use std::sync::Mutex;
use crate::native::{PropertyWrapper, UProperty};
use crate::native::reflection::{GlobalObjectArrayWrapper, ActorWrapper, AActor, StructWrapper, UeObjectWrapper, ClassWrapper};

pub static LEVELS: Mutex<Vec<LevelWrapper>> = Mutex::new(Vec::new());

#[derive(Debug, Clone)]
pub struct LevelWrapper<'a> {
    level: ActorWrapper<'a>,
}
// WARNING: somewhat unsound - see AMyCharacter
unsafe impl<'a> Send for LevelWrapper<'a> {}
impl<'a> Pointer for LevelWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.level, f)
    }
}
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
    pub fn level_index(&self) -> usize {
        (*self.level.as_object().get_field("LevelIndex").unwrap_int()).try_into().unwrap()
    }
    pub fn _source_location(&self) -> (f32, f32, f32) {
        let loc = self.level.as_object().get_field("SourcePosition").unwrap_struct();
        (*loc.get_field("X").unwrap_float(), *loc.get_field("Y").unwrap_float(), *loc.get_field("Z").unwrap_float())
    }
    pub fn platforms(&self) -> impl Iterator<Item = PlatformWrapper<'a>> + '_ {
        self.level.as_object().get_field("FertileLands").unwrap_array()
            .into_iter()
    }
    pub fn _platform(&self, index: usize) -> Option<PlatformWrapper<'a>> {
        let array = self.level.as_object().get_field("FertileLands").unwrap_array();
        array.get(index)
    }
    pub fn cubes(&self) -> impl Iterator<Item = CubeWrapper<'a>> + '_ {
        self.level.as_object().get_field("Collectibles").unwrap_array()
            .into_iter()
    }
    pub fn _cube(&self, index: usize) -> Option<CubeWrapper<'a>> {
        let array = self.level.as_object().get_field("Collectibles").unwrap_array();
        array.get(index)
    }
    pub fn buttons(&self) -> impl Iterator<Item = ButtonWrapper<'a>> + '_ {
        self.level.as_object().get_field("Buttons").unwrap_array()
            .into_iter()
    }
    pub fn _button(&self, index: usize) -> Option<ButtonWrapper<'a>> {
        let array = self.level.as_object().get_field("Buttons").unwrap_array();
        array.get(index)
    }
    pub fn _speed(&self) -> f32 {
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
impl<'a> Pointer for PlatformWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.platform, f)
    }
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
#[derive(Debug, Clone)]
pub struct CubeWrapper<'a> {
    cube: ActorWrapper<'a>,
}
impl<'a> Pointer for CubeWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.cube, f)
    }
}
unsafe impl<'a> UeObjectWrapper for CubeWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_PowerCore_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        CubeWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> CubeWrapper<'a> {
    pub fn new(cube: ActorWrapper<'a>) -> CubeWrapper<'a> {
        assert_eq!(cube.as_object().class().name(), "BP_PowerCore_C");
        CubeWrapper { cube }
    }
    pub fn as_actor(&self) -> ActorWrapper<'a> {
        self.cube.clone()
    }
}
#[derive(Debug, Clone)]
pub struct ButtonWrapper<'a> {
    button: ActorWrapper<'a>,
}
impl<'a> Pointer for ButtonWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.button, f)
    }
}
unsafe impl<'a> UeObjectWrapper for ButtonWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_Button_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        ButtonWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> ButtonWrapper<'a> {
    pub fn new(button: ActorWrapper<'a>) -> ButtonWrapper<'a> {
        assert_eq!(button.as_object().class().name(), "BP_Button_C");
        ButtonWrapper { button }
    }
    pub fn as_actor(&self) -> ActorWrapper<'a> {
        self.button.clone()
    }
}

pub fn init() {
    let mut levels = LEVELS.lock().unwrap();
    for item in unsafe { GlobalObjectArrayWrapper::get().object_array().iter_elements() } {
        let object = item.object();
        let name = object.name();
        let class_name = object.class().name();
        // fn print_children(depth: usize, class: StructWrapper) {
        //     for property in class.iter_properties() {
        //         let class_name = property.as_object().class().name();
        //         log!("{}{property}", "    ".repeat(depth));
        //         if class_name == "ObjectProperty" {
        //             let class = unsafe { ClassWrapper::new((*(property.as_uobjectproperty())).property_class) };
        //             log!("{}going into {}", "    ".repeat(depth), class.name());
        //             // print_children(depth+1, class);
        //         }
        //     }
        //     log!("{}done printing children", "    ".repeat(depth));
        // }
        // log!("{:?} {:?} ({object:p})", class_name, name);
        // print_children(1, object.class().as_struct());

        if class_name == "BP_LevelRoot_C" && name != "Default__BP_LevelRoot_C" {
            let level: LevelWrapper = object.upcast();
            levels.push(level.clone());
        }
    }
    assert_eq!(levels.len(), 31);
    levels.sort_by_key(|level| level.level_index())
}
