static mut wait_direction = 1;
fn wait(mut frames: int) {
    // mouse movement needed to update rendering viewport
    Tas::move_mouse(wait_direction, wait_direction);
    wait_direction = wait_direction * -1;
    // all keys pressed to disable user input
//    Tas::press_key(Key::Forward);
//    Tas::press_key(Key::Backward);
//    Tas::press_key(Key::Left);
//    Tas::press_key(Key::Right);
    while frames > 1 {
        Tas::step();
        frames = frames - 1;
    }
//    Tas::release_key(Key::Forward);
//    Tas::release_key(Key::Backward);
//    Tas::release_key(Key::Left);
//    Tas::release_key(Key::Right);
    Tas::step();
}
fn tp_to(loc: Location) {
    Tas::set_location(loc);
    Tas::set_velocity(Velocity { x: 0., y: 0., z: 0. });
    Tas::set_acceleration(Acceleration { x: 0., y: 0., z: 0. });
    // wait for change to register
    wait(3);
}

fn button(loc: Location, frames: int) {
    tp_to(loc);
    // wait for new platforms to rise
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1. / 2.));
    wait(frames);
    Tas::set_delta(delta);
}

fn teleport_all_cubes() {
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1./60.));
    for cube in CUBES {
        tp_to(cube);
    }
    Tas::set_delta(delta);
}
fn platform_pawn_spawn_location(platform: Platform) -> Location {
    let mut x = platform.loc.x;
    let mut y = platform.loc.y;
    let z = platform.loc.z + platform.size.z;

    // platforms interfering with platforms
    if x == -3750. && y == -875. && z == 0. {
        // platform 39 with pipe above the middle
        y = -1150.;
    } else if x == 625. && y == 2500. && z == 0. {
        // platform 96 with offset thick block in it
        x = 900.;
    } else if x == -250. && y == 1500. && z == -50. {
        // platform 99 with other thick block in the middle
        x = -625.;
    } else if x == 375. && y == 3875. && z == 375. {
        // platform 161 with spring in the middle
        x = 600.;
    } else if x == -4750. && y == 750. && z == 0. {
        // platform 210 with other tall block in the middle
        x = -4550.;
    }
    // platforms interfering with buttons
    if x == -1000. && y == -1000. && z == 625. {
        // Button 0 interferes with platform 0
        x = -800.;
    } else if x == -2000. && y == 0. && z == 750. {
        // Button 1 interferes with platform 9
        x = -1800.;
    } else if x == -2750. && y == -875. && z == 125. {
        // Button 3 interferes with platform 26
        x = -2400.;
    } else if x == -3250. && y == -2250. && z == 1500. {
        // Button 5 interferes with platform 38
        x = -3100.;
    } else if x == -4625. && y == -3000. && z == 0. {
        // Button 6 interferes with platform 43
        x = -4300.;
    } else if x == -2750. && y == -3750. && z == 1500. {
        // Button 8 interferes with platform 57
        x = -2950.;
    } else if x == -2750. && y == 1500. && z == 750. {
        // Button 17 interferes with platform 101
        x = -2950.;
    } else if x == -1875. && y == 1125. && z == 1000. {
        // Button 18 interferes with platform 105
        x = -2000.;
        y = 1250.;
    } else if x == -5125. && y == -1750. && z == 0. {
        // Button 19 interferes with platform 109
        x = -5000.;
        y = -1625.;
    } else if x == -4250. && y == -4000. && z == 1500. {
        // Button 20 interferes with platform 108
        x = -4450.;
    } else if x == 2000. && y == -3875. && z == 1125. {
        // Button 21 interferes with platform 123
        x = 2200.;
    } else if x == 4250. && y == -2125. && z == 1000. {
        // Button 22 interferes with platform 130
        x = 4450.;
    } else if x == 2750.128 && y == -4125. && z == 0. {
        // Button 23 interferes with platform 141
        y = -4600.;
    } else if x == 3000. && y == -1000. && z == 125. {
        // Button 24 interferes with platform 150
        x = 2800.;
    } else if x == 2500. && y == 2250. && z == 500. {
        // Button 25 interferes with platform 153
        x = 2700.;
    } else if x == 3125. && y == 6125. && z == 125. {
        // Button 28 interferes with platform 183
        x = 3250.;
        y = 6250.;
    } else if x == 1375. && y == 6500. && z == 125. {
        // Button 29 interferes with platform 178
        x = 1500.;
        y = 6625.;
    } else if x == -875. && y == 5625. && z == 125. {
        // Button 30 interferes with platform 180
        x = -750.;
        y = 5750.;
    } else if x == -4875. && y == 1750. && z == 1250. {
        // Button 32 interferes with platform 211
        x = -5000.;
        y = 1875.;
    } else if x == -5250. && y == -250. && z == 1250. {
        // Button 33 interferes with platform 201
        x = -5375.;
        y = -125.;
    } else if x == 4875. && y == 2500. && z == 500. {
        // Button 34 interferes with platform 217
        x = 4700.;
    } else if x == 2625. && y == -2250. && z == 1250.2 {
        // Button 36 interferes with platform 247
        x = 2500.;
        y = -2375.;
    }

    Location { x: x, y: y, z: z + 89.15 }
}
fn create_all_platform_pawns() -> List<int> {
    let mut pawns = List::new();
    for platform in PLATFORMS {
        let rot = Rotation { pitch: 0., yaw: 0., roll: 0. };
        let id = Tas::spawn_pawn(Location { x: 0., y: 0., z: 5000. }, rot);
        Tas::move_pawn(id, platform_pawn_spawn_location(platform));
        pawns.push(id);
    }
    pawns
}
fn create_all_button_pawns_up_to(up_to: int) -> List<int> {
    let mut pawns = List::new();
    let mut i = 0;
    while i < up_to {
        let button_loc = BUTTONS.get(i).unwrap().loc;
        let rot = Rotation { pitch: 0., yaw: 0., roll: 0. };
        let id = Tas::spawn_pawn(Location { x: 0., y: 0., z: 5000. }, rot);
        Tas::move_pawn(id, button_loc);
        pawns.push(id);
        i += 1;
    }
    pawns
}
fn teleport_exact(index: int) {
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1./60.));
    let b = BUTTONS_TELEPORT.get(index).unwrap();
    match b {
        TpButton::Simple(b) => button(b.loc, b.frames),
        TpButton::Two(first, b) => {
            tp_to(first);
            button(b.loc, b.frames);
        },
        TpButton::Three(first, second, b) => {
            tp_to(first);
            tp_to(second);
            button(b.loc, b.frames);
        },
        TpButton::Final(last) => {
            tp_to(last);
        }
    }
    Tas::set_delta(delta);
}
/// only possible if all buttons are already raised but not pressed
fn teleport_all_buttons_up_to(up_to: int) {
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1. / 2.));
    wait(9);
    Tas::set_delta(delta);

    let mut i = 0;
    while i < up_to {
        let b = BUTTONS_TELEPORT.get(i).unwrap();
        match b {
            TpButton::Simple(b) => {
                Tas::set_location(b.loc);
                wait(1);
            },
            TpButton::Two(first, b) => {
                Tas::set_location(first);
                wait(1);
                Tas::set_location(b.loc);
                wait(1);
            },
            TpButton::Three(first, second, b) => {
                Tas::set_location(first);
                wait(1);
                Tas::set_location(second);
                wait(1);
                Tas::set_location(b.loc);
                wait(1);
            },
            TpButton::Final(last) => {
                Tas::set_location(last);
                wait(1);
            },
        }
        i += 1;
    }
}
fn teleport_buttons(up_to: int) {
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1. / 2.));
    wait(9);
    Tas::set_delta(delta);

    let mut i = 0;
    while i < up_to {
        teleport_exact(i);
        i += 1;
    }
}


static CUBES = List::of(
    Location { x: -2250., y: -1250., z: 1089. },
    Location { x: -4800., y: -3375., z: 714. },
    Location { x: -3250., y: -4625., z: 90. },
    Location { x: -2375., y: -3750., z: 2090. },
    Location { x: -125., y: -3500., z: 90. },
    Location { x: -500., y: -2000., z: 1590. },
    Location { x: 2375., y: -1125., z: 965. },
    Location { x: 875., y: 1900., z: 714. },
    Location { x: -500., y: 2875., z: 964. },
    Location { x: -4500., y: -2225., z: 1339. },
    Location { x: 5000., y: -2625., z: 90. },
    Location { x: 4125., y: -4250., z: 840. },
    Location { x: 2750., y: 1250., z: 1089. },
    Location { x: -1625., y: 4375., z: 964. },
    Location { x: -5625., y: 375., z: 714. },
    Location { x: 3425., y: 5100., z: 1839. },
    Location { x: 5375., y: 1875., z: 214. },
    Location { x: 4750., y: -350., z: 964. },
);

struct Platform {
    cluster: int,
    loc: Location,
    size: PlatformSize,
}
struct PlatformSize {
    x: float,
    y: float,
    z: float,
}

static PLATFORMS = List::of(
    // Level 0, 7 platforms: 0 - 6
    Platform { cluster: 0, loc: Location { x: -1000., y: -1000., z: 125. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 0, loc: Location { x: -500., y: -875., z: -250. }, size: PlatformSize { x: 250., y: 375., z: 250. } },
    Platform { cluster: 0, loc: Location { x: -1500., y: -1375., z: -250. }, size: PlatformSize { x: 250., y: 375., z: 375. } },
    Platform { cluster: 0, loc: Location { x: -1750., y: -625., z: -125. }, size: PlatformSize { x: 500., y: 375., z: 125. } },
    Platform { cluster: 0, loc: Location { x: -1000., y: -500., z: -75. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 0, loc: Location { x: -1500., y: -2125., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 375. } },
    Platform { cluster: 0, loc: Location { x: -1000., y: -1625., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 500. } },
    // Level 1, 7 platforms: 7 - 13
    Platform { cluster: 1, loc: Location { x: -1000., y: 0., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 1, loc: Location { x: -375., y: -250., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 250. } },
    Platform { cluster: 1, loc: Location { x: -2000., y: 0., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    Platform { cluster: 1, loc: Location { x: -2375., y: 0., z: 0. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 1, loc: Location { x: -2250., y: 500., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 1, loc: Location { x: -875., y: 500., z: 62.5 }, size: PlatformSize { x: 125., y: 250., z: 187.5 } },
    Platform { cluster: 1, loc: Location { x: -1250., y: 500., z: -125. }, size: PlatformSize { x: 750., y: 500., z: 125. } },
    // Level 2, 9 platforms: 14 - 22
    Platform { cluster: 2, loc: Location { x: 1500., y: -250., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 2, loc: Location { x: 2125., y: -375., z: 250. }, size: PlatformSize { x: 125., y: 375., z: 750. } },
    Platform { cluster: 2, loc: Location { x: 625., y: 0., z: -125. }, size: PlatformSize { x: 625., y: 1125., z: 125. } },
    Platform { cluster: 2, loc: Location { x: 375., y: -750., z: 62.5 }, size: PlatformSize { x: 125., y: 250., z: 187.5 } },
    Platform { cluster: 2, loc: Location { x: 750., y: -750., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 2, loc: Location { x: 750., y: 250., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 2, loc: Location { x: -250., y: 250., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 2, loc: Location { x: 1375., y: 375., z: 62.5 }, size: PlatformSize { x: 125., y: 375., z: 187.5 } },
    Platform { cluster: 2, loc: Location { x: 750., y: -1125., z: 62.5 }, size: PlatformSize { x: 250., y: 125., z: 187.5 } },
    // Level 3, 9 platforms: 23 - 31
    Platform { cluster: 3, loc: Location { x: -2625., y: -625., z: 250. }, size: PlatformSize { x: 375., y: 125., z: 500. } },
    Platform { cluster: 3, loc: Location { x: -2250., y: -1250., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    Platform { cluster: 3, loc: Location { x: -2875., y: -875., z: 437.5 }, size: PlatformSize { x: 125., y: 125., z: 187.5 } },
    Platform { cluster: 3, loc: Location { x: -2750., y: -875., z: -125. }, size: PlatformSize { x: 500., y: 125., z: 250. } },
    Platform { cluster: 3, loc: Location { x: -2575., y: -875., z: 437.5 }, size: PlatformSize { x: 125., y: 125., z: 187.5 } },
    Platform { cluster: 3, loc: Location { x: -2750., y: -500., z: -175. }, size: PlatformSize { x: 500., y: 250., z: 125. } },
    Platform { cluster: 3, loc: Location { x: -3000., y: -1375., z: -175. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 3, loc: Location { x: -2750., y: 125., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 3, loc: Location { x: -2750., y: -1250., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    // Level 4, 6 platforms: 32 - 37
    Platform { cluster: 4, loc: Location { x: -4875., y: -875., z: 250. }, size: PlatformSize { x: 250., y: 125., z: 500. } },
    Platform { cluster: 4, loc: Location { x: -4250., y: -875., z: 0. }, size: PlatformSize { x: 500., y: 125., z: 125. } },
    Platform { cluster: 4, loc: Location { x: -4375., y: -875., z: 500. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 4, loc: Location { x: -4375., y: -625., z: 125. }, size: PlatformSize { x: 375., y: 125., z: 500. } },
    Platform { cluster: 4, loc: Location { x: -4750., y: -1250., z: 250. }, size: PlatformSize { x: 500., y: 250., z: 375. } },
    Platform { cluster: 4, loc: Location { x: -3750., y: -875., z: -125. }, size: PlatformSize { x: 500., y: 500., z: 125. } },
    // Level 5, 5 platforms: 38 - 42
    Platform { cluster: 5, loc: Location { x: -3250., y: -2250., z: 500. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 5, loc: Location { x: -2000., y: -2250., z: 750. }, size: PlatformSize { x: 250., y: 250., z: 1250. } },
    Platform { cluster: 5, loc: Location { x: -3000., y: -2625., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 5, loc: Location { x: -2250., y: -2750., z: 0. }, size: PlatformSize { x: 500., y: 250., z: 125. } },
    Platform { cluster: 5, loc: Location { x: -2250., y: -1750., z: -125. }, size: PlatformSize { x: 500., y: 250., z: 125. } },
    // Level 6, 4 platforms: 43 - 46
    Platform { cluster: 6, loc: Location { x: -4625., y: -3000., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 6, loc: Location { x: -4500., y: -3625., z: -125. }, size: PlatformSize { x: 500., y: 125., z: 125. } },
    Platform { cluster: 6, loc: Location { x: -4750., y: -3375., z: 250. }, size: PlatformSize { x: 250., y: 125., z: 375. } },
    Platform { cluster: 6, loc: Location { x: -4250., y: -3375., z: 125. }, size: PlatformSize { x: 250., y: 125., z: 375. } },
    // Level 7, 11 platforms: 47 - 57
    Platform { cluster: 7, loc: Location { x: -3750., y: -3875., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 7, loc: Location { x: -3750., y: -3250., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 7, loc: Location { x: -2625., y: -4250., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 250. } },
    Platform { cluster: 7, loc: Location { x: -3250., y: -4625., z: -125. }, size: PlatformSize { x: 500., y: 125., z: 125. } },
    Platform { cluster: 7, loc: Location { x: -3250., y: -4250., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 7, loc: Location { x: -2000., y: -4000., z: -125. }, size: PlatformSize { x: 250., y: 500., z: 250. } },
    Platform { cluster: 7, loc: Location { x: -1750., y: -3375., z: -125. }, size: PlatformSize { x: 500., y: 375., z: 125. } },
    Platform { cluster: 7, loc: Location { x: -2625., y: -3250., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 375. } },
    Platform { cluster: 7, loc: Location { x: -3250., y: -3250., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 7, loc: Location { x: -2375., y: -3750., z: 500. }, size: PlatformSize { x: 125., y: 250., z: 1500. } },
    Platform { cluster: 7, loc: Location { x: -2750., y: -3750., z: 500. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    // Level 8, 10 platforms: 58 - 67
    Platform { cluster: 8, loc: Location { x: -500., y: -4000., z: 50. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 8, loc: Location { x: 0., y: -4125., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 375. } },
    Platform { cluster: 8, loc: Location { x: -1000., y: -4125., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 375. } },
    Platform { cluster: 8, loc: Location { x: -1000., y: -3500., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 8, loc: Location { x: -1500., y: -4000., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 8, loc: Location { x: -500., y: -3500., z: 500. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 8, loc: Location { x: 500., y: -4000., z: -62.5 }, size: PlatformSize { x: 250., y: 250., z: 187.5 } },
    Platform { cluster: 8, loc: Location { x: 250., y: -3500., z: -125. }, size: PlatformSize { x: 500., y: 125., z: 125. } },
    Platform { cluster: 8, loc: Location { x: -500., y: -4500., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 8, loc: Location { x: 500., y: -4500., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    // Level 9, 9 platforms: 68 - 76
    Platform { cluster: 9, loc: Location { x: 125., y: -2375., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 9, loc: Location { x: -500., y: -2000., z: 500. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 9, loc: Location { x: 1875., y: -2375., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 250. } },
    Platform { cluster: 9, loc: Location { x: -125., y: -3000., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 500. } },
    Platform { cluster: 9, loc: Location { x: 0., y: -1500., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 9, loc: Location { x: 125., y: -2375., z: 500. }, size: PlatformSize { x: 125., y: 375., z: 125. } },
    Platform { cluster: 9, loc: Location { x: 0., y: -1875., z: 0. }, size: PlatformSize { x: 250., y: 125., z: 500. } },
    Platform { cluster: 9, loc: Location { x: -175., y: -2375., z: 500. }, size: PlatformSize { x: 125., y: 375., z: 125. } },
    Platform { cluster: 9, loc: Location { x: -1000., y: -2250., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    // Level 10, 8 platforms: 77 - 84
    Platform { cluster: 10, loc: Location { x: 1875., y: 375., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 2000., y: -1625., z: -125. }, size: PlatformSize { x: 125., y: 250., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 1625., y: -1375., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 500., y: -1500., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 1750., y: -750., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 1125., y: -1625., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 1000., y: -2125., z: -125. }, size: PlatformSize { x: 250., y: 125., z: 125. } },
    Platform { cluster: 10, loc: Location { x: 1750., y: 1000., z: 0. }, size: PlatformSize { x: 500., y: 250., z: 125. } },
    // Level 11, 5 platforms: 85 - 89
    Platform { cluster: 11, loc: Location { x: 2375., y: -375., z: -187.5 }, size: PlatformSize { x: 125., y: 375., z: 187.5 } },
    Platform { cluster: 11, loc: Location { x: 2625., y: -375., z: 250. }, size: PlatformSize { x: 125., y: 375., z: 500. } },
    Platform { cluster: 11, loc: Location { x: 2375., y: -1000., z: 375. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 11, loc: Location { x: 2500., y: 250., z: 125. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 11, loc: Location { x: 2500., y: 750., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    // Level 12, 4 platforms: 90 - 93
    Platform { cluster: 12, loc: Location { x: 500., y: 2500., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 12, loc: Location { x: 375., y: 1625., z: 375. }, size: PlatformSize { x: 125., y: 500., z: 500. } },
    Platform { cluster: 12, loc: Location { x: 875., y: 1875., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 375. } },
    Platform { cluster: 12, loc: Location { x: 875., y: 1375., z: 375. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    // Level 13, 3 platforms: 94 - 96
    Platform { cluster: 13, loc: Location { x: 625., y: 2500., z: -125. }, size: PlatformSize { x: 500., y: 500., z: 125. } },
    Platform { cluster: 13, loc: Location { x: -250., y: 2875., z: 375. }, size: PlatformSize { x: 375., y: 125., z: 500. } },
    Platform { cluster: 13, loc: Location { x: -875., y: 2250., z: 0. }, size: PlatformSize { x: 250., y: 500., z: 125. } },
    // Level 14, 4 platforms: 97 - 100
    Platform { cluster: 14, loc: Location { x: -250., y: 1500., z: -175. }, size: PlatformSize { x: 500., y: 500., z: 125. } },
    Platform { cluster: 14, loc: Location { x: -250., y: 1500., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 14, loc: Location { x: -1000., y: 1375., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 750. } },
    Platform { cluster: 14, loc: Location { x: -1375., y: 2000., z: -125. }, size: PlatformSize { x: 375., y: 500., z: 125. } },
    // Level 15, 3 platforms: 101 - 103
    Platform { cluster: 15, loc: Location { x: -2750., y: 1500., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 15, loc: Location { x: -3125., y: 1000., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 15, loc: Location { x: -2750., y: 875., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    // Level 16, 4 platforms: 104 - 107
    Platform { cluster: 16, loc: Location { x: -1875., y: 1750., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 16, loc: Location { x: -1875., y: 1125., z: 250. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 16, loc: Location { x: -2250., y: 1750., z: -125. }, size: PlatformSize { x: 250., y: 500., z: 125. } },
    Platform { cluster: 16, loc: Location { x: -1875., y: 2375., z: 375. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    // Level 17, 11 platforms: 108 - 118
    Platform { cluster: 17, loc: Location { x: -4250., y: -4000., z: 500. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 17, loc: Location { x: -5125., y: -1750., z: -500. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 17, loc: Location { x: -4500., y: -2250., z: 500. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    Platform { cluster: 17, loc: Location { x: -4975., y: -1600., z: 375. }, size: PlatformSize { x: 375., y: 125., z: 125. } },
    Platform { cluster: 17, loc: Location { x: -4975., y: -2025., z: 375. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 17, loc: Location { x: -5500., y: -1875., z: 125. }, size: PlatformSize { x: 250., y: 375., z: 250. } },
    Platform { cluster: 17, loc: Location { x: -5000., y: -2500., z: 62.5 }, size: PlatformSize { x: 250., y: 250., z: 187.5 } },
    Platform { cluster: 17, loc: Location { x: -3875., y: -2250., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 17, loc: Location { x: -4425., y: -1600., z: 187.6 }, size: PlatformSize { x: 125., y: 125., z: 312.5 } },
    Platform { cluster: 17, loc: Location { x: -4425., y: -1900., z: 187.5 }, size: PlatformSize { x: 125., y: 125., z: 312.5 } },
    Platform { cluster: 17, loc: Location { x: -3750., y: -2750., z: -175. }, size: PlatformSize { x: 500., y: 250., z: 125. } },
    // Level 18, 5 platforms: 119 - 123
    Platform { cluster: 18, loc: Location { x: 1250., y: -3000., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 18, loc: Location { x: 1250., y: -4500., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 18, loc: Location { x: 2000., y: -4625., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 18, loc: Location { x: 2000., y: -4250., z: 250. }, size: PlatformSize { x: 250., y: 125., z: 375. } },
    Platform { cluster: 18, loc: Location { x: 2000., y: -3875., z: 375. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    // Level 19, 10 platforms: 124 - 133
    Platform { cluster: 19, loc: Location { x: 4875., y: -1500., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 19, loc: Location { x: 4875., y: -1250., z: 125. }, size: PlatformSize { x: 125., y: 375., z: 500. } },
    Platform { cluster: 19, loc: Location { x: 4750., y: -2125., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 19, loc: Location { x: 4250., y: -2625., z: 125. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 19, loc: Location { x: 3750., y: -2125., z: 62.5 }, size: PlatformSize { x: 250., y: 250., z: 187.5 } },
    Platform { cluster: 19, loc: Location { x: 4250., y: -1625., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 19, loc: Location { x: 4250., y: -2125., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    Platform { cluster: 19, loc: Location { x: 3750., y: -3250., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 19, loc: Location { x: 2375., y: -3125., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 19, loc: Location { x: 3250., y: -2875., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    // Level 20, 15 platforms: 134 - 148
    Platform { cluster: 20, loc: Location { x: 4375., y: -4625., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 4500., y: -4000., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 4875., y: -3875., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 3500., y: -4750., z: 62.5 }, size: PlatformSize { x: 250., y: 125., z: 187.5 } },
    Platform { cluster: 20, loc: Location { x: 4125., y: -4250., z: 250. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 20, loc: Location { x: 3875., y: -4500., z: -125. }, size: PlatformSize { x: 125., y: 375., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 3500., y: -4000., z: 125. }, size: PlatformSize { x: 250., y: 375., z: 375. } },
    Platform { cluster: 20, loc: Location { x: 2750.128, y: -4125., z: -125. }, size: PlatformSize { x: 500.1282, y: 750., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 4875., y: -2750., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 4375., y: -3125., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 4875., y: -3375., z: 250. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 20, loc: Location { x: 4375., y: -3500., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 20, loc: Location { x: 2750., y: -3950., z: 375. }, size: PlatformSize { x: 500., y: 125., z: 250. } },
    Platform { cluster: 20, loc: Location { x: 2750., y: -4250., z: 250. }, size: PlatformSize { x: 500., y: 125., z: 125. } },
    Platform { cluster: 20, loc: Location { x: 4625., y: -2750., z: 375. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    // Level 21, 2 platforms: 149 - 150
    Platform { cluster: 21, loc: Location { x: 3000., y: 625., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 21, loc: Location { x: 3000., y: -1000., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    // Level 22, 4 platforms: 151 - 154
    Platform { cluster: 22, loc: Location { x: 2125., y: 1750., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 22, loc: Location { x: 2750., y: 1250., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    Platform { cluster: 22, loc: Location { x: 2500., y: 2250., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 22, loc: Location { x: 1750., y: 1500., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    // Level 23, 9 platforms: 155 - 163
    Platform { cluster: 23, loc: Location { x: 375., y: 4625., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 23, loc: Location { x: 1000., y: 3875., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 23, loc: Location { x: 1500., y: 3375., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 23, loc: Location { x: 2125., y: 2375., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 23, loc: Location { x: 375., y: 3875., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 23, loc: Location { x: 375., y: 4625., z: -125. }, size: PlatformSize { x: 500., y: 500., z: 125. } },
    Platform { cluster: 23, loc: Location { x: 3125., y: 2000., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 23, loc: Location { x: 2875., y: 2500., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 23, loc: Location { x: 750., y: 3375., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 375. } },
    // Level 24, 8 platforms: 164 - 171
    Platform { cluster: 24, loc: Location { x: 1875., y: 4625., z: 125. }, size: PlatformSize { x: 500., y: 250., z: 750. } },
    Platform { cluster: 24, loc: Location { x: 1250., y: 4250., z: 0. }, size: PlatformSize { x: 375., y: 125., z: 500. } },
    Platform { cluster: 24, loc: Location { x: 1625., y: 3875., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 24, loc: Location { x: 1125., y: 4750., z: 250. }, size: PlatformSize { x: 250., y: 375., z: 500. } },
    Platform { cluster: 24, loc: Location { x: 4250., y: 4625., z: 0. }, size: PlatformSize { x: 500., y: 500., z: 125. } },
    Platform { cluster: 24, loc: Location { x: 2125., y: 4000., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 24, loc: Location { x: 1875., y: 5125., z: -125. }, size: PlatformSize { x: 500., y: 250., z: 125. } },
    Platform { cluster: 24, loc: Location { x: 2500., y: 4625., z: -175. }, size: PlatformSize { x: 250., y: 500., z: 125. } },
    // Level 25, 14 platforms: 172 - 185
    Platform { cluster: 25, loc: Location { x: 1000., y: 5625., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 375., y: 5625., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 2875., y: 5625., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 2250., y: 5875., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 2500., y: 6375., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 1875., y: 6375., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 1375., y: 6500., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 750., y: 6375., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 25, loc: Location { x: -875., y: 5625., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 125., y: 6125., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: -375., y: 5875., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 3125., y: 6125., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 3750., y: 5625., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 25, loc: Location { x: 1625., y: 5875., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 125. } },
    // Level 26, 8 platforms: 186 - 193
    Platform { cluster: 26, loc: Location { x: -1625., y: 4375., z: 375. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    Platform { cluster: 26, loc: Location { x: -750., y: 4000., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 26, loc: Location { x: -1125., y: 4750., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 26, loc: Location { x: -1375., y: 3750., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 26, loc: Location { x: -875., y: 3375., z: 0. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 26, loc: Location { x: -250., y: 3375., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 26, loc: Location { x: -375., y: 4750., z: 250. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 26, loc: Location { x: -1375., y: 3125., z: 375. }, size: PlatformSize { x: 250., y: 250., z: 500. } },
    // Level 27, 20 platforms: 194 - 213
    Platform { cluster: 27, loc: Location { x: -3500., y: 375., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 27, loc: Location { x: -4125., y: 500., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 27, loc: Location { x: -4375., y: 1250., z: 375. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 27, loc: Location { x: -3750., y: 1125., z: 375. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 27, loc: Location { x: -4125., y: 1875., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 27, loc: Location { x: -3125., y: 2375., z: 250. }, size: PlatformSize { x: 250., y: 125., z: 375. } },
    Platform { cluster: 27, loc: Location { x: -3500., y: 1750., z: 250. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 27, loc: Location { x: -5250., y: -250., z: 500. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 27, loc: Location { x: -5250., y: 1125., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 27, loc: Location { x: -4750., y: 125., z: 250. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 27, loc: Location { x: -5500., y: 375., z: 250. }, size: PlatformSize { x: 250., y: 125., z: 375. } },
    Platform { cluster: 27, loc: Location { x: -4000., y: 0., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 375. } },
    Platform { cluster: 27, loc: Location { x: -5625., y: -750., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 27, loc: Location { x: -4750., y: 750., z: 375. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    Platform { cluster: 27, loc: Location { x: -4750., y: 750., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 27, loc: Location { x: -5250., y: -250., z: -125.2 }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 27, loc: Location { x: -3500., y: 1750., z: -250.2 }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 27, loc: Location { x: -4875., y: 1750., z: 500. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 27, loc: Location { x: -4875., y: 1750., z: -125.2 }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 27, loc: Location { x: -3500., y: 375., z: 375. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
    // Level 28, 18 platforms: 214 - 231
    Platform { cluster: 28, loc: Location { x: 3000., y: 4125., z: 750. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 28, loc: Location { x: 3500., y: 5125., z: 750. }, size: PlatformSize { x: 250., y: 250., z: 1000. } },
    Platform { cluster: 28, loc: Location { x: 3500., y: 3625., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    Platform { cluster: 28, loc: Location { x: 4875., y: 2500., z: 125. }, size: PlatformSize { x: 250., y: 250., z: 375. } },
    Platform { cluster: 28, loc: Location { x: 5375., y: 3000., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4750., y: 3750., z: -125. }, size: PlatformSize { x: 125., y: 250., z: 125. } },
    Platform { cluster: 28, loc: Location { x: 5250., y: 3375., z: -250. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 5625., y: 2500., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 5375., y: 1875., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 5000., y: 1750., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4500., y: 2250., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4500., y: 1875., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4125., y: 2375., z: 0. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 3750., y: 2500., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 3500., y: 2875., z: -250. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4875., y: 3000., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4375., y: 2750., z: -125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 28, loc: Location { x: 4250., y: 3125., z: -250. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    // Level 29, 14 platforms: 232 - 245
    Platform { cluster: 29, loc: Location { x: 4750., y: -750., z: -125. }, size: PlatformSize { x: 500., y: 125., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 4500., y: 125., z: -125. }, size: PlatformSize { x: 250., y: 250., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 3625., y: -1250., z: -125. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 4125., y: -525., z: 250. }, size: PlatformSize { x: 125., y: 250., z: 500. } },
    Platform { cluster: 29, loc: Location { x: 3750., y: -650., z: 500. }, size: PlatformSize { x: 250., y: 125., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 3875., y: 750., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 3375., y: -275., z: 125. }, size: PlatformSize { x: 125., y: 500., z: 375. } },
    Platform { cluster: 29, loc: Location { x: 3750., y: 25., z: 500. }, size: PlatformSize { x: 250., y: 500., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 4125., y: -25., z: 125. }, size: PlatformSize { x: 125., y: 250., z: 375. } },
    Platform { cluster: 29, loc: Location { x: 4625., y: -1000., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 250. } },
    Platform { cluster: 29, loc: Location { x: 4750., y: -375., z: 125. }, size: PlatformSize { x: 250., y: 250., z: 750. } },
    Platform { cluster: 29, loc: Location { x: 3750., y: -375., z: 0. }, size: PlatformSize { x: 250., y: 250., z: 250. } },
    Platform { cluster: 29, loc: Location { x: 3874.8, y: 125., z: 0. }, size: PlatformSize { x: 375., y: 250., z: 125. } },
    Platform { cluster: 29, loc: Location { x: 4875., y: 1000., z: -125. }, size: PlatformSize { x: 250., y: 375., z: 125. } },
    // Level 30, 5 platforms: 246 - 250
    Platform { cluster: 30, loc: Location { x: 2125., y: -2750., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 30, loc: Location { x: 2625., y: -2250., z: 500.2 }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 30, loc: Location { x: 2875., y: -2750., z: 250. }, size: PlatformSize { x: 125., y: 125., z: 750. } },
    Platform { cluster: 30, loc: Location { x: 2625., y: -2250., z: -125. }, size: PlatformSize { x: 375., y: 375., z: 125. } },
    Platform { cluster: 30, loc: Location { x: 1875., y: -3000., z: 125. }, size: PlatformSize { x: 125., y: 125., z: 500. } },
);
static CLUSTER_DEPTHS = List::of(
    // 0 is irrelevant as it's always risen when the player starts moving
    0.,
    1000., 1250., 1250., 1000., 2250., 875., 2375., 1875., 1875., 375.,
    1125., 1125., 1125., 1000., 1000., 1250., 1750., 1500., 1250., 1125.,
    375., 1250., 1500., 1125., 375., 1125., 1500., 2125., 1125., 1500.,
);

enum TpButton {
    Simple(ButtonLoc),
    Two(Location, ButtonLoc),
    Three(Location, Location, ButtonLoc),
    Final(Location),
}
struct ButtonLoc {
    loc: Location,
    frames: int,
}
struct Button {
    cluster: int,
    loc: Location,
}
static BUTTONS = List::of(
    Button { cluster: 0, loc: Location { x: -1000., y: -1000., z: 732. } },
    Button { cluster: 1, loc: Location { x: -2000., y: 0., z: 857. } },
    Button { cluster: 2, loc: Location { x: 2125., y: -250., z: 1107. } },
    Button { cluster: 3, loc: Location { x: -2725., y: -875., z: 193. } },
    Button { cluster: 4, loc: Location { x: -5000., y: -875., z: 857. } },
    Button { cluster: 5, loc: Location { x: -3250., y: -2250., z: 1607. } },
    Button { cluster: 6, loc: Location { x: -4625., y: -3000., z: 107. } },
    Button { cluster: 6, loc: Location { x: -4625., y: -3625., z: 107. } },
    Button { cluster: 7, loc: Location { x: -2750., y: -3750., z: 1607. } },
    Button { cluster: 8, loc: Location { x: -625., y: -3375., z: 1607. } },
    Button { cluster: 9, loc: Location { x: -25., y: -2375., z: 107. } },
    Button { cluster: 9, loc: Location { x: 2000., y: -2375., z: 232. } },
    Button { cluster: 10, loc: Location { x: 1875., y: 975., z: 232. } },
    Button { cluster: 11, loc: Location { x: 2375., y: -500., z: 107. } },
    Button { cluster: 12, loc: Location { x: 600., y: 2625., z: 232. } },
    Button { cluster: 13, loc: Location { x: -875., y: 2500., z: 232. } },
    Button { cluster: 14, loc: Location { x: -375., y: 1625., z: 732. } },
    Button { cluster: 15, loc: Location { x: -2750., y: 1500., z: 857. } },
    Button { cluster: 16, loc: Location { x: -1875., y: 1125., z: 1107. } },
    Button { cluster: 17, loc: Location { x: -5125., y: -1750., z: 107. } },
    Button { cluster: 17, loc: Location { x: -4250., y: -4000., z: 1607. } },
    Button { cluster: 18, loc: Location { x: 2000., y: -3875., z: 1232. } },
    Button { cluster: 19, loc: Location { x: 4250., y: -2125., z: 1107. } },
    Button { cluster: 20, loc: Location { x: 2750., y: -4100., z: 68. } },
    Button { cluster: 21, loc: Location { x: 3000., y: -1000., z: 232. } },
    Button { cluster: 22, loc: Location { x: 2500., y: 2250., z: 607. } },
    Button { cluster: 23, loc: Location { x: 375., y: 4750., z: 1357. } },
    Button { cluster: 24, loc: Location { x: 4512.5, y: 4625., z: 232. } },
    Button { cluster: 25, loc: Location { x: 3125., y: 6120., z: 232. } },
    Button { cluster: 25, loc: Location { x: 1375., y: 6500., z: 232. } },
    Button { cluster: 25, loc: Location { x: -875., y: 5625., z: 232. } },
    Button { cluster: 26, loc: Location { x: -1375., y: 3000., z: 982. } },
    Button { cluster: 27, loc: Location { x: -4875., y: 1750., z: 1357. } },
    Button { cluster: 27, loc: Location { x: -5250., y: -250., z: 1357. } },
    Button { cluster: 28, loc: Location { x: 4887.5, y: 2500., z: 607. } },
    Button { cluster: 29, loc: Location { x: 3750., y: -500., z: 318. } },
    Button { cluster: 30, loc: Location { x: 2625., y: -2250., z: 1357. } },
);
static BUTTONS_TELEPORT = List::of(
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(0).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(1).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(2).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(3).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(4).unwrap().loc, frames: 6 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(5).unwrap().loc, frames: 8 }),
    TpButton::Two(BUTTONS.get(6).unwrap().loc, ButtonLoc { loc: BUTTONS.get(7).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(8).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(9).unwrap().loc, frames: 8 }),
    TpButton::Two(BUTTONS.get(10).unwrap().loc, ButtonLoc { loc: BUTTONS.get(11).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(12).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(13).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(14).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(15).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(16).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(17).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(18).unwrap().loc, frames: 7 }),
    TpButton::Two(BUTTONS.get(19).unwrap().loc, ButtonLoc { loc: BUTTONS.get(20).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(21).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(22).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(23).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(24).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(25).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(26).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(27).unwrap().loc, frames: 8 }),
    TpButton::Three(BUTTONS.get(28).unwrap().loc, BUTTONS.get(29).unwrap().loc, ButtonLoc { loc: BUTTONS.get(30).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(31).unwrap().loc, frames: 6 }),
    TpButton::Two(BUTTONS.get(32).unwrap().loc, ButtonLoc { loc: BUTTONS.get(33).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(34).unwrap().loc, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: BUTTONS.get(35).unwrap().loc, frames: 7 }),
    TpButton::Final(BUTTONS.get(36).unwrap().loc),
);
