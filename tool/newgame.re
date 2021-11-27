fn new_game_nothing() {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_VALUE = Option::None;
    ON_RESET_VALUE = Option::None;
}
fn new_game_100_percent() {
    NEW_GAME_FUNCTION = new_game_100_percent_function;
    ON_LEVEL_CHANGE_VALUE = Option::None;
    ON_RESET_VALUE = Option::None;
    /*ON_LEVEL_CHANGE_VALUE = Option::Some(30);*/
    /*ON_RESET_VALUE = Option::Some(30);*/
}
fn new_game_level_reset(level_change: int, reset: int) {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_VALUE = Option::Some(level_change);
    ON_RESET_VALUE = Option::Some(reset);
}
fn new_game_practice(practice: Practice) {
    NEW_GAME_PRACTICE = practice;
    NEW_GAME_FUNCTION = new_game_practice_function;
    ON_LEVEL_CHANGE_VALUE = Option::Some(practice.button);
    ON_RESET_VALUE = Option::Some(practice.button);
}

struct Practice {
    name: string,
    button: int,
    location: Location,
    rotation: Rotation,
}

static mut NEW_GAME_PRACTICE = Practice { name: "none", button: 0, location: Location { x: 0., y: 0., z: 0. }, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. } };

static mut NEW_GAME_FUNCTION = new_game_noop_function;
fn new_game_noop_function() {}
fn new_game_100_percent_function() {
    teleport_buttons(30);
    /*teleport_all_buttons_up_to(30);*/
    teleport_all_platforms();
    teleport_all_cubes();
    teleport_exact(30);
}
fn new_game_practice_function() {
    let old_delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1./2.));
    wait(9);
    Tas::set_rotation(NEW_GAME_PRACTICE.rotation);
    Tas::set_location(NEW_GAME_PRACTICE.location);
    Tas::set_velocity(Velocity { x: 0., y: 0., z: 0. });
    Tas::set_acceleration(Acceleration { x: 0., y: 0., z: 0. });
    Tas::set_delta(old_delta);

    /*let old_delta = Tas::get_delta();*/
    /*Tas::set_delta(Option::Some(1./60.));*/
    /*teleport_buttons(NEW_GAME_PRACTICE.button);*/
    /*Tas::set_delta(Option::Some(1./2.));*/
    /*wait(10);*/
    /*Tas::set_rotation(NEW_GAME_PRACTICE.rotation);*/
    /*Tas::set_location(NEW_GAME_PRACTICE.location);*/
    /*Tas::set_velocity(Velocity { x: 0., y: 0., z: 0. });*/
    /*Tas::set_acceleration(Acceleration { x: 0., y: 0., z: 0. });*/
    /*wait(10);*/
    /*Tas::set_delta(old_delta);*/
}

static mut ON_LEVEL_CHANGE_VALUE = Option::None;
static mut ON_RESET_VALUE = Option::None;

fn on_level_change(level: int) {
    if level == 0 {
        match ON_LEVEL_CHANGE_VALUE {
            Option::Some(level) => Tas::set_level(level),
            Option::None => (),
        }
    }
}
fn on_reset(reset: int) {
    match ON_RESET_VALUE {
        Option::Some(reset) => Tas::set_level(reset),
        Option::None => (),
    }
}
