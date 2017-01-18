use gdb::Debugger;
use error::*;
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
        let break_tick = format!("break *{:#}", consts::FENGINELOOP_TICK);
        self.send_cmd(&break_tick).map(|_| ())
    }

    pub fn step(&mut self) -> Result<()> {
        self.send_cmd("c").map(|_| ())
    }

    pub fn press_key(&mut self, key: char) -> Result<()> {
        self.send_cmd(&format!("call {0}($fslateapplication,{1},{1},0)", consts::FSLATEAPPLICATION_ONKEYDOWN, key as u8)).map(|_| ())
    }

    pub fn release_key(&mut self, key: char) -> Result<()> {
        self.send_cmd(&format!("call {0}($fslateapplication,{1},{1},0)", consts::FSLATEAPPLICATION_ONKEYUP, key as u8)).map(|_| ())
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        self.send_cmd(&format!("call {}($fslateapplication,{},{})", consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE, x, y)).map(|_| ())
    }
}
