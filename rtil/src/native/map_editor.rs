use std::collections::HashMap;
use std::fmt::{Formatter, Pointer};
use std::ops::Deref;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::native::reflection::{GlobalObjectArrayWrapper, ActorWrapper, AActor, UeObjectWrapper};

pub static LEVELS: Mutex<Vec<LevelWrapper>> = Mutex::new(Vec::new());
pub static LIFTS: Lazy<Mutex<HashMap<(usize, usize), LiftWrapper>>> = Lazy::new(||Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct LevelWrapper<'a> {
    base: ActorWrapper<'a>,
}
// WARNING: somewhat unsound - see AMyCharacter
unsafe impl<'a> Send for LevelWrapper<'a> {}
unsafe impl<'a> UeObjectWrapper<'a> for LevelWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_LevelRoot_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> LevelWrapper<'a> {
        LevelWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for LevelWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for LevelWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}

impl<'a> LevelWrapper<'a> {
    pub fn new(level: ActorWrapper<'a>) -> LevelWrapper<'a> {
        assert_eq!(level.class().name(), "BP_LevelRoot_C");
        LevelWrapper { base: level }
    }
    pub fn level_index(&self) -> usize {
        (*self.base.get_field("LevelIndex").unwrap_int()).try_into().unwrap()
    }
    pub fn _source_location(&self) -> (f32, f32, f32) {
        let loc = self.base.get_field("SourcePosition").unwrap_struct();
        (*loc.get_field("X").unwrap_float(), *loc.get_field("Y").unwrap_float(), *loc.get_field("Z").unwrap_float())
    }
    pub fn platforms(&self) -> impl Iterator<Item = PlatformWrapper<'a>> + '_ {
        self.base.get_field("FertileLands").unwrap_array()
            .into_iter()
    }
    pub fn _platform(&self, index: usize) -> Option<PlatformWrapper<'a>> {
        let array = self.base.get_field("FertileLands").unwrap_array();
        array.get(index)
    }
    pub fn cubes(&self) -> impl Iterator<Item = CubeWrapper<'a>> + '_ {
        self.base.get_field("Collectibles").unwrap_array()
            .into_iter()
    }
    pub fn _cube(&self, index: usize) -> Option<CubeWrapper<'a>> {
        let array = self.base.get_field("Collectibles").unwrap_array();
        array.get(index)
    }
    pub fn buttons(&self) -> impl Iterator<Item = ButtonWrapper<'a>> + '_ {
        self.base.get_field("Buttons").unwrap_array()
            .into_iter()
    }
    pub fn _button(&self, index: usize) -> Option<ButtonWrapper<'a>> {
        let array = self.base.get_field("Buttons").unwrap_array();
        array.get(index)
    }
    pub fn _speed(&self) -> f32 {
        *self.base.get_field("Speed").unwrap_float()
    }
    pub fn set_speed(&self, speed: f32) {
        *self.base.get_field("Speed").unwrap_float() = speed
    }
}

#[derive(Debug, Clone)]
pub struct PlatformWrapper<'a> {
    base: ActorWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper<'a> for PlatformWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_IslandChunk_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> PlatformWrapper<'a> {
        PlatformWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for PlatformWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for PlatformWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}
impl<'a> PlatformWrapper<'a> {
    pub fn new(base: ActorWrapper<'a>) -> PlatformWrapper<'a> {
        assert_eq!(base.class().name(), "BP_IslandChunk_C");
        PlatformWrapper { base }
    }
}
#[derive(Debug, Clone)]
pub struct CubeWrapper<'a> {
    base: ActorWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper<'a> for CubeWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_PowerCore_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> CubeWrapper<'a> {
        CubeWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for CubeWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for CubeWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}
impl<'a> CubeWrapper<'a> {
    pub fn new(cube: ActorWrapper<'a>) -> CubeWrapper<'a> {
        assert_eq!(cube.class().name(), "BP_PowerCore_C");
        CubeWrapper { base: cube }
    }
}
#[derive(Debug, Clone)]
pub struct ButtonWrapper<'a> {
    base: ActorWrapper<'a>,
}
unsafe impl<'a> UeObjectWrapper<'a> for ButtonWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_Button_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> ButtonWrapper<'a> {
        ButtonWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for ButtonWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for ButtonWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}
impl<'a> ButtonWrapper<'a> {
    pub fn new(button: ActorWrapper<'a>) -> ButtonWrapper<'a> {
        assert_eq!(button.class().name(), "BP_Button_C");
        ButtonWrapper { base: button }
    }
}
#[derive(Debug, Clone)]
pub struct LiftWrapper<'a> {
    base: ActorWrapper<'a>,
}
// WARNING: somewhat unsound - see AMyCharacter
unsafe impl<'a> Send for LiftWrapper<'a> {}
unsafe impl<'a> UeObjectWrapper<'a> for LiftWrapper<'a> {
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_Lift_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        LiftWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for LiftWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for LiftWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}
impl<'a> LiftWrapper<'a> {
    pub fn new(lift: ActorWrapper<'a>) -> LiftWrapper<'a> {
        assert_eq!(lift.class().name(), "BP_Lift_C");
        LiftWrapper { base: lift }
    }
}

pub fn init() {
    let mut levels = LEVELS.lock().unwrap();
    let mut lifts = LIFTS.lock().unwrap();
    for item in unsafe { GlobalObjectArrayWrapper::get().object_array().iter_elements() } {
        let object = item.object();
        let name = object.name();
        let class_name = object.class().name();
        // fn print_children(depth: usize, class: StructWrapper) {
        //     use crate::native::{PropertyWrapper, UProperty};
        //     use crate::native::reflection::{StructWrapper, ClassWrapper};
        //     for property in class.iter_properties() {
        //         let class_name = property.class().name();
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
        // print_children(1, object.class());

        if class_name == "BP_LevelRoot_C" && name != "Default__BP_LevelRoot_C" {
            let level: LevelWrapper = object.upcast();
            levels.push(level.clone());
        }
        if class_name == "BP_Lift_C" && name != "Default__BP_Lift_C" {
            let lift: LiftWrapper = object.upcast();
            let index: usize = match name.as_str() {
                "BP_Lift_C_1" => 5,
                "BP_Mover7" => 7,
                "BP_Mover6" => 8,
                _ => unreachable!("Invalid lift: {name:?}"),
            };
            lifts.insert((index, 0), lift);
        }
    }
    assert_eq!(levels.len(), 31);
    levels.sort_by_key(|level| level.level_index())
}
