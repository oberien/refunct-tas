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

static mut NEW_GAME_FUNCTION = new_game_noop_function;
fn new_game_noop_function() {}
fn new_game_100_percent_function() {
    teleport_buttons(30);
    /*teleport_all_buttons_up_to(30);*/
    teleport_all_platforms();
    teleport_all_cubes();
    teleport_exact(30);
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
