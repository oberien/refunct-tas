use std::rc::Rc;
use std::cell::RefCell;
use hlua::{self, Lua};

use tas::Tas;
use config::Config;

pub fn init_tas(lua: &mut Lua, outer: Rc<RefCell<Tas>>, config: Config) {
    let tas = outer.clone();
    lua.set("step", hlua::function0(move || {
        tas.borrow_mut().step().expect("error stepping");
    }));

    let tas = outer.clone();
    let cfg = config.clone();
    lua.set("presskey", hlua::function1(move |key: String| {
        let key = to_key(&key, &cfg);
        tas.borrow_mut().press_key(key).expect("error pressing key");
    }));

    let tas = outer.clone();
    lua.set("releasekey", hlua::function1(move |key: String| {
        let key = to_key(&key, &config);
        tas.borrow_mut().release_key(key).expect("error releasing key");
    }));

    let tas = outer.clone();
    lua.set("movemouse", hlua::function2(move |x: i32, y: i32| {
        tas.borrow_mut().move_mouse(x, y).expect("error moving mouse");
    }));
}

fn to_key(key: &str, cfg: &Config) -> char {
    match key {
        "forward" => cfg.forward,
        "backward" => cfg.backward,
        "left" => cfg.left,
        "right" => cfg.right,
        "jump" => cfg.jump,
        "crouch" => cfg.crouch as char,
        "menu" => cfg.menu as char,
        s => panic!("Unknown key {}", s)
    }
}
