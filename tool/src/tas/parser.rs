//use std::io::{Lines, BufRead};
//use config::Infile;

#[derive(Clone, Debug, Default)]
pub struct Frame {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub esc: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

impl Frame {
    pub fn new(forward: bool, backward: bool, left: bool, right: bool, jump: bool, esc: bool, mouse_x: i32, mouse_y: i32) -> Frame {
        Frame {
            forward: forward,
            backward: backward,
            left: left,
            right: right,
            jump: jump,
            esc: esc,
            mouse_x: mouse_x,
            mouse_y: mouse_y,
        }
    }
}

//pub fn parse_lines<B: BufRead>(lines: Lines<B>, inputs: &Infile) -> Vec<Frame> {
    //let mut frames: Vec<Frame> = Vec::new();
    //for (i, l) in lines.enumerate() {
        //let mut l = l.expect(&format!("Line {}: Error reading line", i));
        //// parse `{42}` lines as "repeat last line 42 times"
        //if l.starts_with("{") {
            //// if the first line is {42}, print an error
            //assert!(frames.len() > 0, "Line {}: Cannot repeat nothing", i);
            //assert_eq!(l.pop(), Some('}'), "Missing closing `}}`");
            //let count: u32 = l[1..].parse().expect(&format!("Line {}: Cant convert {} to a number", i, &l[1..]));
            //let el: Frame = frames[frames.len()-1].clone();
            //for _ in 0..count {
                //frames.push(el.clone());
            //}
            //continue;
        //}

        //let mut frame = Frame::default();

        //// parse normal input lines
        //let mut split = l.split("|");
        //// expect keys in the beginning
        //let keys = split.next().expect(&format!("Line {}: empty line???", i));
        //for c in keys.chars() {
            //match c {
                //c if c == inputs.forward => frame.forward = true,
                //c if c == inputs.backward => frame.backward = true,
                //c if c == inputs.left => frame.left = true,
                //c if c == inputs.right => frame.right = true,
                //c if c == inputs.jump => frame.jump = true,
                //c => panic!("Line {}: Unknown key `{:?}`", i, c)
            //}
        //}

        //// check if the line contains mouse movements
        //if let Some(mouse) = split.next() {
            //let mut split = mouse.split(|c| c == ' ' || c == ':');
            //frame.mouse_x = split.next().map(|x| x.parse().expect(&format!("Line {}: cannot convert {} to number", i, x))).unwrap_or(0);
            //frame.mouse_y = split.next().map(|y| y.parse().expect(&format!("Line {}: cannot convert {} to number", i, y))).unwrap_or(0);
        //}

        //// check if ESC should be pressed
        //if let Some(esc) = split.next() {
            //assert_eq!(esc, "ESC", "Line {}: expected `ESC`", i);
            //frame.esc = true;
        //}
        //frames.push(frame);
    //}
    //frames
//}
