Tas::set_reticle_width(SETTINGS.reticle_w);
Tas::set_reticle_height(SETTINGS.reticle_h);

fn create_reticle_menu() -> Ui {
    static mut PLAYER_RETICLE_WIDTH_INPUT_LABEL = Text { text: "Reticle Width" };
    static mut PLAYER_RETICLE_HEIGHT_INPUT_LABEL = Text { text: "Reticle Height" };
    static mut PLAYER_RETICLE_SCALE_INPUT_LABEL = Text { text: "Reticle Scale" };
    Ui::new("Reticle:", List::of(
        UiElement::FloatInput(FloatInput {
            label: PLAYER_RETICLE_WIDTH_INPUT_LABEL,
            input: f"{SETTINGS.reticle_w}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                PLAYER_RETICLE_WIDTH_INPUT_LABEL.text = "Reticle Width";
                match input.parse_float() {
                    Result::Ok(num) => {
                        Tas::set_reticle_width(num);
                        SETTINGS.reticle_w = num;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {
                        PLAYER_RETICLE_WIDTH_INPUT_LABEL.text = f"Reticle Width (invalid value)";
                    },
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: PLAYER_RETICLE_HEIGHT_INPUT_LABEL,
            input: f"{SETTINGS.reticle_h}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                PLAYER_RETICLE_HEIGHT_INPUT_LABEL.text = "Reticle Height";
                match input.parse_float() {
                    Result::Ok(num) => {
                        Tas::set_reticle_height(num);
                        SETTINGS.reticle_h = num;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {
                        PLAYER_RETICLE_HEIGHT_INPUT_LABEL.text = f"Reticle Height (invalid value)";
                    },
                }
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

fn create_camera_menu() -> Ui {
    Ui::new("Camera:", List::of(
        UiElement::Chooser(Chooser {
            label: Text { text: "Camera Mode" },
            options: List::of(
                Text { text: "Perspective" },
                Text { text: "Orthographic" },
            ),
            selected: Tas::get_camera_mode(),
            onchange: fn(index: int) { Tas::set_camera_mode(index); },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}

fn create_player_menu() -> Ui {
    Ui::new("Player:", List::of(
        UiElement::Button(UiButton {
            label: Text { text: "Reticle" },
            onclick: fn(label: Text) { enter_ui(create_reticle_menu()); }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Camera" },
            onclick: fn(label: Text) { enter_ui(create_camera_menu()); }
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}
