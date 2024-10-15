include "settings.re";
include "keys.re";
include "component.re"
include "prelude.re";
include "ui.re";
include "teleport.re";
include "randomizer.re";
include "newgame.re";
include "practice.re";
include "windshieldwipers.re";
include "tas.re";
include "timer.re";
include "minimap.re";
include "multiplayer.re";
include "movement.re";
include "misc.re";
include "mapeditor.re";
include "world_options.re";

static mut NEW_VERSION: Option<string> = Tas::new_version_string();

fn create_start_menu() -> Ui {
    let mut start_menu_text = Text { text: "Press 'm' for menu." };
    Ui {
        name: start_menu_text,
        elements: List::new(),
        on_draw: Option::Some(fn() {
            let mut text = "Press 'm' for menu.";
            match NEW_VERSION {
                Option::Some(t) => text = f"{text}\n{t}",
                Option::None => (),
            }
            for comp in CURRENT_COMPONENTS {
                let draw_hud_text = comp.draw_hud_text;
                text = draw_hud_text(text);
            }
            if SETTINGS.show_character_stats {
                let loc = Tas::get_location();
                let vel = Tas::get_velocity();
                let rot = Tas::get_rotation();
                let acc = Tas::get_acceleration();
                let velxy = vel.x*vel.x + vel.y*vel.y;
                let velxy = velxy.sqrt();
                let velxyz = vel.x*vel.x + vel.y*vel.y + vel.z*vel.z;
                let velxyz = velxyz.sqrt();
                text = f"{text}
x: {loc.x:8.2}    y: {loc.y:8.2}    z: {loc.z:8.2}
velx {vel.x:8.2}    vely: {vel.y:8.2}    velz: {vel.z:8.2}
velxy: {velxy:8.2}
velxyz: {velxyz:8.2}
accx {acc.x:8.2}    accy: {acc.y:8.2}    accz: {acc.z:8.2}
pitch {rot.pitch:8.2}    yaw: {rot.yaw:8.2}    roll: {rot.roll:8.2}";
            }
            if SETTINGS.show_game_stats {
                text = f"{text}
Level: {GAME_STATS.current_level} (Total: {GAME_STATS.total_levels})
Buttons: {GAME_STATS.current_buttons} (Total: {GAME_STATS.total_buttons})
Cubes: {GAME_STATS.current_cubes} (Total: {GAME_STATS.total_cubes})
Platforms: {GAME_STATS.current_platforms} (Total: {GAME_STATS.total_platforms})
Resets: {GAME_STATS.total_resets} | Any%: {GAME_STATS.total_runs_completed} | 100%: {GAME_STATS.total_100_runs_completed} | All Platforms: {GAME_STATS.total_all_platforms_runs_completed} | All Cubes: {GAME_STATS.total_all_cubes_runs_completed} | Lowest #Platforms: {GAME_STATS.fewest_platform_run}";
            }
            start_menu_text.text = text;
        }),
        selected: 0,
    }
}

fn create_base_menu() -> Ui {
    Ui::new("Menu:", List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Practice" },
            onclick: fn(label: Text) { enter_ui(create_practice_menu()); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Randomizer" },
            onclick: fn(label: Text) { enter_ui(create_randomizer_menu()); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "New Game Actions" },
            onclick: fn(label: Text) { enter_ui(create_new_game_actions_menu()); },
        }),
//        UiElement::Button(UiButton {
//            label: Text { text: "Multiplayer" },
//            onclick: fn(label: Text) { enter_ui(create_multiplayer_menu()); },
//        }),
        UiElement::Button(UiButton {
            label: Text { text: "Map Editor" },
            onclick: fn(label: Text) { enter_ui(create_map_editor_menu()); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Misc" },
            onclick: fn(label: Text) { enter_ui(create_misc_menu()); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Settings" },
            onclick: fn(label: Text) { enter_ui(create_settings_menu()); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

Tas::set_default_font();
enter_ui(create_start_menu());

loop {
    let mut tick_mode = TickMode::DontCare;
    for comp in CURRENT_COMPONENTS {
        match comp.tick_mode {
            TickMode::DontCare => (),
            TickMode::Yield => tick_mode = TickMode::Yield,
        }
    }
    Tas::show_hud();
    Tas::set_sky_time_speed(SETTINGS.sky_time_speed);
    step_frame(tick_mode);
}
