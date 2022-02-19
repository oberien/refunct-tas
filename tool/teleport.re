static mut wait_direction = 1;
fn wait(mut frames: int) {
    // mouse movement needed to update rendering viewport
    Tas::move_mouse(wait_direction, wait_direction);
    wait_direction = wait_direction * -1;
    // all keys pressed to disable user input
    Tas::press_key(Key::Forward);
    Tas::press_key(Key::Backward);
    Tas::press_key(Key::Left);
    Tas::press_key(Key::Right);
    while frames > 1 {
        Tas::step();
        frames = frames - 1;
    }
    Tas::release_key(Key::Forward);
    Tas::release_key(Key::Backward);
    Tas::release_key(Key::Left);
    Tas::release_key(Key::Right);
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
fn teleport_all_platforms() {
    let delta = Tas::get_delta();
    Tas::set_delta(Option::Some(1./60.));
    for platform in PLATFORMS {
        tp_to(platform);
    }
    Tas::set_delta(delta);
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
static PLATFORMS = List::of(
// button 1
    Location { x: -1000., y: -500., z: 139. },
    Location { x: -1750., y: -600., z: 89. },
    Location { x: -1500., y: -1400., z: 214. },
    Location { x: -1400., y: -1900., z: 339. },
    Location { x: -1050., y: -1600., z: 589. },
// button 2
    Location { x: -500., y: -250., z: 214. },
    Location { x: -860., y: 360., z: 89. },
    Location { x: -850., y: 500., z: 339. },
    Location { x: -900., y: 100., z: 589. },
    Location { x: -2200., y: 400., z: 339. },
    Location { x: -2350., y: 50., z: 589. },
// button 3
    Location { x: -150., y: 250., z: 589. },
    Location { x: 750., y: 300., z: 589. },
    Location { x: 750., y: -750., z: 58. },
    Location { x: 350., y: -725., z: 339. },
    Location { x: 500., y: -250., z: 89. },
    Location { x: 1400., y: 325., z: 339. },
    Location { x: 1525., y: -250., z: 839. },
    Location { x: 750., y: -1125., z: 339. },
// button 4 (+cube)
    Location { x: -2750., y: -400., z: 39. },
    Location { x: -2750., y: -100., z: 89. },
    Location { x: -2550., y: -950., z: 714. },
    Location { x: -2900., y: -850., z: 714. },
    Location { x: -2650., y: -600., z: 839. },
    Location { x: -2750., y: -1200., z: 589. },
// button 5
    Location { x: -3100., y: -1250., z: 39. },
    Location { x: -3500., y: -1200., z: 89. },
    Location { x: -4000., y: -850., z: 214. },
    Location { x: -4150., y: -625., z: 714. },
    Location { x: -4300., y: -900., z: 839. },
    Location { x: -4600., y: -1250., z: 714. },
// button 6
    Location { x: -2450., y: -1700., z: 89. },
    Location { x: -3000., y: -2750., z: 89. },
    Location { x: -2350., y: -2700., z: 214. },
    Location { x: -2000., y: -2250., z: 2089. },
// button 7
    Location { x: -4275., y: -3400., z: 589. },
// button 8
    Location { x: -3750., y: -3875., z: 214. },
    Location { x: -3250., y: -4200., z: 464. },
    Location { x: -2650., y: -4200., z: 339. },
    Location { x: -2000., y: -4000., z: 214. },
    Location { x: -1550., y: -3400., z: 89. },
    Location { x: -2600., y: -3300., z: 339. },
    Location { x: -3250., y: -3250., z: 464. },
// button 9
    Location { x: -1400., y: -4000., z: 339. },
    Location { x: -1000., y: -4000., z: 464. },
    Location { x: -550., y: -4000., z: 514. },
    Location { x: -400., y: -4500., z: 89. },
    Location { x: 450., y: -4400., z: 89. },
    Location { x: 450., y: -4000., z: 214. },
    Location { x: 0., y: -4100., z: 464. },
    Location { x: -1000., y: -3500., z: 214. },
// button 10
    Location { x: -75., y: -3000., z: 589. },
    Location { x: -182., y: -2500., z: 714. },
    Location { x: 150., y: -2400., z: 714. },
    Location { x: -50., y: 1800., z: 589. },
    Location { x: 0., y: -1400., z: 339. },
    Location { x: -1100., y: -2175., z: 89. },
// button 11
    Location { x: 900., y: -2100., z: 89. },
    Location { x: 450., y: -1450., z: 89. },
    Location { x: 1150., y: -1600., z: 89. },
    Location { x: 1650., y: -1350., z: 89. },
    Location { x: 2025., y: -1650., z: 89. },
    Location { x: 1750., y: -750., z: 89. },
    Location { x: 1800., y: 500., z: 89. },
// button 12
    Location { x: 2500., y: 800., z: 339. },
    Location { x: 2500., y: 250., z: 589. },
    Location { x: 2650., y: -350., z: 839. },
// button 13
    Location { x: 350., y: 1500., z: 964. },
    Location { x: 925., y: 1400., z: 964. },
// button 14
    Location { x: 215., y: 2500., z: 89. },
// button 15
    Location { x: -1450., y: 2075., z: 89. },
    Location { x: -225., y: 1850., z: 39. },
    Location { x: -1000., y: 1350., z: 839. },
// button 16
    Location { x: -2800., y: 850., z: 214. },
    Location { x: -3125., y: 975., z: 839. },
// button 17
    Location { x: -2100., y: 1850., z: 89. },
    Location { x: -1800., y: 1750., z: 839. },
    Location { x: -1850., y: 2350., z: 964. },
// button 18
    Location { x: -4475., y: -1625., z: 589. },
    Location { x: -4450., y: -1900., z: 589. },
    Location { x: -5000., y: -2000., z: 589. },
    Location { x: -4950., y: -1600., z: 589. },
    Location { x: -5550., y: -1850., z: 464. },
    Location { x: -4950., y: -2550., z: 339. },
    Location { x: -3700., y: -2200., z: 89. },
    Location { x: -3675., y: -2750., z: 39. },
    Location { x: -3700., y: -3150., z: 89. },
// button 19
    Location { x: 1300., y: -2900., z: 89. },
    Location { x: 1250., y: -4500., z: 89. },
    Location { x: 2000., y: -4650., z: 214. },
    Location { x: 2025., y: -4200., z: 714. },
// button 20
    Location { x: 2400., y: -3100., z: 214. },
    Location { x: 3250., y: -2850., z: 214. },
    Location { x: 3800., y: -3200., z: 89. },
    Location { x: 3750., y: -2150., z: 339. },
    Location { x: 4300., y: -1600., z: 214. },
    Location { x: 4800., y: -1725., z: 89. },
    Location { x: 4300., y: -2600., z: 589. },
    Location { x: 4750., y: -2050., z: 839. },
    Location { x: 4850., y: -1250., z: 714. },
// button 21
    Location { x: 4625., y: -2775., z: 964. },
    Location { x: 4400., y: -3000., z: 214. },
    Location { x: 4900., y: -3425., z: 839. },
    Location { x: 4375., y: -3500., z: 714. },
    Location { x: 4600., y: -4000., z: 89. },
    Location { x: 4900., y: -3850., z: 214. },
    Location { x: 4400., y: -4650., z: 214. },
    Location { x: 3900., y: -4550., z: 89. },
    Location { x: 3500., y: -45750., z: 339. },
    Location { x: 3500., y: -4100., z: 589. },
    Location { x: 2950., y: -4000., z: 714. },
    Location { x: 2625., y: -4262., z: 464. },
// button 22
    Location { x: 3000., y: 500., z: 89. },
// button 23
    Location { x: 1750., y: 1500., z: 89. },
    Location { x: 2125., y: 1750., z: 839. },
// button 24
    Location { x: 1900., y: 2400., z: 89. },
    Location { x: 1500., y: 3400., z: 89. },
    Location { x: 950., y: 3850., z: 214. },
    Location { x: 750., y: 3350., z: 339. },
    Location { x: 725., y: 4550., z: 89. },
    Location { x: 550., y: 3900., z: 464. },
// button 25
    Location { x: 1150., y: 4750., z: 839. },
    Location { x: 1225., y: 4200., z: 589. },
    Location { x: 1700., y: 3850., z: 464. },
    Location { x: 2200., y: 4100., z: 89. },
    Location { x: 2500., y: 4600., z: 39. },
    Location { x: 2000., y: 5150., z: 89. },
    Location { x: 3000., y: 2500., z: 214. }, // don't know when this went up
    Location { x: 3150., y: 1950., z: 89. }, // don't know when this went up
    Location { x: 1800., y: 4600., z: 964. },
// button 26
    Location { x: 3700., y: 5600., z: 89. },
    Location { x: 2850., y: 5600., z: 89. },
    Location { x: 2500., y: 6400., z: 89. },
    Location { x: 2250., y: 5800., z: 89. },
    Location { x: 1800., y: 6350., z: 89. },
    Location { x: 1600., y: 5850., z: 89. },
    Location { x: 700., y: 6350., z: 89. },
    Location { x: 1000., y: 5575., z: 89. },
    Location { x: 350., y: 5600., z: 89. },
    Location { x: 100., y: 6100., z: 89. },
    Location { x: -350., y: 5850., z: 89. },
// button 27
    Location { x: -1100., y: 450., z: 89. },
    Location { x: -1500., y: 3800., z: 89. },
    Location { x: -800., y: 3300., z: 214. },
    Location { x: -250., y: 3350., z: 89. },
    Location { x: -800., y: 4000., z: 464. },
    Location { x: -450., y: 4700., z: 714. },
// button 28
    Location { x: -3075., y: 2375., z: 714. },
    Location { x: -4125., y: 1900., z: 839. },
    Location { x: -3475., y: 1750., z: 1089. },
    Location { x: -3750., y: -1075., z: 964. },
    Location { x: -3500., y: 375., z: 964. },
    Location { x: -4000., y: 0., z: 589. },
    Location { x: -4125., y: 500., z: 714. },
    Location { x: -4750., y: 750., z: 964. },
    Location { x: -5275., y: 1075., z: 839. },
    Location { x: -4750., y: 100., z: 1089. },
    Location { x: -5650., y: -750., z: 714. },
    Location { x: -5150., y: -400., z: 89. },
    Location { x: -4700., y: 550., z: 89. },
    Location { x: -4575., y: 1750., z: 89. },
    Location { x: -3500., y: 1550., z: 89. },
    Location { x: -3500., y: 550., z: 89. },
    Location { x: -4400., y: 1250., z: 1214. },
// button 29
    Location { x: 3500., y: 2900., z: 89. },
    Location { x: 3500., y: 3550., z: 89. },
    Location { x: 3050., y: 4150., z: 1839. },
    Location { x: 4275., y: 3125., z: 89. },
    Location { x: 4750., y: 3750., z: 89. },
    Location { x: 5250., y: 3350., z: 89. },
    Location { x: 5350., y: 3000., z: 214. },
    Location { x: 5625., y: 2500., z: 214. },
    Location { x: 5000., y: 1800., z: 339. },
    Location { x: 4450., y: 1900., z: 339. },
    Location { x: 4075., y: 2350., z: 339. },
    Location { x: 3750., y: 2500., z: 214. },
    Location { x: 4400., y: 2750., z: 214. },
    Location { x: 4900., y: 3000., z: 214. },
    Location { x: 4500., y: 2275., z: 464. },
// button 30
    Location { x: 4900., y: 1050., z: 89. },
    Location { x: 4500., y: 150., z: 89. },
    Location { x: 4500., y: -725., z: 89. },
    Location { x: 3750., y: -1250., z: 89. },
    Location { x: 4600., y: -1000., z: 404. },
    Location { x: 4075., y: -500., z: 839. },
    Location { x: 3700., y: -650., z: 714. },
    Location { x: 3750., y: 0., z: 714. },
    Location { x: 3300., y: -350., z: 589. },
    Location { x: 4150., y: 0., z: 589. },
    Location { x: 3800., y: 650., z: 89. },
    Location { x: 3775., y: 200., z: 214. },
// button 31
    Location { x: 2875., y: -2200., z: 89. },
    Location { x: 2900., y: -2750., z: 1089. },
    Location { x: 2125., y: -2725., z: 964. },
    Location { x: 1850., y: -3000., z: 714. },
// uncategorized
    Location { x: 0., y: -1900., z: 589. },
    Location { x: 725., y: -750., z: 589. },
    Location { x: -1100., y: 4800., z: 89. },
    Location { x: -3750., y: 1150., z: 964. },
    Location { x: -4675., y: 1750., z: 89. },
    Location { x: 3500., y: -4750., z: 339. },
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
    TpButton::Simple(ButtonLoc { loc: Location { x: -3250., y: -2250., z: 1800. }, frames: 8 }),
    TpButton::Two(Location { x: -4625., y: -3000., z: 107. }, ButtonLoc { loc: Location { x: -4625., y: -3625., z: 107. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -2750., y: -3750., z: 1607. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -625., y: -3375., z: 1607. }, frames: 10 }),
    TpButton::Two(Location { x: 0., y: -2375., z: 107. }, ButtonLoc { loc: Location { x: 2000., y: -2375., z: 232. }, frames: 8 }),
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
    TpButton::Simple(ButtonLoc { loc: Location { x: 4500., y: 4625., z: 232. }, frames: 8 }),
    TpButton::Three(Location { x: 3125., y: 6120., z: 232. }, Location { x: 1375., y: 6500., z: 232. }, ButtonLoc { loc: Location { x: -875., y: 5625., z: 232. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: -1375., y: 3000., z: 982. }, frames: 6 }),
    TpButton::Two(Location { x: -4875., y: 1750., z: 1357. }, ButtonLoc { loc: Location { x: -5250., y: -250., z: 1357. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 4888., y: 2500., z: 607. }, frames: 8 }),
    TpButton::Simple(ButtonLoc { loc: Location { x: 3750., y: -500., z: 318. }, frames: 7 }),
    TpButton::Final(Location { x: 2625., y: -2250., z: 1357. }),
);

