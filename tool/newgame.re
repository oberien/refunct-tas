fn new_game_nothing() {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_FUNCTION = on_level_change_noop_function;
    ON_RESET_FUNCTION = on_reset_noop_function;
}
fn new_game_100_percent() {
    NEW_GAME_FUNCTION = new_game_100_percent_function;
    ON_LEVEL_CHANGE_FUNCTION = on_level_change_noop_function;
    ON_RESET_FUNCTION = on_reset_noop_function;
    /*ON_LEVEL_CHANGE_VALUE = Option::Some(30);*/
    /*ON_RESET_VALUE = Option::Some(30);*/
}
fn new_game_level_reset(level_change: int, reset: int) {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_VALUE = level_change;
    ON_LEVEL_CHANGE_FUNCTION = on_level_zero_set_value_function;
    ON_RESET_VALUE = reset;
    ON_RESET_FUNCTION = on_reset_set_value_function;
}
fn new_game_practice(practice: Practice) {
    NEW_GAME_PRACTICE = practice;
    NEW_GAME_FUNCTION = new_game_practice_function;
    ON_LEVEL_CHANGE_VALUE = practice.button;
    ON_LEVEL_CHANGE_FUNCTION = on_level_zero_set_value_function;
    ON_RESET_VALUE = practice.button;
    ON_RESET_FUNCTION = on_reset_set_value_function;
}
fn new_game_randomizer() {
    NEW_GAME_FUNCTION = randomizer_new_game_function;
    ON_LEVEL_CHANGE_FUNCTION = randomizer_on_level_change_function;
    ON_RESET_FUNCTION = randomizer_on_reset_function;
}

struct Practice {
    name: string,
    button: int,
    location: Location,
    rotation: Rotation,
}

static mut NEW_GAME_PRACTICE = Practice { name: "none", button: 0, location: Location { x: 0., y: 0., z: 0. }, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. } };

// new game
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

// on level change
static mut ON_LEVEL_CHANGE_FUNCTION = on_level_change_noop_function;
fn on_level_change_noop_function(level: int) {}
static mut ON_LEVEL_CHANGE_VALUE = 0;
fn on_level_zero_set_value_function(level: int) {
    if level == 0 {
        Tas::set_level(ON_LEVEL_CHANGE_VALUE);
    }
}
fn on_level_change(level: int) {
    ON_LEVEL_CHANGE_FUNCTION(level);
}

// on reset
static mut ON_RESET_FUNCTION = on_reset_noop_function;
fn on_reset_noop_function(reset: int) {}
static mut ON_RESET_VALUE = 0;
fn on_reset_set_value_function(reset: int) {
    Tas::set_level(ON_RESET_VALUE);
}
fn on_reset(reset: int) {
    ON_RESET_FUNCTION(reset);
}
