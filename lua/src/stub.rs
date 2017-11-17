use ::{LuaInterface, Response};

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
    fn step(&mut self) -> Response {
        println!("Step");
        Response::Stopped
    }

    fn press_key(&mut self, key: String) {
        println!("Press Key: {:?}", key);
    }

    fn release_key(&mut self, key: String) {
        println!("Release Key: {:?}", key);
    }

    fn move_mouse(&mut self, x: i32, y: i32) {
        println!("Move Mouse: {}:{}", x, y);
    }

    fn get_delta(&mut self) -> f64 {
        self.delta
    }

    fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    fn get_location(&mut self) -> (f32, f32, f32) {
        self.location
    }

    fn set_location(&mut self, x: f32, y: f32, z: f32) {
        self.location = (x, y, z);
    }

    fn get_rotation(&mut self) -> (f32, f32, f32) {
        self.rotation
    }

    fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = (x, y, z);
    }

    fn get_velocity(&mut self) -> (f32, f32, f32) {
        self.velocity
    }

    fn set_velocity(&mut self, x: f32, y: f32, z: f32) {
        self.velocity = (x, y, z);
    }

    fn get_acceleration(&mut self) -> (f32, f32, f32) {
        self.acceleration
    }

    fn set_acceleration(&mut self, x: f32, y: f32, z: f32) {
        self.acceleration = (x, y, z);
    }

    fn wait_for_new_game(&mut self) {
        println!("Wait for new game (triggered)");
    }

    fn print(&mut self, s: String) {
        println!("print: {:?}", s);
    }
}