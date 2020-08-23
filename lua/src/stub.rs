use std::cell::RefCell;

use ::{LuaInterface, Event, IfaceResult, Context, LuaResult};

pub struct Stub {
    delta: RefCell<f64>,
    location: RefCell<(f32, f32, f32)>,
    rotation: RefCell<(f32, f32, f32)>,
    velocity: RefCell<(f32, f32, f32)>,
    acceleration: RefCell<(f32, f32, f32)>,
    pawn_id: RefCell<u32>,
}

impl Stub {
    pub fn new() -> Stub {
        Stub {
            delta: RefCell::new(1.0/60.0),
            location: RefCell::new((0.0, 0.0, 0.0)),
            rotation: RefCell::new((0.0, 0.0, 0.0)),
            velocity: RefCell::new((0.0, 0.0, 0.0)),
            acceleration: RefCell::new((0.0, 0.0, 0.0)),
            pawn_id: RefCell::new(0),
        }
    }
}

impl LuaInterface for Stub {
    fn step(&self, _lua: Context<'_>) -> LuaResult<Event> {
        println!("Step");
        Ok(Event::Stopped)
    }

    fn press_key(&self, key: String) -> IfaceResult<()> {
        println!("Press Key: {:?}", key);
        Ok(())
    }

    fn release_key(&self, key: String) -> IfaceResult<()> {
        println!("Release Key: {:?}", key);
        Ok(())
    }

    fn key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> IfaceResult<()> {
        println!("Key Down: {}, {}, {}", key_code, character_code, is_repeat);
        Ok(())
    }

    fn key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> IfaceResult<()> {
        println!("Key Up: {}, {}, {}", key_code, character_code, is_repeat);
        Ok(())
    }

    fn move_mouse(&self, x: i32, y: i32) -> IfaceResult<()> {
        println!("Move Mouse: {}:{}", x, y);
        Ok(())
    }

    fn get_delta(&self) -> IfaceResult<f64> {
        Ok(*self.delta.borrow())
    }

    fn set_delta(&self, delta: f64) -> IfaceResult<()> {
        *self.delta.borrow_mut() = delta;
        Ok(())
    }

    fn get_location(&self) -> IfaceResult<(f32, f32, f32)> {
        Ok(*self.location.borrow())
    }

    fn set_location(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        *self.location.borrow_mut() = (x, y, z);
        Ok(())
    }

    fn get_rotation(&self) -> IfaceResult<(f32, f32, f32)> {
        Ok(*self.rotation.borrow())
    }

    fn set_rotation(&self, pitch: f32, yaw: f32, roll: f32) -> IfaceResult<()> {
        *self.rotation.borrow_mut() = (pitch, yaw, roll);
        Ok(())
    }

    fn get_velocity(&self) -> IfaceResult<(f32, f32, f32)> {
        Ok(*self.velocity.borrow())
    }

    fn set_velocity(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        *self.velocity.borrow_mut() = (x, y, z);
        Ok(())
    }

    fn get_acceleration(&self) -> IfaceResult<(f32, f32, f32)> {
        Ok(*self.acceleration.borrow())
    }

    fn set_acceleration(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        *self.acceleration.borrow_mut() = (x, y, z);
        Ok(())
    }

    fn wait_for_new_game(&self, _lua: Context<'_>) -> LuaResult<()> {
        println!("Wait for new game (triggered)");
        Ok(())
    }

    fn draw_line(&self, startx: f32, starty: f32, endx: f32, endy: f32, color: (f32, f32, f32, f32), thickness: f32) -> IfaceResult<()> {
        println!("Draw Line from ({}:{}) to ({}:{}) with color {:?} and thickness {}", startx, endx,
                 starty, endy, color, thickness);
        Ok(())
    }

    fn draw_text(&self, text: String, color: (f32, f32, f32, f32), x: f32, y: f32, scale: f32, scale_position: bool) -> IfaceResult<()> {
        println!("Draw Text \"{:?}\" at ({}:{}) with color {:?}, scale {} and scale_position: {}",
                 text, x, y, color, scale, scale_position);
        Ok(())
    }

    fn project(&self, x: f32, y: f32, z: f32) -> IfaceResult<(f32, f32, f32)> {
        println!("Project {} {} {}", x, y, z);
        Ok((0.0, 0.0, 1.0))
    }

    fn print(&self, s: String) -> IfaceResult<()> {
        println!("print: {:?}", s);
        Ok(())
    }

    fn working_dir(&self) -> IfaceResult<String> {
        Ok(".".to_string())
    }

    fn spawn_pawn(&self) -> IfaceResult<u32> {
        println!("Spawn Pawn");
        let pawn_id = *self.pawn_id.borrow();
        *self.pawn_id.borrow_mut() += 1;
        Ok(pawn_id)
    }

    fn move_pawn(&self, pawn_id: u32, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        println!("Move Pawn {} to {}, {}, {}", pawn_id, x, y, z);
        Ok(())
    }

    fn destroy_pawn(&self, pawn_id: u32) -> IfaceResult<()> {
        println!("Destroy Pawn {}", pawn_id);
        Ok(())
    }

    fn tcp_connect(&self, server_port: String) -> IfaceResult<()> {
        println!("Tcp Connect: {}", server_port);
        Ok(())
    }

    fn tcp_disconnect(&self) -> IfaceResult<()> {
        println!("Tcp Disconnect");
        Ok(())
    }

    fn tcp_join_room(&self, room: String, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        println!("Tcp Join Room {:?} at {}, {}, {}", room, x, y, z);
        Ok(())
    }
    fn tcp_move(&self, x: f32, y: f32, z: f32) -> IfaceResult<()> {
        println!("Tcp move: {}, {}, {}", x, y, z);
        Ok(())
    }

    fn set_level(&self, level: i32) -> IfaceResult<()> {
        println!("Set Level: {}", level);
        Ok(())
    }
}