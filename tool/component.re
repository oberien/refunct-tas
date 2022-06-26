enum TickMode {
    DontCare,
    Yield,
}

static MULTIPLAYER_COMPONENT_ID = 1;
static NEW_GAME_100_PERCENT_COMPONENT_ID = 2;
static NEW_GAME_ALL_BUTTONS_COMPONENT_ID = 3;
static PRACTICE_COMPONENT_ID = 4;
static RANDOMIZER_COMPONENT_ID = 5;
static TAS_COMPONENT_ID = 6;
static WINDSCREEN_WIPERS_COMPONENT_ID = 7;
static NEW_GAME_NGG_COMPONENT_ID = 8;

struct Component {
    id: int,
    conflicts_with: List<int>,
    draw_hud: fn(string) -> string,
    tick_mode: TickMode,
    // largest delta time wins and is used for the next frame
    requested_delta_time: Option<float>,
    on_tick: fn(),
    on_yield: fn(),
    on_new_game: fn(),
    on_level_change: fn(int, int),
    on_reset: fn(int, int),
    on_platforms_change: fn(int, int),
    on_buttons_change: fn(int, int),
    on_key_down: fn(KeyCode, bool),
    on_key_up: fn(KeyCode),
    on_mouse_move: fn(int, int),
    on_component_exit: fn(),
}

static mut CURRENT_COMPONENTS = List::new();

fn add_component(component: Component) {
    let mut i = 0;
    loop {
        let comp = match CURRENT_COMPONENTS.get(i) {
            Option::Some(comp) => comp,
            Option::None => break,
        };

        if component.conflicts_with.contains(comp.id) || comp.conflicts_with.contains(component.id) {
            let on_component_exit = comp.on_component_exit;
            on_component_exit();
            CURRENT_COMPONENTS.swap_remove(i);
        } else {
            i += 1;
        }
    }
    CURRENT_COMPONENTS.push(component);
    CURRENT_COMPONENTS.sort();
}

fn remove_component(component: Component) {
    let mut i = 0;
    loop {
        let comp = match CURRENT_COMPONENTS.get(i) {
            Option::Some(comp) => comp,
            Option::None => break,
        };
        if comp == component {
            let on_component_exit = comp.on_component_exit;
            on_component_exit();
            CURRENT_COMPONENTS.swap_remove(i);
            return;
        }
        i += 1;
    }
}
