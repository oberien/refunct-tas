use ::{LuaInterface, Event, Response};

pub struct Stub {
    delta: f64,
    location: (f32, f32, f32),
    rotation: (f32, f32, f32),
    velocity: (f32, f32, f32),
    acceleration: (f32, f32, f32),
}

impl Stub {
    pub fn new() -> Stub {
        Stub {
            delta: 1.0/60.0,
            location: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            velocity: (0.0, 0.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
        }
    }
}

impl LuaInterface for Stub {
    fn step(&mut self) -> Response<Event> {
        println!("Step");
        Response::Result(Event::Stopped)
    }

    fn press_key(&mut self, key: String) -> Response<()> {
        println!("Press Key: {:?}", key);
        Response::Result(())
    }

    fn release_key(&mut self, key: String) -> Response<()> {
        println!("Release Key: {:?}", key);
        Response::Result(())
    }

    fn move_mouse(&mut self, x: i32, y: i32) -> Response<()> {
        println!("Move Mouse: {}:{}", x, y);
        Response::Result(())
    }

    fn get_delta(&mut self) -> Response<f64> {
        Response::Result(self.delta)
    }

    fn set_delta(&mut self, delta: f64) -> Response<()> {
        self.delta = delta;
        Response::Result(())
    }

    fn get_location(&mut self) -> Response<(f32, f32, f32)> {
        Response::Result(self.location)
    }

    fn set_location(&mut self, x: f32, y: f32, z: f32) -> Response<()> {
        self.location = (x, y, z);
        Response::Result(())
    }

    fn get_rotation(&mut self) -> Response<(f32, f32, f32)> {
        Response::Result(self.rotation)
    }

    fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) -> Response<()> {
        self.rotation = (pitch, yaw, roll);
        Response::Result(())
    }

    fn get_velocity(&mut self) -> Response<(f32, f32, f32)> {
        Response::Result(self.velocity)
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) -> Response<()> {
        self.velocity = (x, y, z);
        Response::Result(())
    }

    fn get_acceleration(&mut self) -> Response<(f32, f32, f32)> {
        Response::Result(self.acceleration)
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) -> Response<()> {
        self.acceleration = (x, y, z);
        Response::Result(())
    }

    fn wait_for_new_game(&mut self) -> Response<()> {
        println!("Wait for new game (triggered)");
        Response::Result(())
    }

    fn sleep(&mut self, time: u64) -> Response<()> {
        println!("Pausing for {}s", time);
        Response::Result(())
    }

    fn print(&mut self, s: String) -> Response<()> {
        println!("print: {:?}", s);
        Response::Result(())
    }
}