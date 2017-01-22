mod parser;

pub use self::parser::Frame;
pub use self::parser::parse_lines;

use gdb::Debugger;
use error::*;
use config::Inputs;
use consts;

pub struct Tas {
    dbg: Debugger,
}

impl Tas {
    pub fn new(pid: u32) -> Result<Tas> {
        let dbg = Debugger::start().chain_err(|| "Cannot start gdb")?;
        let mut this = Tas {
            dbg: dbg,
        };
        this.send_cmd(&format!("attach {}", pid))?;
        Ok(this)
    }

    fn send_cmd(&mut self, cmd: &str) -> Result<()> {
        self.dbg.send_cmd_raw(&cmd)
            .chain_err(|| "Cannot send_cmd").map(|_| ())
    }

    pub fn init(&mut self) -> Result<()> {
        let break_slate = format!("break *{:#}", consts::FSLATEAPPLICATION_TICK);
        self.send_cmd(&break_slate)?;
        self.send_cmd("call $slatetickbp = $bpnum")?;
        self.send_cmd("c")?;
        self.send_cmd("call $fslateapplication = $rdi")?;
        self.send_cmd("del $slatetickbp")?;
        let break_tick = format!("break *{:#}", consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
        self.send_cmd(&break_tick)?;
        self.send_cmd("call $tickbp = $bpnum")?;
        self.send_cmd(&format!("break *{:#}", consts::AMYCHARACTER_EXECFORCEDUNCROUCH))?;
        self.send_cmd("call $newgamebp = $bpnum")?;
        self.send_cmd("disable $newgamebp")?;
        self.send_cmd("disable $tickbp").map(|_| ())
    }

    pub fn step(&mut self) -> Result<()> {
        // set delta float for smooth fps
        self.send_cmd(&format!("set {{double}} {:#} = {}", consts::APP_DELTATIME, 1f64 / 60f64))?;
        self.send_cmd("c").map(|_| ())
    }

    pub fn press_key(&mut self, key: char) -> Result<()> {
        self.send_cmd(&format!("call ((void(*)(void*, int, int, int)){0})($fslateapplication,{1},{1},0)", consts::FSLATEAPPLICATION_ONKEYDOWN, key as u8)).map(|_| ())
    }

    pub fn release_key(&mut self, key: char) -> Result<()> {
        self.send_cmd(&format!("call ((void(*)(void*, int, int, int)){0})($fslateapplication,{1},{1},0)", consts::FSLATEAPPLICATION_ONKEYUP, key as u8)).map(|_| ())
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        self.send_cmd(&format!("call ((void(*)(void*, int, int)){})($fslateapplication,{},{})", consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE, x, y)).map(|_| ())
    }

    pub fn wait_for_new_game(&mut self) -> Result<()> {
        self.send_cmd("enable $newgamebp")?;
        self.send_cmd("c")?;
        self.send_cmd("disable $newgamebp").map(|_| ())
    }

    pub fn play(&mut self, frames: &Vec<Frame>, inputs: &Inputs) -> Result<()> {
        self.send_cmd("enable $tickbp")?;
        let mut last = Frame::default();
        for frame in frames {
            // new inputs
            if frame.forward && !last.forward {
                self.press_key(inputs.forward)?;
            }
            if frame.backward && !last.backward {
                self.press_key(inputs.backward)?;
            }
            if frame.left && !last.left {
                self.press_key(inputs.left)?;
            }
            if frame.right && !last.right {
                self.press_key(inputs.right)?;
            }
            if frame.jump && !last.jump {
                self.press_key(inputs.jump)?;
            }

            // old inputs
            if last.forward && !frame.forward {
                self.release_key(inputs.forward)?;
            }
            if last.backward && !frame.backward {
                self.release_key(inputs.backward)?;
            }
            if last.left && !frame.left {
                self.release_key(inputs.left)?;
            }
            if last.right && !frame.right {
                self.release_key(inputs.right)?;
            }
            if last.jump && !frame.jump {
                self.release_key(inputs.jump)?;
            }

            last = frame.clone();

            // press ESC
            if frame.esc {
                 self.press_key(0x1b as char)?;
            }

            // mouse movements
            if frame.mouse_x != 0 || frame.mouse_y != 0 {
                 self.move_mouse(frame.mouse_x, frame.mouse_y)?;
            }

            self.step()?;
        }
        self.send_cmd("disable $tickbp").map(|_| ())
    }
}

