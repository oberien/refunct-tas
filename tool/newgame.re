fn new_game_nothing() {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_FUNCTION = on_level_change_noop_function;
    ON_RESET_FUNCTION = on_reset_noop_function;
}
fn new_game_100_percent() {
    NEW_GAME_FUNCTION = new_game_100_percent_function;
    ON_LEVEL_CHANGE_FUNCTION = on_level_change_noop_function;
    ON_RESET_FUNCTION = on_reset_noop_function;
}
fn new_game_all_buttons() {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_FUNCTION = on_level_change_29_function;
    ON_RESET_FUNCTION = on_reset_0_function;
}
fn new_game_ngg() {
    NEW_GAME_FUNCTION = new_game_noop_function;
    ON_LEVEL_CHANGE_FUNCTION = on_level_change_1_function;
    ON_RESET_FUNCTION = on_reset_1_function;
}

static mut NEW_GAME_FUNCTION = new_game_noop_function;
fn new_game_noop_function() {}
fn new_game_100_percent_function() {
    teleport_buttons(30);
    teleport_all_platforms();
    teleport_all_cubes();
    teleport_exact(30);
}


static mut ON_LEVEL_CHANGE_FUNCTION = on_level_change_noop_function;
fn on_level_change_noop_function(reset: int) {}
fn on_level_change_29_function(reset: int) {
    if reset == 0 {
        Tas::set_level(29);
    }
}
fn on_level_change_1_function(reset: int) {
    if reset == 0 {
        Tas::set_level(1);
    }
}

static mut ON_RESET_FUNCTION = on_reset_noop_function;
fn on_reset_noop_function(level: int) {}
fn on_reset_0_function(level: int) {
    Tas::set_level(0);
}
fn on_reset_1_function(level: int) {
    Tas::set_level(1);
}

fn on_level_change(level: int) {
    ON_LEVEL_CHANGE_FUNCTION(level);
}
fn on_reset(reset: int) {
    ON_RESET_FUNCTION(reset);
}
