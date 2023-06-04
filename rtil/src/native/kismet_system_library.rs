use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::Ordering;
use crate::native::{AActor, AMyCharacter, GlobalObjectArrayWrapper, ObjectWrapper, UObject};
use crate::native::ue::{FLinearColor, FName, FVector, TArray};
use crate::native::{UKISMETSYSTEMLIBRARY_LINETRACESINGLE, FROTATOR_VECTOR};

pub struct KismetSystemLibrary;

impl KismetSystemLibrary {
    pub fn line_trace_single(player: AMyCharacter) -> *mut AActor {
        unsafe {
            let fun: extern "C" fn(
                world_context_object: *mut UObject,
                start: FVector,
                end: FVector,
                trace_channel: i32,
                trace_complex: bool,
                actors_to_ignore: *mut TArray<*mut AActor>,
                draw_debug_type: EDrawDebugTraceType,
                out_hit: *mut FHitResult,
                ignore_self: bool,
                trace_color: FLinearColor,
                trace_hit_color: FLinearColor,
                draw_time: f32
            ) -> bool = std::mem::transmute(UKISMETSYSTEMLIBRARY_LINETRACESINGLE.load(Ordering::SeqCst));

            let mut hit_result = FHitResult {
                bitfield: 0,
                time: 0.0,
                distance: 0.0,
                location: Default::default(),
                impact_point: Default::default(),
                normal: Default::default(),
                impact_normal: Default::default(),
                trace_start: Default::default(),
                trace_end: Default::default(),
                penetration_depth: 0.0,
                item: 0,
                phys_material: Default::default(),
                actor: Default::default(),
                component: Default::default(),
                bone_name: FName::NAME_None,
                face_index: 0,
            };


            let character = ObjectWrapper::new(player.as_ptr() as *mut UObject);
            let controller = character.get_field("Controller").unwrap_object();
            let camera = controller.get_field("PlayerCameraManager").unwrap_object();
            let get_camera_location = camera.class().find_function("GetCameraLocation").unwrap();
            let loc = get_camera_location.create_argument_struct();
            get_camera_location.call(camera.as_ptr(), &loc);
            let loc = loc.get_field("ReturnValue").unwrap_struct();

            let get_camera_rotation = camera.class().find_function("GetCameraRotation").unwrap();
            let rot = get_camera_rotation.create_argument_struct();
            get_camera_rotation.call(camera.as_ptr(), &rot);
            let rot = rot.get_field("ReturnValue").unwrap_struct();
            let frotator_vector: extern_fn!(fn(this: *mut c_void) -> FVector)
                = ::std::mem::transmute(FROTATOR_VECTOR.load(Ordering::SeqCst));
            let direction = frotator_vector(rot.as_ptr());
            let location = FVector {
                x: *loc.get_field("X").unwrap_float(),
                y: *loc.get_field("Y").unwrap_float(),
                z: *loc.get_field("Z").unwrap_float(),
            };

            let hit = fun(
                player.as_ptr() as *mut UObject,
                location,
                FVector { x: direction.x * 100000. + location.x, y: direction.y * 100000. + location.y, z: direction.z * 100000. + location.z },
                0,
                false,
                &mut TArray::new(),
                EDrawDebugTraceType::ForDuration,
                &mut hit_result,
                true,
                FLinearColor { red: 1., green: 0., blue: 0., alpha: 1. },
                FLinearColor { red: 1., green: 0., blue: 0., alpha: 1. },
                10.,
            );

            if !hit {
                ptr::null_mut()
            } else {
                let array = GlobalObjectArrayWrapper::get();
                let item = array.object_array().get(hit_result.actor.object_index.try_into().unwrap());
                assert_eq!(item.serial_number(), hit_result.actor.object_serial_number);
                item.object().as_ptr() as *mut AActor
            }
        }
    }
}

#[repr(C)]
pub struct FHitResult {
    pub bitfield: u8,
    pub time: f32,
    pub distance: f32,
    pub location: FVector,
    pub impact_point: FVector,
    pub normal: FVector,
    pub impact_normal: FVector,
    pub trace_start: FVector,
    pub trace_end: FVector,
    pub penetration_depth: f32,
    pub item: i32,
    pub phys_material: TWeakObjectPtr<c_void>,
    pub actor: TWeakObjectPtr<AActor>,
    pub component: TWeakObjectPtr<c_void>,
    pub bone_name: FName,
    pub face_index: i32,
}

#[repr(C)]
pub struct TWeakObjectPtr<T> {
    object_index: i32,
    object_serial_number: i32,
    _marker: PhantomData<*mut T>,
}
// get rid of derive-implied T: Default
impl<T> Default for TWeakObjectPtr<T> {
    fn default() -> Self {
        TWeakObjectPtr { object_index: 0, object_serial_number: 0, _marker: PhantomData }
    }
}

#[repr(i32)]
enum EDrawDebugTraceType {
    None,
    ForOneFrame,
    ForDuration,
    Persistent,
}
