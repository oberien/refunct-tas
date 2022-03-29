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
fn trigger_all_platforms() {
    let mut pawns = List::new();
    for platform in PLATFORMS {
        let mut x = platform.loc.x;
        let mut y = platform.loc.y;
        let z = platform.loc.z + platform.size.z;
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
        } else if x == 2625. && y == -2250. && z == 1250.2 {
            // platform 248 with last button on top; trigger platform but not button
            x = 2500.;
            y = -2375.;
        }
        let rot = Rotation { pitch: 0., yaw: 0., roll: 0. };
        let id = Tas::spawn_pawn(Location { x: 0., y: 0., z: 5000. }, rot);
        pawns.push(id);
        Tas::move_pawn(id, Location { x: x, y: y, z: z + 89.15 });
    }
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1./60.));
    wait(5);
    Tas::set_delta(delta);
    for id in pawns {
        Tas::destroy_pawn(id);
    }
}
fn teleport_exact(index: int) {
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1./60.));
    let b = BUTTONS.get(index).unwrap();
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
        let b = BUTTONS.get(i).unwrap();
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
        i = i + 1;
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
        i = i + 1;
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
    loc: Location,
    size: Size,
}
struct Size {
    x: float,
    y: float,
    z: float,
}

static PLATFORMS = List::of(
    // Level 0
    Platform { loc: Location { x: -1000., y: -1000., z: 125. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: -500., y: -875., z: -250. }, size: Size { x: 250., y: 375., z: 250. } },
    Platform { loc: Location { x: -1500., y: -1375., z: -250. }, size: Size { x: 250., y: 375., z: 375. } },
    Platform { loc: Location { x: -1750., y: -625., z: -125. }, size: Size { x: 500., y: 375., z: 125. } },
    Platform { loc: Location { x: -1000., y: -500., z: -75. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -1500., y: -2125., z: -125. }, size: Size { x: 250., y: 375., z: 375. } },
    Platform { loc: Location { x: -1000., y: -1625., z: 0. }, size: Size { x: 250., y: 375., z: 500. } },
    // Level 1
    Platform { loc: Location { x: -1000., y: 0., z: 0. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: -375., y: -250., z: -125. }, size: Size { x: 375., y: 250., z: 250. } },
    Platform { loc: Location { x: -2000., y: 0., z: 0. }, size: Size { x: 250., y: 250., z: 750. } },
    Platform { loc: Location { x: -2375., y: 0., z: 0. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: -2250., y: 500., z: 0. }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: -875., y: 500., z: 62.5 }, size: Size { x: 125., y: 250., z: 187.5 } },
    Platform { loc: Location { x: -1250., y: 500., z: -125. }, size: Size { x: 750., y: 500., z: 125. } },
    // Level 2
    Platform { loc: Location { x: 1500., y: -250., z: 250. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: 2125., y: -375., z: 250. }, size: Size { x: 125., y: 375., z: 750. } },
    Platform { loc: Location { x: 625., y: 0., z: -125. }, size: Size { x: 625., y: 1125., z: 125. } },
    Platform { loc: Location { x: 375., y: -750., z: 62.5 }, size: Size { x: 125., y: 250., z: 187.5 } },
    Platform { loc: Location { x: 750., y: -750., z: 0. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: 750., y: 250., z: 0. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: -250., y: 250., z: 0. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: 1375., y: 375., z: 62.5 }, size: Size { x: 125., y: 375., z: 187.5 } },
    Platform { loc: Location { x: 750., y: -1125., z: 62.5 }, size: Size { x: 250., y: 125., z: 187.5 } },
    // Level 3
    Platform { loc: Location { x: -2625., y: -625., z: 250. }, size: Size { x: 375., y: 125., z: 500. } },
    Platform { loc: Location { x: -2250., y: -1250., z: 250. }, size: Size { x: 250., y: 250., z: 750. } },
    Platform { loc: Location { x: -2875., y: -875., z: 437.5 }, size: Size { x: 125., y: 125., z: 187.5 } },
    Platform { loc: Location { x: -2750., y: -875., z: -125. }, size: Size { x: 500., y: 125., z: 250. } },
    Platform { loc: Location { x: -2575., y: -875., z: 437.5 }, size: Size { x: 125., y: 125., z: 187.5 } },
    Platform { loc: Location { x: -2750., y: -500., z: -175. }, size: Size { x: 500., y: 250., z: 125. } },
    Platform { loc: Location { x: -3000., y: -1375., z: -175. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: -2750., y: 125., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: -2750., y: -1250., z: 0. }, size: Size { x: 250., y: 250., z: 500. } },
    // Level 4
    Platform { loc: Location { x: -4875., y: -875., z: 250. }, size: Size { x: 250., y: 125., z: 500. } },
    Platform { loc: Location { x: -4250., y: -875., z: 0. }, size: Size { x: 500., y: 125., z: 125. } },
    Platform { loc: Location { x: -4375., y: -875., z: 500. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: -4375., y: -625., z: 125. }, size: Size { x: 375., y: 125., z: 500. } },
    Platform { loc: Location { x: -4750., y: -1250., z: 250. }, size: Size { x: 500., y: 250., z: 375. } },
    Platform { loc: Location { x: -3750., y: -875., z: -125. }, size: Size { x: 500., y: 500., z: 125. } },
    // Level 5
    Platform { loc: Location { x: -3250., y: -2250., z: 500. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: -2000., y: -2250., z: 750. }, size: Size { x: 250., y: 250., z: 1250. } },
    Platform { loc: Location { x: -3000., y: -2625., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: -2250., y: -2750., z: 0. }, size: Size { x: 500., y: 250., z: 125. } },
    Platform { loc: Location { x: -2250., y: -1750., z: -125. }, size: Size { x: 500., y: 250., z: 125. } },
    // Level 6
    Platform { loc: Location { x: -4625., y: -3000., z: -125. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: -4500., y: -3625., z: -125. }, size: Size { x: 500., y: 125., z: 125. } },
    Platform { loc: Location { x: -4750., y: -3375., z: 250. }, size: Size { x: 250., y: 125., z: 375. } },
    Platform { loc: Location { x: -4250., y: -3375., z: 125. }, size: Size { x: 250., y: 125., z: 375. } },
    // Level 7
    Platform { loc: Location { x: -3750., y: -3875., z: 0. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: -3750., y: -3250., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -2625., y: -4250., z: 0. }, size: Size { x: 375., y: 250., z: 250. } },
    Platform { loc: Location { x: -3250., y: -4625., z: -125. }, size: Size { x: 500., y: 125., z: 125. } },
    Platform { loc: Location { x: -3250., y: -4250., z: 0. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: -2000., y: -4000., z: -125. }, size: Size { x: 250., y: 500., z: 250. } },
    Platform { loc: Location { x: -1750., y: -3375., z: -125. }, size: Size { x: 500., y: 375., z: 125. } },
    Platform { loc: Location { x: -2625., y: -3250., z: -125. }, size: Size { x: 375., y: 250., z: 375. } },
    Platform { loc: Location { x: -3250., y: -3250., z: 0. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: -2375., y: -3750., z: 500. }, size: Size { x: 125., y: 250., z: 1500. } },
    Platform { loc: Location { x: -2750., y: -3750., z: 500. }, size: Size { x: 250., y: 250., z: 1000. } },
    // Level 8
    Platform { loc: Location { x: -500., y: -4000., z: 50. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: 0., y: -4125., z: 0. }, size: Size { x: 250., y: 375., z: 375. } },
    Platform { loc: Location { x: -1000., y: -4125., z: 0. }, size: Size { x: 250., y: 375., z: 375. } },
    Platform { loc: Location { x: -1000., y: -3500., z: -125. }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: -1500., y: -4000., z: 0. }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: -500., y: -3500., z: 500. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: 500., y: -4000., z: -62.5 }, size: Size { x: 250., y: 250., z: 187.5 } },
    Platform { loc: Location { x: 250., y: -3500., z: -125. }, size: Size { x: 500., y: 125., z: 125. } },
    Platform { loc: Location { x: -500., y: -4500., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: 500., y: -4500., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    // Level 9
    Platform { loc: Location { x: 125., y: -2375., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: -500., y: -2000., z: 500. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: 1875., y: -2375., z: -125. }, size: Size { x: 375., y: 250., z: 250. } },
    Platform { loc: Location { x: -125., y: -3000., z: 0. }, size: Size { x: 375., y: 250., z: 500. } },
    Platform { loc: Location { x: 0., y: -1500., z: 0. }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: 125., y: -2375., z: 500. }, size: Size { x: 125., y: 375., z: 125. } },
    Platform { loc: Location { x: 0., y: -1875., z: 0. }, size: Size { x: 250., y: 125., z: 500. } },
    Platform { loc: Location { x: -175., y: -2375., z: 500. }, size: Size { x: 125., y: 375., z: 125. } },
    Platform { loc: Location { x: -1000., y: -2250., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    // Level 10
    Platform { loc: Location { x: 1875., y: 375., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: 2000., y: -1625., z: -125. }, size: Size { x: 125., y: 250., z: 125. } },
    Platform { loc: Location { x: 1625., y: -1375., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 500., y: -1500., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: 1750., y: -750., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: 1125., y: -1625., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 1000., y: -2125., z: -125. }, size: Size { x: 250., y: 125., z: 125. } },
    Platform { loc: Location { x: 1750., y: 1000., z: 0. }, size: Size { x: 500., y: 250., z: 125. } },
    // Level 11
    Platform { loc: Location { x: 2375., y: -375., z: -187.5 }, size: Size { x: 125., y: 375., z: 187.5 } },
    Platform { loc: Location { x: 2625., y: -375., z: 250. }, size: Size { x: 125., y: 375., z: 500. } },
    Platform { loc: Location { x: 2375., y: -1000., z: 375. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: 2500., y: 250., z: 125. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: 2500., y: 750., z: -125. }, size: Size { x: 250., y: 250., z: 375. } },
    // Level 12
    Platform { loc: Location { x: 500., y: 2500., z: 0. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 375., y: 1625., z: 375. }, size: Size { x: 125., y: 500., z: 500. } },
    Platform { loc: Location { x: 875., y: 1875., z: 250. }, size: Size { x: 125., y: 250., z: 375. } },
    Platform { loc: Location { x: 875., y: 1375., z: 375. }, size: Size { x: 125., y: 250., z: 500. } },
    // Level 13
    Platform { loc: Location { x: 625., y: 2500., z: -125. }, size: Size { x: 500., y: 500., z: 125. } },
    Platform { loc: Location { x: -250., y: 2875., z: 375. }, size: Size { x: 375., y: 125., z: 500. } },
    Platform { loc: Location { x: -875., y: 2250., z: 0. }, size: Size { x: 250., y: 500., z: 125. } },
    // Level 14
    Platform { loc: Location { x: -250., y: 1500., z: -175. }, size: Size { x: 500., y: 500., z: 125. } },
    Platform { loc: Location { x: -250., y: 1500., z: 250. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: -1000., y: 1375., z: 0. }, size: Size { x: 250., y: 375., z: 750. } },
    Platform { loc: Location { x: -1375., y: 2000., z: -125. }, size: Size { x: 375., y: 500., z: 125. } },
    // Level 15
    Platform { loc: Location { x: -2750., y: 1500., z: 250. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: -3125., y: 1000., z: 250. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: -2750., y: 875., z: 0. }, size: Size { x: 250., y: 375., z: 125. } },
    // Level 16
    Platform { loc: Location { x: -1875., y: 1750., z: 250. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: -1875., y: 1125., z: 250. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: -2250., y: 1750., z: -125. }, size: Size { x: 250., y: 500., z: 125. } },
    Platform { loc: Location { x: -1875., y: 2375., z: 375. }, size: Size { x: 125., y: 125., z: 500. } },
    // Level 17
    Platform { loc: Location { x: -4250., y: -4000., z: 500. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: -5125., y: -1750., z: -500. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: -4500., y: -2250., z: 500. }, size: Size { x: 250., y: 250., z: 750. } },
    Platform { loc: Location { x: -4975., y: -1600., z: 375. }, size: Size { x: 375., y: 125., z: 125. } },
    Platform { loc: Location { x: -4975., y: -2025., z: 375. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: -5500., y: -1875., z: 125. }, size: Size { x: 250., y: 375., z: 250. } },
    Platform { loc: Location { x: -5000., y: -2500., z: 62.5 }, size: Size { x: 250., y: 250., z: 187.5 } },
    Platform { loc: Location { x: -3875., y: -2250., z: -125. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: -4425., y: -1600., z: 187.6 }, size: Size { x: 125., y: 125., z: 312.5 } },
    Platform { loc: Location { x: -4425., y: -1900., z: 187.5 }, size: Size { x: 125., y: 125., z: 312.5 } },
    Platform { loc: Location { x: -3750., y: -2750., z: -175. }, size: Size { x: 500., y: 250., z: 125. } },
    // Level 18
    Platform { loc: Location { x: 1250., y: -3000., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 1250., y: -4500., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 2000., y: -4625., z: -125. }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: 2000., y: -4250., z: 250. }, size: Size { x: 250., y: 125., z: 375. } },
    Platform { loc: Location { x: 2000., y: -3875., z: 375. }, size: Size { x: 250., y: 250., z: 750. } },
    // Level 19
    Platform { loc: Location { x: 4875., y: -1500., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: 4875., y: -1250., z: 125. }, size: Size { x: 125., y: 375., z: 500. } },
    Platform { loc: Location { x: 4750., y: -2125., z: 250. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: 4250., y: -2625., z: 125. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: 3750., y: -2125., z: 62.5 }, size: Size { x: 250., y: 250., z: 187.5 } },
    Platform { loc: Location { x: 4250., y: -1625., z: 0. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: 4250., y: -2125., z: 250. }, size: Size { x: 250., y: 250., z: 750. } },
    Platform { loc: Location { x: 3750., y: -3250., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 2375., y: -3125., z: 0. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 3250., y: -2875., z: -125. }, size: Size { x: 250., y: 250., z: 250. } },
    // Level 20
    Platform { loc: Location { x: 4375., y: -4625., z: 0. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 4500., y: -4000., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 4875., y: -3875., z: 0. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 3500., y: -4750., z: 62.5 }, size: Size { x: 250., y: 125., z: 187.5 } },
    Platform { loc: Location { x: 4125., y: -4250., z: 250. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: 3875., y: -4500., z: -125. }, size: Size { x: 125., y: 375., z: 125. } },
    Platform { loc: Location { x: 3500., y: -4000., z: 125. }, size: Size { x: 250., y: 375., z: 375. } },
    Platform { loc: Location { x: 2750.128, y: -4125., z: -125. }, size: Size { x: 500.1282, y: 750., z: 125. } },
    Platform { loc: Location { x: 4875., y: -2750., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: 4375., y: -3125., z: 0. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 4875., y: -3375., z: 250. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: 4375., y: -3500., z: 125. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: 2750., y: -3950., z: 375. }, size: Size { x: 500., y: 125., z: 250. } },
    Platform { loc: Location { x: 2750., y: -4250., z: 250. }, size: Size { x: 500., y: 125., z: 125. } },
    Platform { loc: Location { x: 4625., y: -2750., z: 375. }, size: Size { x: 125., y: 125., z: 500. } },
    // Level 21
    Platform { loc: Location { x: 3000., y: 625., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 3000., y: -1000., z: 0. }, size: Size { x: 250., y: 375., z: 125. } },
    // Level 22
    Platform { loc: Location { x: 2125., y: 1750., z: 250. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: 2750., y: 1250., z: 250. }, size: Size { x: 250., y: 250., z: 750. } },
    Platform { loc: Location { x: 2500., y: 2250., z: 0. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: 1750., y: 1500., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    // Level 23
    Platform { loc: Location { x: 375., y: 4625., z: 250. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: 1000., y: 3875., z: 0. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 1500., y: 3375., z: -125. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 2125., y: 2375., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: 375., y: 3875., z: 0. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: 375., y: 4625., z: -125. }, size: Size { x: 500., y: 500., z: 125. } },
    Platform { loc: Location { x: 3125., y: 2000., z: -125. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 2875., y: 2500., z: 0. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 750., y: 3375., z: -125. }, size: Size { x: 375., y: 250., z: 375. } },
    // Level 24
    Platform { loc: Location { x: 1875., y: 4625., z: 125. }, size: Size { x: 500., y: 250., z: 750. } },
    Platform { loc: Location { x: 1250., y: 4250., z: 0. }, size: Size { x: 375., y: 125., z: 500. } },
    Platform { loc: Location { x: 1625., y: 3875., z: 0. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: 1125., y: 4750., z: 250. }, size: Size { x: 250., y: 375., z: 500. } },
    Platform { loc: Location { x: 4250., y: 4625., z: 0. }, size: Size { x: 500., y: 500., z: 125. } },
    Platform { loc: Location { x: 2125., y: 4000., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 1875., y: 5125., z: -125. }, size: Size { x: 500., y: 250., z: 125. } },
    Platform { loc: Location { x: 2500., y: 4625., z: -175. }, size: Size { x: 250., y: 500., z: 125. } },
    // Level 25
    Platform { loc: Location { x: 1000., y: 5625., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 375., y: 5625., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 2875., y: 5625., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 2250., y: 5875., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 2500., y: 6375., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 1875., y: 6375., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 1375., y: 6500., z: 0. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 750., y: 6375., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -875., y: 5625., z: 0. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 125., y: 6125., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: -375., y: 5875., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 3125., y: 6125., z: 0. }, size: Size { x: 125., y: 125., z: 125. } },
    Platform { loc: Location { x: 3750., y: 5625., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: 1625., y: 5875., z: -125. }, size: Size { x: 125., y: 125., z: 125. } },
    // Level 26
    Platform { loc: Location { x: -1625., y: 4375., z: 375. }, size: Size { x: 250., y: 250., z: 500. } },
    Platform { loc: Location { x: -750., y: 4000., z: 0. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: -1125., y: 4750., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: -1375., y: 3750., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: -875., y: 3375., z: 0. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: -250., y: 3375., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: -375., y: 4750., z: 250. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: -1375., y: 3125., z: 375. }, size: Size { x: 250., y: 250., z: 500. } },
    // Level 27
    Platform { loc: Location { x: -3500., y: 375., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -4125., y: 500., z: 125. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: -4375., y: 1250., z: 375. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: -3750., y: 1125., z: 375. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: -4125., y: 1875., z: 250. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: -3125., y: 2375., z: 250. }, size: Size { x: 250., y: 125., z: 375. } },
    Platform { loc: Location { x: -3500., y: 1750., z: 250. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: -5250., y: -250., z: 500. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: -5250., y: 1125., z: 250. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: -4750., y: 125., z: 250. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: -5500., y: 375., z: 250. }, size: Size { x: 250., y: 125., z: 375. } },
    Platform { loc: Location { x: -4000., y: 0., z: 125. }, size: Size { x: 125., y: 125., z: 375. } },
    Platform { loc: Location { x: -5625., y: -750., z: 125. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: -4750., y: 750., z: 375. }, size: Size { x: 125., y: 125., z: 500. } },
    Platform { loc: Location { x: -4750., y: 750., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -5250., y: -250., z: -125.2 }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -3500., y: 1750., z: -250.2 }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: -4875., y: 1750., z: 500. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: -4875., y: 1750., z: -125.2 }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: -3500., y: 375., z: 375. }, size: Size { x: 125., y: 125., z: 500. } },
    // Level 28
    Platform { loc: Location { x: 3000., y: 4125., z: 750. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: 3500., y: 5125., z: 750. }, size: Size { x: 250., y: 250., z: 1000. } },
    Platform { loc: Location { x: 3500., y: 3625., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    Platform { loc: Location { x: 4875., y: 2500., z: 125. }, size: Size { x: 250., y: 250., z: 375. } },
    Platform { loc: Location { x: 5375., y: 3000., z: -125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4750., y: 3750., z: -125. }, size: Size { x: 125., y: 250., z: 125. } },
    Platform { loc: Location { x: 5250., y: 3375., z: -250. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 5625., y: 2500., z: -125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 5375., y: 1875., z: -125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 5000., y: 1750., z: 0. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4500., y: 2250., z: 125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4500., y: 1875., z: 0. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4125., y: 2375., z: 0. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 3750., y: 2500., z: -125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 3500., y: 2875., z: -250. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4875., y: 3000., z: -125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4375., y: 2750., z: -125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4250., y: 3125., z: -250. }, size: Size { x: 125., y: 125., z: 250. } },
    // Level 29
    Platform { loc: Location { x: 4750., y: -750., z: -125. }, size: Size { x: 500., y: 125., z: 125. } },
    Platform { loc: Location { x: 4500., y: 125., z: -125. }, size: Size { x: 250., y: 250., z: 125. } },
    Platform { loc: Location { x: 3625., y: -1250., z: -125. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 4125., y: -525., z: 250. }, size: Size { x: 125., y: 250., z: 500. } },
    Platform { loc: Location { x: 3750., y: -650., z: 500. }, size: Size { x: 250., y: 125., z: 125. } },
    Platform { loc: Location { x: 3875., y: 750., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: 3375., y: -275., z: 125. }, size: Size { x: 125., y: 500., z: 375. } },
    Platform { loc: Location { x: 3750., y: 25., z: 500. }, size: Size { x: 250., y: 500., z: 125. } },
    Platform { loc: Location { x: 4125., y: -25., z: 125. }, size: Size { x: 125., y: 250., z: 375. } },
    Platform { loc: Location { x: 4625., y: -1000., z: 125. }, size: Size { x: 125., y: 125., z: 250. } },
    Platform { loc: Location { x: 4750., y: -375., z: 125. }, size: Size { x: 250., y: 250., z: 750. } },
    Platform { loc: Location { x: 3750., y: -375., z: 0. }, size: Size { x: 250., y: 250., z: 250. } },
    Platform { loc: Location { x: 3874.8, y: 125., z: 0. }, size: Size { x: 375., y: 250., z: 125. } },
    Platform { loc: Location { x: 4875., y: 1000., z: -125. }, size: Size { x: 250., y: 375., z: 125. } },
    // Level 30
    Platform { loc: Location { x: 2125., y: -2750., z: 125. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: 2625., y: -2250., z: 500.2 }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: 2875., y: -2750., z: 250. }, size: Size { x: 125., y: 125., z: 750. } },
    Platform { loc: Location { x: 2625., y: -2250., z: -125. }, size: Size { x: 375., y: 375., z: 125. } },
    Platform { loc: Location { x: 1875., y: -3000., z: 125. }, size: Size { x: 125., y: 125., z: 500. } },
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
static BUTTONS = List::of(
    TpButton::Simple(ButtonLoc { loc: Location { x: -1000., y: -1000., z: 732. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -2000., y: 0., z: 857. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 2125., y: -250., z: 1107. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -2725., y: -875., z: 193. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -5000., y: -875., z: 857. }, frames: 6 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -3250., y: -2250., z: 1607. }, frames: 8 }),
    TpButton::Two(Location { x: -4625., y: -3000., z: 107. }, ButtonLoc { loc: Location { x: -4625., y: -3625., z: 107. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -2750., y: -3750., z: 1607. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -625., y: -3375., z: 1607. }, frames: 8 }),
    TpButton::Two(Location { x: -25., y: -2375., z: 107. }, ButtonLoc { loc: Location { x: 2000., y: -2375., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 1875., y: 975., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 2375., y: -500., z: 107. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 600., y: 2625., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -875., y: 2500., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -375., y: 1625., z: 732. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -2750., y: 1500., z: 857. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -1875., y: 1125., z: 1107. }, frames: 7 }),
    TpButton::Two(Location { x: -5125., y: -1750., z: 107. }, ButtonLoc { loc: Location { x: -4250., y: -4000., z: 1607. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 2000., y: -3875., z: 1232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 4250., y: -2125., z: 1107. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 2750., y: -4100., z: 68. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 3000., y: -1000., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 2500., y: 2250., z: 607. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 375., y: 4750., z: 1357. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 4512.5, y: 4625., z: 232. }, frames: 8 }),
    TpButton::Three(Location { x: 3125., y: 6120., z: 232. }, Location { x: 1375., y: 6500., z: 232. }, ButtonLoc { loc: Location { x: -875., y: 5625., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -1375., y: 3000., z: 982. }, frames: 6 }),
    TpButton::Two(Location { x: -4875., y: 1750., z: 1357. }, ButtonLoc { loc: Location { x: -5250., y: -250., z: 1357. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 4887.5, y: 2500., z: 607. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 3750., y: -500., z: 318. }, frames: 7 }),
    TpButton::Final(Location { x: 2625., y: -2250., z: 1357. }),
);
