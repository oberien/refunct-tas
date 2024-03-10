Tas::set_reflection_render_scale(SETTINGS.reflection_render_scale);
Tas::set_lighting_casts_shadows(SETTINGS.lighting_casts_shadows);
Tas::set_sky_light_enabled(SETTINGS.sky_light_enabled);
Tas::set_time_dilation(SETTINGS.time_dilation);
Tas::set_gravity(SETTINGS.gravity);
Tas::set_time_of_day(SETTINGS.time_of_day);
Tas::set_sky_time_speed(SETTINGS.sky_time_speed);
Tas::set_sky_light_brightness(SETTINGS.sky_light_brightness);
Tas::set_sky_light_intensity(SETTINGS.sky_light_intensity);
Tas::set_stars_brightness(TimeOfDay::Day, SETTINGS.day_stars_brightness);
Tas::set_stars_brightness(TimeOfDay::Night, SETTINGS.night_stars_brightness);
Tas::set_fog_enabled(SETTINGS.fog_enabled);
Tas::set_cloud_speed(SETTINGS.cloud_speed);
Tas::set_gamma(SETTINGS.display_gamma);
Tas::set_kill_z(SETTINGS.kill_z);
Tas::set_cloud_redness(SETTINGS.cloud_redness);
Tas::set_outro_dilated_duration(SETTINGS.outro_dilated_duration);
Tas::set_outro_time_dilation(SETTINGS.outro_time_dilation);
Tas::set_sun_redness(SETTINGS.sun_redness);
Tas::set_screen_percentage(SETTINGS.screen_percentage);

fn create_world_options_menu() -> Ui {
    let mut lighting_casts_shadows_button_text = Text { text: f"Lighting Casts Shadows: {SETTINGS.lighting_casts_shadows}" };
    let mut sky_light_enabled_button_text = Text { text: f"Sky Light Enabled: {SETTINGS.sky_light_enabled}" };
    let mut fog_enabled_button_text = Text { text: f"Fog Enabled: {SETTINGS.fog_enabled}" };
    Ui::new("World Options:", List::of(
        UiElement::Button(UiButton {
            label: lighting_casts_shadows_button_text,
            onclick: fn(label: Text) {
                SETTINGS.toggle_lighting_casts_shadows();
                Tas::set_lighting_casts_shadows(SETTINGS.lighting_casts_shadows);
                lighting_casts_shadows_button_text.text = f"Lighting Casts Shadows: {SETTINGS.lighting_casts_shadows}";
            },
        }),
        UiElement::Button(UiButton {
            label: sky_light_enabled_button_text,
            onclick: fn(label: Text) {
                SETTINGS.toggle_sky_light_enabled();
                Tas::set_sky_light_enabled(SETTINGS.sky_light_enabled);
                sky_light_enabled_button_text.text = f"Sky Light Enabled: {SETTINGS.sky_light_enabled}";
            },
        }),
        UiElement::Button(UiButton {
            label: fog_enabled_button_text,
            onclick: fn(label: Text) {
                SETTINGS.toggle_fog_enabled();
                Tas::set_fog_enabled(SETTINGS.fog_enabled);
                fog_enabled_button_text.text = f"Fog Enabled: {SETTINGS.fog_enabled}";
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Time Dilation" },
            input: f"{SETTINGS.time_dilation}",
            onclick: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(dilation) => {
                        SETTINGS.time_dilation = dilation;
                        SETTINGS.store();
                        Tas::set_time_dilation(dilation);
                    },
                    Result::Err(e) => {},
                }
            },
            onchange: fn(input: string) {},
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Gravity" },
            input: f"{SETTINGS.gravity}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(gravity) => {
                        Tas::set_gravity(gravity);
                        SETTINGS.gravity = gravity;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Kill Z" },
            input: f"{SETTINGS.kill_z}",
            onclick: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(kill_z) => {
                        Tas::set_kill_z(kill_z);
                        SETTINGS.kill_z = kill_z;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
            onchange: fn(input: string) {},
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Outro Dilated Duration" },
            input: f"{SETTINGS.outro_dilated_duration}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(duration) => {
                        Tas::set_outro_dilated_duration(duration);
                        SETTINGS.outro_dilated_duration = duration;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Outro Time Dilation" },
            input: f"{SETTINGS.outro_time_dilation}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(dilation) => {
                        Tas::set_outro_time_dilation(dilation);
                        SETTINGS.outro_time_dilation = dilation;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Time Of Day" },
            input: f"{Tas::get_time_of_day()}",
            onclick: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(time) => {
                        Tas::set_time_of_day(time);
                        SETTINGS.time_of_day = time;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
            onchange: fn(input: string) {},
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Sky Time Speed" },
            input: f"{SETTINGS.sky_time_speed}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(speed) => {
                        Tas::set_sky_time_speed(speed);
                        SETTINGS.sky_time_speed = speed;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Sky Light Brightness" },
            input: f"{SETTINGS.sky_light_brightness}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(brightness) => {
                        Tas::set_sky_light_brightness(brightness);
                        SETTINGS.sky_light_brightness = brightness;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Sky Light Intensity" },
            input: f"{SETTINGS.sky_light_intensity}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(intensity) => {
                        Tas::set_sky_light_intensity(intensity);
                        SETTINGS.sky_light_intensity = intensity;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Day Stars Brightness" },
            input: f"{SETTINGS.day_stars_brightness}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(brightness) => {
                        Tas::set_stars_brightness(TimeOfDay::Day, brightness);
                        SETTINGS.day_stars_brightness = brightness;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Night Stars Brightness" },
            input: f"{SETTINGS.night_stars_brightness}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(brightness) => {
                        Tas::set_stars_brightness(TimeOfDay::Night, brightness);
                        SETTINGS.night_stars_brightness = brightness;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Sun Color Red" },
            input: f"{SETTINGS.sun_redness}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(color) => {
                        Tas::set_sun_redness(color);
                        SETTINGS.sun_redness = color;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Cloud Color Red" },
            input: f"{SETTINGS.cloud_redness}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(color) => {
                        Tas::set_cloud_redness(color);
                        SETTINGS.cloud_redness = color;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Cloud Speed" },
            input: f"{SETTINGS.cloud_speed}",
            onclick: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(speed) => {
                        Tas::set_cloud_speed(speed);
                        SETTINGS.cloud_speed = speed;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
            onchange: fn(input: string) {},
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Reflection Render Scale" },
            input: f"{SETTINGS.reflection_render_scale}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_int() {
                    Result::Ok(scale) => {
                        Tas::set_reflection_render_scale(scale);
                        SETTINGS.reflection_render_scale = scale;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Display Gamma" },
            input: f"{SETTINGS.display_gamma}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(gamma) => {
                        Tas::set_gamma(gamma);
                        SETTINGS.display_gamma = gamma;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::FloatInput(FloatInput {
            label: Text { text: "Screen Percentage" },
            input: f"{SETTINGS.screen_percentage}",
            onclick: fn(input: string) {},
            onchange: fn(input: string) {
                match input.parse_float() {
                    Result::Ok(percentage) => {
                        Tas::set_screen_percentage(percentage);
                        SETTINGS.screen_percentage = percentage;
                        SETTINGS.store();
                    },
                    Result::Err(e) => {},
                }
            },
        }),
        UiElement::Button(UiButton {
            label: Text { text: "Back" },
            onclick: fn(label: Text) { leave_ui() },
        }),
    ))
}
