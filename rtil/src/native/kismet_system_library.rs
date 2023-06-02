use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr;
use std::sync::atomic::Ordering;
use crate::native::{AActor, AMyCharacter, GlobalObjectArrayWrapper, ObjectWrapper, UObject};
use crate::native::ue::{FLinearColor, FName, FRotator, FVector, TArray};
use crate::native::{UKISMETSYSTEMLIBRARY_LINETRACESINGLE, AACTOR_PROCESSEVENT, FROTATOR_VECTOR};

pub struct KismetSystemLibrary;

impl KismetSystemLibrary {
    pub fn line_trace_single(player: AMyCharacter) -> *mut AActor {
        unsafe {
            let fun: extern_fn!(fn(
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
            ) -> bool) = std::mem::transmute(UKISMETSYSTEMLIBRARY_LINETRACESINGLE.load(Ordering::SeqCst));

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


            let process_event: extern_fn!(fn(this: *mut c_void, function: *const c_void, args: *const c_void)) =
                unsafe { ::std::mem::transmute(AACTOR_PROCESSEVENT.load(Ordering::SeqCst)) };
            let character = ObjectWrapper::new(player.as_ptr() as *mut UObject);
            let controller = character.get_field("Controller").unwrap_object();
            let camera = controller.get_field("PlayerCameraManager").unwrap_object();
            let get_camera_location = camera.class().as_struct().iter_properties()
                .find(|p| p.name() == "GetCameraLocation")
                .unwrap().as_ptr();
            let mut location = FVector::default();
            process_event(camera.as_ptr() as *mut c_void, get_camera_location as *mut c_void, &mut location as *mut _ as *mut c_void);

            let get_camera_rotation = camera.class().as_struct().iter_properties()
                .find(|p| p.name() == "GetCameraRotation")
                .unwrap().as_ptr();
            let mut rotation = FRotator::default();
            process_event(camera.as_ptr() as *mut c_void, get_camera_rotation as *mut c_void, &mut rotation as *mut _ as *mut c_void);
            let frotator_vector: extern_fn!(fn(this: *mut FRotator) -> FVector) =
                unsafe { ::std::mem::transmute(FROTATOR_VECTOR.load(Ordering::SeqCst)) };
            let direction = frotator_vector(&mut rotation);
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
                10000.,
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
struct FHitResult {
    bitfield: u8,
    time: f32,
    distance: f32,
    location: FVector,
    impact_point: FVector,
    normal: FVector,
    impact_normal: FVector,
    trace_start: FVector,
    trace_end: FVector,
    penetration_depth: f32,
    item: i32,
    phys_material: TWeakObjectPtr<c_void>,
    actor: TWeakObjectPtr<AActor>,
    component: TWeakObjectPtr<c_void>,
    bone_name: FName,
    face_index: i32,
}

#[repr(C)]
struct TWeakObjectPtr<T> {
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
