static mut CURRENT_PRACTICE = Practice {
    name: "none",
    button: 0,
    location: Location { x: 0., y: 0., z: 0. },
    rotation: Rotation { pitch: 0., yaw: 0., roll: 0. },
};

static PRACTICE_COMPONENT = Component {
    draw_hud: fn(text: string) -> string {
        f"{text}\nPracticing: {CURRENT_PRACTICE.name}"
    },
    tick_fn: Tas::step,
    on_tick: fn() {},
    on_yield: fn() {},
    on_new_game: fn() {
        let old_delta = Tas::get_delta();
        Tas::set_delta(Option::Some(1./2.));
        wait(9);
        Tas::set_rotation(CURRENT_PRACTICE.rotation);
        Tas::set_location(CURRENT_PRACTICE.location);
        Tas::set_velocity(Velocity { x: 0., y: 0., z: 0. });
        Tas::set_acceleration(Acceleration { x: 0., y: 0., z: 0. });
        Tas::set_delta(old_delta);
    },
    on_level_change: fn(old: int, new: int) {
        if new == 0 {
            Tas::set_level(CURRENT_PRACTICE.button);
        }
    },
    on_reset: fn(old: int, new: int) {
        Tas::set_level(CURRENT_PRACTICE.button);
    },
    on_platforms_change: fn(old: int, new: int) {},
    on_buttons_change: fn(old: int, new: int) {},
    on_key_down: fn(key: KeyCode, is_repeat: bool) {},
    on_key_up: fn(key: KeyCode) {},
    on_component_exit: fn() {},
};

struct Practice {
    name: string,
    button: int,
    location: Location,
    rotation: Rotation,
}

static PRACTICE_POINTS = List::of(
    Practice { name: "Dive Skip", button: 8, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. }, location: Location { x: -1065., y: -3842., z: 464. } },
    Practice { name: "LoF & Spiral Skip", button: 18, rotation: Rotation { pitch: 0., yaw: 0., roll: 0. }, location: Location { x: -1065., y: -3842., z: 464. } },
    Practice { name: "Final Climb / Hdnoftr", button: 29, rotation: Rotation { pitch: 0., yaw:  247., roll:  0. }, location: Location { x: 4741., y: 2294., z: 588. } },
    Practice { name: "Ls Jump", button: 6, rotation: Rotation { pitch: 0., yaw:  180., roll:  0. }, location: Location { x: -4265., y: -2989., z: 90. } },
    Practice { name: "Pit", button: 10, rotation: Rotation { pitch: 0., yaw:  90., roll:  0. }, location: Location { x: 1859., y: -869., z: 89. } },
    Practice { name: "Pillars", button: 26, rotation: Rotation { pitch: 0., yaw:  256., roll:  0. }, location: Location { x: -847., y: 5589., z: 231. } },
    Practice { name: "5 Turn & 6 Elevator", button: 4, rotation: Rotation { pitch: 0., yaw:  180., roll:  0. }, location: Location { x: -4284., y: -806., z: 840. } },
    Practice { name: "16", button: 15, rotation: Rotation { pitch: 0., yaw:  200., roll:  0. }, location: Location { x: -752., y: 1513., z: 839. } },
    Practice { name: "21", button: 19, rotation: Rotation { pitch: 0., yaw:  35., roll:  0. }, location: Location { x: 4015., y: -2743., z: 589. } },
    Practice { name: "Button 2", button: 0, rotation: Rotation { pitch: 327.65, yaw: 135.33, roll: 0. }, location: Location { x: -1037.57, y: -955.68, z: 732.16 } },
    Practice { name: "Button 3", button: 1, rotation: Rotation { pitch: 344.88, yaw: 359.73, roll: 0. }, location: Location { x: -1904.85, y: -8.17, z: 857.28 } },
    Practice { name: "Button 4", button: 2, rotation: Rotation { pitch: 338.98, yaw: 187.46, roll: 0. }, location: Location { x: 2074.04, y: -260.32, z: 1107.16 } },
    Practice { name: "Button 5", button: 3, rotation: Rotation { pitch: 340.15, yaw: 179.73, roll: 0. }, location: Location { x: -2728.39, y: -837.92, z: 193.16 } },
    Practice { name: "Button 6", button: 4, rotation: Rotation { pitch: 337.15, yaw: 333.42, roll: 0. }, location: Location { x: -4891.93, y: -892.98, z: 857.16 } },
    Practice { name: "Button 7", button: 5, rotation: Rotation { pitch: 305.20, yaw: 204.11, roll: 0. }, location: Location { x: -3241.14, y: -2295.33, z: 1607.15 } },
    Practice { name: "Button 8", button: 6, rotation: Rotation { pitch: 340.17, yaw: 357.06, roll: 0. }, location: Location { x: -4663.23, y: -3636.14, z: 107.16 } },
    Practice { name: "Button 9", button: 7, rotation: Rotation { pitch: 339.01, yaw: 0.22, roll: 0. }, location: Location { x: -2827.18, y: -3767.32, z: 1607.25 } },
    Practice { name: "Button 10", button: 8, rotation: Rotation { pitch: 320.00, yaw: 89.51, roll: 0. }, location: Location { x: -648.95, y: -3328.46, z: 1607.16 } },
    Practice { name: "Button 11", button: 9, rotation: Rotation { pitch: 343.86, yaw: 92., roll: 0. }, location: Location { x: 1950., y: -2312.88, z: 232.16 } },
    Practice { name: "Button 12", button: 10, rotation: Rotation { pitch: 355.37, yaw: 90.26, roll: 0. }, location: Location { x: 1910.94, y: 859.68, z: 239.98 } },
    Practice { name: "Button 13", button: 11, rotation: Rotation { pitch: 344.65, yaw: 90.26, roll: 0. }, location: Location { x: 2382.06, y: -431.27, z: 107.16 } },
    Practice { name: "Button 14", button: 12, rotation: Rotation { pitch: 346.40, yaw: 169.93, roll: 0. }, location: Location { x: 607.75, y: 2504.56, z: 228.53 } },
    Practice { name: "Button 15", button: 13, rotation: Rotation { pitch: 0.90, yaw: 298.41, roll: 0. }, location: Location { x: -865.92, y: 2487.93, z: 232.35 } },
    Practice { name: "Button 16", button: 14, rotation: Rotation { pitch: 339.76, yaw: 301.56, roll: 0. }, location: Location { x: -465.44, y: 1604.85, z: 732.16 } },
    Practice { name: "Button 17", button: 15, rotation: Rotation { pitch: 328.47, yaw: 339.28, roll: 0. }, location: Location { x: -2652.97, y: 1453.47, z: 857.21 } },
    Practice { name: "Button 18", button: 16, rotation: Rotation { pitch: 325.51, yaw: 231.20, roll: 0. }, location: Location { x: -1895.30, y: 1134.02, z: 1107.64 } },
    Practice { name: "Button 19", button: 17, rotation: Rotation { pitch: 340.55, yaw: 357.75, roll: 0. }, location: Location { x: -4147.88, y: -4007.69, z: 1607.26 } },
    Practice { name: "Button 20", button: 18, rotation: Rotation { pitch: 334.01, yaw: 30.91, roll: 0. }, location: Location { x: 2026.23, y: -3783.01, z: 1232.17 } },
    Practice { name: "Button 21", button: 19, rotation: Rotation { pitch: 332.07, yaw: 232.09, roll: 0. }, location: Location { x: 4226., y: -2202.19, z: 1107.16 } },
    Practice { name: "Button 22", button: 20, rotation: Rotation { pitch: 355.22, yaw: 70.25, roll: 0. }, location: Location { x: 2737., y: -4020.95, z: 68.16 } },
    Practice { name: "Button 23", button: 21, rotation: Rotation { pitch: 352.48, yaw: 99.27, roll: 0. }, location: Location { x: 3034.37, y: -985.16, z: 232.30 } },
    Practice { name: "Button 24", button: 22, rotation: Rotation { pitch: 341.05, yaw: 140.52, roll: 0. }, location: Location { x: 2412.56, y: 2271.34, z: 607.15 } },
    Practice { name: "Button 25", button: 23, rotation: Rotation { pitch: 318.08, yaw: 358.64, roll: 0. }, location: Location { x: 492.67, y: 4725.55, z: 1355.44 } },
    Practice { name: "Button 26", button: 24, rotation: Rotation { pitch: 338.07, yaw: 135.37, roll: 0. }, location: Location { x: 4477.55, y: 4711.60, z: 232.16 } },
    Practice { name: "Button 27", button: 25, rotation: Rotation { pitch: 331.94, yaw: 257.52, roll: 0. }, location: Location { x: -883.96, y: 5552.63, z: 232.16 } },
    Practice { name: "Button 28", button: 26, rotation: Rotation { pitch: 344.67, yaw: 228.22, roll: 0. }, location: Location { x: -1411.66, y: 2970.87, z: 982.16 } },
    Practice { name: "Button 29", button: 27, rotation: Rotation { pitch: 342.90, yaw: 13.59, roll: 0. }, location: Location { x: -5176.97, y: -222.32, z: 1357.16 } },
    Practice { name: "Button 30", button: 28, rotation: Rotation { pitch: 345.10, yaw: 247.02, roll: 0. }, location: Location { x: 4846.33, y: 2449.16, z: 607.32 } },
    Practice { name: "Button 31", button: 29, rotation: Rotation { pitch: 350.36, yaw: 243.57, roll: 0. }, location: Location { x: 3740.51, y: -534.68, z: 318.16 } },
    Practice { name: "Button 32", button: 30, rotation: Rotation { pitch: 300.36, yaw: 107.72, roll: 0. }, location: Location { x: 2617.49, y: -2265.24, z: 1357.16 } },
);
