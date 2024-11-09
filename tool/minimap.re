static mut MINIMAP_STATE = {
    let mut state = MinimapState {
        size: SETTINGS.minimap_size,
        x: 0.,
        y: 0.,
        scale: 0.,
        alpha: SETTINGS.minimap_alpha,
    };
    state.calculate_minimap_size(state.size);
    state
};

if SETTINGS.minimap_enabled {
    add_component(MINIMAP_COMPONENT);
}
Tas::set_minimap_alpha(SETTINGS.minimap_alpha);

enum MinimapPosition {
    TopLeft,
    TopCenter,
    TopRight,
    CenterRight,
    BottomRight,
    BottomCenter,
    BottomLeft,
    CenterLeft,
    Center,
}

struct MinimapState {
    size: float,
    x: float,
    y: float,
    scale: float,
    alpha: float,
}

impl MinimapState {
    fn calculate_minimap_size(mut self, size: float) {
        self.size = size;
        let viewport = Tas::get_viewport_size();
        let minimap = Tas::minimap_size();
        let expected_height = size * viewport.height.to_float();
        self.scale = expected_height / minimap.height.to_float();
        let tw = minimap.width.to_float() * self.scale;
        let th = minimap.height.to_float() * self.scale;
        match SETTINGS.minimap_position {
           MinimapPosition::TopLeft => {
                self.x = 0.;
                self.y = 0.;
           },
           MinimapPosition::TopCenter => {
                self.x = viewport.width.to_float() / 2. - tw / 2.;
                self.y = 0.;
           },
           MinimapPosition::TopRight => {
                self.x = viewport.width.to_float() - tw;
                self.y = 0.;
           },
           MinimapPosition::CenterRight => {
                self.x = viewport.width.to_float() - tw;
                self.y = viewport.height.to_float() / 2. - th / 2.;
           },
           MinimapPosition::BottomRight => {
                self.x = viewport.width.to_float() - tw;
                self.y = viewport.height.to_float() - th;
           },
           MinimapPosition::BottomCenter => {
                self.x = viewport.width.to_float() / 2. - tw / 2.;
                self.y = viewport.height.to_float() - th;
           },
           MinimapPosition::BottomLeft => {
                self.x = 0.;
                self.y = viewport.height.to_float() - th;
           },
           MinimapPosition::CenterLeft => {
                self.x = 0.;
                self.y = viewport.height.to_float() / 2. - th / 2.;
           },
           MinimapPosition::Center => {
                self.x = viewport.width.to_float() / 2. - tw / 2.;
                self.y = viewport.height.to_float() / 2. - th / 2.;
           },
        }
    }
}

static MINIMAP_COMPONENT = Component {
    id: MINIMAP_COMPONENT_ID,
    conflicts_with: List::of(MINIMAP_COMPONENT_ID),
    draw_hud_text: fn(text: string) -> string { text },
    draw_hud_always: fn() {
        Tas::draw_minimap(MINIMAP_STATE.x, MINIMAP_STATE.y, MINIMAP_STATE.scale, false);
        minimap_draw_player(Tas::get_location(), Tas::get_rotation(), Color {
            red: SETTINGS.player_color_red,
            green: SETTINGS.player_color_green,
            blue: SETTINGS.player_color_blue,
            alpha: 1.,
        });
    },
    tick_mode: TickMode::DontCare,
    requested_delta_time: Option::None,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {},
    on_level_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_cubes_change: fn(old: int, new: int) {},
    on_platforms_change: fn(old: int, new: int) {},
    on_reset: fn(old: int, new: int) {},
    on_element_pressed: fn(index: ElementIndex) {},
    on_element_released: fn(index: ElementIndex) {},
    on_key_down: fn(key_code: KeyCode, is_repeat: bool) {},
    on_key_down_always: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key_code: KeyCode) {},
    on_key_up_always: fn(key: KeyCode) {},
    on_mouse_move: fn(x: int, y: int) {},
    on_component_enter: fn() {},
    on_component_exit: fn() {},
    on_resolution_change: fn() { MINIMAP_STATE.calculate_minimap_size(MINIMAP_STATE.size); },
    on_menu_open: fn() {},
};

fn minimap_draw_player(location: Location, rotation: Rotation, mut color: Color) {
    if SETTINGS.minimap_enabled {
        let minimap_size = Tas::minimap_size();
        let minimap_scale = MINIMAP_STATE.scale;
        let player_minimap_size = Tas::player_minimap_size();
        // scale minimap icon to be 10% of the map width
        let player_minimap_scale = minimap_size.width.to_float() * minimap_scale * 0.1 / player_minimap_size.width.to_float();
        let ue_left = 5784.;
        let ue_right = -5784.;
        let ue_top = 6659.;
        let ue_bottom = -4909.;
        let ue_width = ue_left - ue_right;
        let ue_height = ue_top - ue_bottom;
        let ratio_x = minimap_size.width.to_float() / ue_width;
        let ratio_y = minimap_size.height.to_float() / ue_height;
        let minimap_x0 = ue_left * ratio_x;
        let minimap_y0 = ue_top * ratio_y;
        let ue_x = location.x;
        let ue_y = location.y;
        let minimap_x = minimap_x0 - ue_x * ratio_x;
        let minimap_y = minimap_y0 - ue_y * ratio_y;
        let hud_width = player_minimap_size.width.to_float() * player_minimap_scale;
        let hud_height = player_minimap_size.height.to_float() * player_minimap_scale;
        let hud_x = MINIMAP_STATE.x + minimap_x * minimap_scale - hud_width / 2.;
        let hud_y = MINIMAP_STATE.y + minimap_y * minimap_scale - hud_height / 2.;
        color.alpha = MINIMAP_STATE.alpha;
        Tas::draw_player_minimap(hud_x, hud_y, hud_width, hud_height, rotation.yaw - 90., color);
    }
}

static mut MINIMAP_LABEL = Text { text: if SETTINGS.minimap_enabled { "Disable Minimap" } else { "Enable Minimap" } };

fn create_minimap_menu() -> Ui {
    let mut pos = Number { number: 0 };
    match SETTINGS.minimap_position {
        MinimapPosition::TopLeft => { pos.number = 0 },
        MinimapPosition::TopCenter => { pos.number = 1 },
        MinimapPosition::TopRight => { pos.number = 2 },
        MinimapPosition::CenterRight => { pos.number = 3 },
        MinimapPosition::BottomRight => { pos.number = 4 },
        MinimapPosition::BottomCenter => { pos.number = 5 },
        MinimapPosition::BottomLeft => { pos.number = 6 },
        MinimapPosition::CenterLeft => { pos.number = 7 },
        MinimapPosition::Center => { pos.number = 8 },
    }
    Ui::new("Minimap:", List::of(
        UiElement::Button(UiButton {
            label: MINIMAP_LABEL,
            onclick: fn(label: Text) {
                if SETTINGS.minimap_enabled {
                    remove_component(MINIMAP_COMPONENT);
                    MINIMAP_LABEL.text = "Enable Minimap";
                    SETTINGS.minimap_enabled = false;
                    SETTINGS.store();
                } else {
                    add_component(MINIMAP_COMPONENT);
                    MINIMAP_LABEL.text = "Disable Minimap";
                    SETTINGS.minimap_enabled = true;
                    SETTINGS.store();
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Size (0.0 - 1.0)" },
            input: f"{MINIMAP_STATE.size}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(size) => if 0.0 <= size && size <= 1.0 {
                        MINIMAP_STATE.calculate_minimap_size(size);
                        SETTINGS.minimap_size = size;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Alpha (0.0 - 1.0)" },
            input: f"{MINIMAP_STATE.alpha}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(alpha) => if 0.0 <= alpha && alpha <= 1.0 {
                        MINIMAP_STATE.alpha = alpha;
                        Tas::set_minimap_alpha(alpha);
                        SETTINGS.minimap_alpha = alpha;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::Chooser(Chooser {
            label: Text { text: "Position" },
            options: List::of(
                Text { text: "Top Left" },
                Text { text: "Top Center" },
                Text { text: "Top Right" },
                Text { text: "Center Right" },
                Text { text: "Bottom Right" },
                Text { text: "Bottom Center" },
                Text { text: "Bottom Left" },
                Text { text: "Center Left" },
                Text { text: "Center" },
            ),
            selected: pos.number,
            onchange: fn(index: int) {
                match index {
                    0 => { SETTINGS.minimap_position = MinimapPosition::TopLeft; },
                    1 => { SETTINGS.minimap_position = MinimapPosition::TopCenter; },
                    2 => { SETTINGS.minimap_position = MinimapPosition::TopRight; },
                    3 => { SETTINGS.minimap_position = MinimapPosition::CenterRight; },
                    4 => { SETTINGS.minimap_position = MinimapPosition::BottomRight; },
                    5 => { SETTINGS.minimap_position = MinimapPosition::BottomCenter; },
                    6 => { SETTINGS.minimap_position = MinimapPosition::BottomLeft; },
                    7 => { SETTINGS.minimap_position = MinimapPosition::CenterLeft; },
                    8 => { SETTINGS.minimap_position = MinimapPosition::Center; },
                    _ => panic(f"unknown index {index}"),
                }
                SETTINGS.store();
                pos.number = index;
                MINIMAP_STATE.calculate_minimap_size(MINIMAP_STATE.size);
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

