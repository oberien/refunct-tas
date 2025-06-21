use std::cell::Cell;
use std::fmt::{Formatter, Pointer};
use std::ops::Deref;
use std::sync::Mutex;
use crate::native::{ArrayWrapper, ObjectIndex, StructValueWrapper, UeObjectWrapperType, UeScope, UObject, ObjectWrapper};
use crate::native::reflection::{AActor, ActorWrapper, UeObjectWrapper};

pub static LEVELS: Mutex<Vec<Level>> = Mutex::new(Vec::new());

pub struct Level {
    pub level: ObjectIndex<LevelWrapperType>,
    pub platforms: Vec<ObjectIndex<PlatformWrapperType>>,
    pub cubes: Vec<ObjectIndex<CubeWrapperType>>,
    pub buttons: Vec<ObjectIndex<ButtonWrapperType>>,
    pub lifts: Vec<ObjectIndex<LiftWrapperType>>,
    pub pipes: Vec<ObjectIndex<PipeWrapperType>>,
    pub springpads: Vec<ObjectIndex<SpringpadWrapperType>>,
}

#[derive(Debug, Clone)]
pub struct LevelWrapper<'a> {
    base: ActorWrapper<'a>,
}
pub enum LevelWrapperType {}
impl UeObjectWrapperType for LevelWrapperType {
    type UeObjectWrapper<'a> = LevelWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for LevelWrapper<'a> {
    type UeObjectWrapperType = LevelWrapperType;
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
        self.base.get_field("LevelIndex").unwrap::<i32>().try_into().unwrap()
    }
    pub fn source_location(&self) -> (f32, f32, f32) {
        let loc = self.base.get_field("SourcePosition").unwrap::<StructValueWrapper>();
        (loc.get_field("X").unwrap(), loc.get_field("Y").unwrap(), loc.get_field("Z").unwrap())
    }
    pub fn set_source_location(&self, x: f32, y: f32, z: f32) {
        let loc = self.base.get_field("SourcePosition").unwrap::<StructValueWrapper>();
        loc.get_field("X").unwrap::<&Cell<f32>>().set(x);
        loc.get_field("Y").unwrap::<&Cell<f32>>().set(y);
        loc.get_field("Z").unwrap::<&Cell<f32>>().set(z);
    }
    pub fn platforms(&self) -> impl Iterator<Item = PlatformWrapper<'a>> + '_ {
        self.base.get_field("FertileLands").unwrap::<ArrayWrapper<'_, _>>()
            .into_iter()
    }
    pub fn _platform(&self, index: usize) -> Option<PlatformWrapper<'a>> {
        let array = self.base.get_field("FertileLands").unwrap::<ArrayWrapper<'_, _>>();
        array.get(index)
    }
    pub fn cubes(&self) -> impl Iterator<Item = CubeWrapper<'a>> + '_ {
        self.base.get_field("Collectibles").unwrap::<ArrayWrapper<'_, _>>()
            .into_iter()
    }
    pub fn _cube(&self, index: usize) -> Option<CubeWrapper<'a>> {
        let array = self.base.get_field("Collectibles").unwrap::<ArrayWrapper<'_, _>>();
        array.get(index)
    }
    pub fn buttons(&self) -> impl Iterator<Item = ButtonWrapper<'a>> + '_ {
        self.base.get_field("Buttons").unwrap::<ArrayWrapper<'_, _>>()
            .into_iter()
    }
    pub fn _button(&self, index: usize) -> Option<ButtonWrapper<'a>> {
        let array = self.base.get_field("Buttons").unwrap::<ArrayWrapper<'_, _>>();
        array.get(index)
    }
    pub fn speed(&self) -> f32 {
        self.base.get_field("Speed").unwrap()
    }
    pub fn set_speed(&self, speed: f32) {
        self.base.get_field("Speed").unwrap::<&Cell<f32>>().set(speed)
    }
}

#[derive(Debug, Clone)]
pub struct PlatformWrapper<'a> {
    base: ActorWrapper<'a>,
}
pub enum PlatformWrapperType {}
impl UeObjectWrapperType for PlatformWrapperType {
    type UeObjectWrapper<'a> = PlatformWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for PlatformWrapper<'a> {
    type UeObjectWrapperType = PlatformWrapperType;
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
pub enum CubeWrapperType {}
impl UeObjectWrapperType for CubeWrapperType {
    type UeObjectWrapper<'a> = CubeWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for CubeWrapper<'a> {
    type UeObjectWrapperType = CubeWrapperType;
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
pub enum ButtonWrapperType {}
impl UeObjectWrapperType for ButtonWrapperType {
    type UeObjectWrapper<'a> = ButtonWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for ButtonWrapper<'a> {
    type UeObjectWrapperType = ButtonWrapperType;
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
pub enum LiftWrapperType {}
impl UeObjectWrapperType for LiftWrapperType {
    type UeObjectWrapper<'a> = LiftWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for LiftWrapper<'a> {
    type UeObjectWrapperType = LiftWrapperType;
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
#[derive(Debug, Clone)]
pub struct PipeWrapper<'a> {
    base: ActorWrapper<'a>,
}
pub enum PipeWrapperType {}
impl UeObjectWrapperType for PipeWrapperType {
    type UeObjectWrapper<'a> = PipeWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for PipeWrapper<'a> {
    type UeObjectWrapperType = PipeWrapperType;
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_TravelPipe_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        PipeWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for PipeWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for PipeWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}
impl<'a> PipeWrapper<'a> {
    pub fn new(pipe: ActorWrapper<'a>) -> PipeWrapper<'a> {
        assert_eq!(pipe.class().name(), "BP_TravelPipe_C");
        PipeWrapper { base: pipe }
    }
}

#[derive(Debug, Clone)]
pub struct SpringpadWrapper<'a> {
    base: ActorWrapper<'a>,
}
pub enum SpringpadWrapperType {}
impl UeObjectWrapperType for SpringpadWrapperType {
    type UeObjectWrapper<'a> = SpringpadWrapper<'a>;
}
unsafe impl<'a> UeObjectWrapper<'a> for SpringpadWrapper<'a> {
    type UeObjectWrapperType = SpringpadWrapperType;
    type Wrapping = AActor;
    const CLASS_NAME: &'static str = "BP_Jumppad_C";

    unsafe fn create(ptr: *mut Self::Wrapping) -> Self {
        SpringpadWrapper::new(ActorWrapper::new(ptr))
    }
}
impl<'a> Deref for SpringpadWrapper<'a> {
    type Target = ActorWrapper<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<'a> Pointer for SpringpadWrapper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.base, f)
    }
}
impl<'a> SpringpadWrapper<'a> {
    pub fn new(springpad: ActorWrapper<'a>) -> SpringpadWrapper<'a> {
        assert_eq!(springpad.class().name(), "BP_Jumppad_C");
        SpringpadWrapper { base: springpad }
    }
}

#[derive(Debug, Copy, Clone, rebo::ExternalType)]
pub enum ElementType {
    Platform,
    Cube,
    Button,
    Lift,
    Pipe,
    Springpad,
}

#[derive(Debug, Clone, Copy, rebo::ExternalType)]
pub struct ElementIndex {
    pub cluster_index: usize,
    pub element_type: ElementType,
    pub element_index: usize,
}


pub fn try_find_element_index(ptr: *mut UObject) -> Option<ElementIndex> {
    UeScope::with(|scope| {
        for (i, level) in LEVELS.lock().unwrap().iter().enumerate() {
            let found = level.platforms.iter().map(|p| (ElementType::Platform, scope.get(p).as_ptr() as usize)).enumerate()
                .chain(level.cubes.iter().map(|c| (ElementType::Cube, scope.get(c).as_ptr() as usize)).enumerate())
                .chain(level.buttons.iter().map(|c| (ElementType::Button, scope.get(c).as_ptr() as usize)).enumerate())
                .chain(level.lifts.iter().map(|c| (ElementType::Lift, scope.get(c).as_ptr() as usize)).enumerate())
                .chain(level.pipes.iter().map(|c| (ElementType::Pipe, scope.get(c).as_ptr() as usize)).enumerate())
                .chain(level.springpads.iter().map(|c| (ElementType::Springpad, scope.get(c).as_ptr() as usize)).enumerate())
                .find(|(_, (_, addr))| ptr as usize == *addr)
                .map(|(ei, (typ, _))| ElementIndex { cluster_index: i, element_type: typ, element_index: ei});
            if let Some(found) = found {
                return Some(found);
            }
        }
        None
    })
}

pub fn init() {
    UeScope::with(|scope| {
        let mut levels = LEVELS.lock().unwrap();
        let mut lifts = Vec::new();
        let mut pipes = Vec::new();
        let mut springpads = Vec::new();
        for item in scope.iter_global_object_array() {
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
                levels.push(Level {
                    level: scope.object_index(&level),
                    platforms: level.platforms().map(|p| scope.object_index(&p)).collect(),
                    cubes: level.cubes().map(|c| scope.object_index(&c)).collect(),
                    buttons: level.buttons().map(|b| scope.object_index(&b)).collect(),
                    lifts: vec![],
                    pipes: vec![],
                    springpads: vec![],
                })
            }
            if class_name == "BP_Lift_C" && name != "Default__BP_Lift_C" {
                let lift: LiftWrapper = object.upcast();
                lifts.push(lift);
            }
            if class_name == "BP_TravelPipe_C" && name != "Default__BP_TravelPipe_C" {
                let pipe: PipeWrapper = object.upcast();
                pipes.push(pipe);
            }
            if class_name == "BP_Jumppad_C" && name != "Default__BP_Jumppad_C" {
                let springpad: SpringpadWrapper = object.upcast();
                springpads.push(springpad);
            }
            if class_name == "BP_PowerCore_C" && name != "Default__BP_PowerCore_C" {
                let mesh = object.get_field("Mesh").unwrap::<ObjectWrapper>();
                let trigger = object.get_field("Trigger").unwrap::<ObjectWrapper>();
                let fun = mesh.class().find_function("SetCollisionResponseToChannel").unwrap();
                let params = fun.create_argument_struct();
                params.get_field("Channel").unwrap::<&Cell<u8>>().set(3); // Visibility channel
                params.get_field("NewResponse").unwrap::<&Cell<u8>>().set(2); // 'Block' response
                unsafe {
                    fun.call(trigger.as_ptr(), &params);
                }
            }
        }
        assert_eq!(levels.len(), 31);
        levels.sort_by_key(|level| scope.get(level.level).level_index());

        for lift in lifts {
            let level_index: usize = match lift.name().as_str() {
                "BP_Lift_C_1" => 5,
                "BP_Mover7" => 7,
                "BP_Mover6" => 8,
                name => unreachable!("Invalid lift: {name:?}"),
            };
            levels[level_index].lifts.push(scope.object_index(&lift));
        }

        for pipe in &pipes {
            let level_index: usize = match pipe.name().as_str() {
                "BP_TravelPipe_C_6" => 4,
                "BP_TravelPipe_C_5" => 4,
                "BP_TravelPipe_359" => 6,
                "BP_TravelPipe_C_3" => 9,
                "BP_TravelPipe_C_2" => 9,
                "BP_Pipe4" => 18,
                "BP_TravelPipe_C_1" => 18,
                "BP_TravelPipe2_855" => 23,
                "BP_TravelPipe_1816" => 25,
                name => unreachable!("Invalid pipe: {name:?}"),
            };
            levels[level_index].pipes.push(scope.object_index(pipe));
        }

        for pad in &springpads {
            let level_index: usize = match pad.name().as_str() {
                "BP_Jumppad_414" if pad.absolute_location().0 == 250. => 23,
                "BP_Jumppad2_530" => 24,
                "BP_Jumppad_414" if pad.absolute_location().0 == 2075. => 24,
                "BP_Jumppad_514" => 26,
                name => unreachable!("Invalid pipe: {name:?}"),
            };
            levels[level_index].springpads.push(scope.object_index(pad));
        }

        for lvl in &mut *levels {
            lvl.pipes.sort_by_key(|pipe| {
                let (x, y, z) = scope.get(pipe).absolute_location();
                (x as i32, y as i32, z as i32)
            });
        }
    })
}
