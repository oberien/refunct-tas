struct Ui {
    name: Text,
    elements: List<UiElement>,
    on_key_down: Option<fn(KeyCode)>,
    on_draw: Option<fn()>,
    selected: int,
}
impl Ui {
    fn new(name: string, elements: List<UiElement>) -> Ui {
        Ui { name: Text { text: name }, elements: elements, on_key_down: Option::None, on_draw: Option::None, selected: 0 }
    }
}
enum UiElement {
    Button(UiButton),
    Input(Input),
    Slider(Slider),
    Chooser(Chooser),
}
struct UiButton {
    label: Text,
    onclick: fn(Text),
}
struct Input {
    label: Text,
    input: string,
    onclick: fn(string),
    onchange: fn(string),
}
struct Text {
    text: string,
}
struct Slider {
    label: Text,
    content: Text,
    onleft: fn(),
    onright: fn(),
}
struct Chooser {
    label: Text,
    selected: int,
    options: List<Text>,
    onchange: fn(int),
}

static mut UI_STACK: List<Ui> = List::new();
static mut LSHIFT_PRESSED = false;
static mut RSHIFT_PRESSED = false;
static mut LCTRL_PRESSED = false;
static mut RCTRL_PRESSED = false;

fn enter_ui(ui: Ui) {
    UI_STACK.push(ui);
}
fn leave_ui() {
    UI_STACK.pop();
}

fn on_key_down(key_code: int, character_code: int, is_repeat: bool) {
    let chr = if character_code >= 0x20 && character_code <= 0x7e {
        let chr = string::from_char(character_code);
        Option::Some(if LSHIFT_PRESSED || RSHIFT_PRESSED {
            chr.to_uppercase()
        } else {
            chr.to_lowercase()
        })
    } else {
        Option::None
    };
    let key = KeyCode::from_large(key_code);
    if key.to_small() == KEY_LEFT_SHIFT.to_small() {
        LSHIFT_PRESSED = true;
    } else if key.to_small() == KEY_RIGHT_SHIFT.to_small() {
        RSHIFT_PRESSED = true;
    } else if key.to_small() == KEY_LEFT_CTRL.to_small() {
        LCTRL_PRESSED = true;
    } else if key.to_small() == KEY_RIGHT_CTRL.to_small() {
        RCTRL_PRESSED = true;
    }
    match UI_STACK.last() {
        Option::Some(ui) => ui.onkey(key, chr),
        Option::None => (),
    }
    let on_key_down = CURRENT_COMPONENT.on_key_down;
    on_key_down(key, is_repeat);
}
fn on_key_up(key_code: int, character_code: int, is_repeat: bool) {
    let key = KeyCode::from_large(key_code);
    if key.to_small() == KEY_LEFT_SHIFT.to_small() {
        LSHIFT_PRESSED = false;
    } else if key.to_small() == KEY_RIGHT_SHIFT.to_small() {
        RSHIFT_PRESSED = false;
    } else if key.to_small() == KEY_LEFT_CTRL.to_small() {
        LCTRL_PRESSED = false;
    } else if key.to_small() == KEY_RIGHT_CTRL.to_small() {
        RCTRL_PRESSED = false;
    }
    let on_key_up = CURRENT_COMPONENT.on_key_up;
    on_key_up(key);
}
fn on_mouse_move(x: int, y: int) {
    let on_mouse_move = CURRENT_COMPONENT.on_mouse_move;
    on_mouse_move(x, y);
}
fn draw_hud() {
    match UI_STACK.last() {
        Option::Some(ui) => ui.draw(),
        Option::None => (),
    }
}

static COLOR_BLACK = Color { red: 0., green: 0., blue: 0., alpha: 1. };
static COLOR_RED = Color { red: 1., green: 0., blue: 0., alpha: 1. };

impl Ui {
    fn onclick(self) {
        match self.elements.get(self.selected) {
            Option::Some(element) => element.onclick(),
            Option::None => (),
        }
    }
    fn onkey(mut self, key: KeyCode, chr: Option<string>) {
        let f = self.on_key_down;
        match f {
            Option::Some(f) => f(key),
            Option::None => (),
        }

        if key.to_small() == KEY_RETURN.to_small() {
            self.onclick();
        } else if key.to_small() == KEY_DOWN.to_small() {
            self.selected = if self.selected == self.elements.len()-1 {
                0
            } else {
                self.selected + 1
            };
        } else if key.to_small() == KEY_UP.to_small() {
            self.selected = if self.selected == 0 {
                self.elements.len() - 1
            } else {
                self.selected - 1
            };
        }
        match self.elements.get(self.selected) {
            Option::Some(element) => element.onkey(key, chr),
            Option::None => (),
        }
    }
    fn draw(self) {
        match self.on_draw {
            Option::Some(f) => f(),
            Option::None => (),
        }
        Tas::draw_text(DrawText {
            text: self.name.text,
            color: COLOR_BLACK,
            x: 0.,
            y: 0.,
            scale: SETTINGS.ui_scale,
            scale_position: true,
        });
        let mut i = 0;
        for element in self.elements {
            let color = if self.selected == i { COLOR_RED } else { COLOR_BLACK };
            element.draw(12. + i.to_float() * 12., color);
            i = i + 1;
        }
    }
}

impl UiElement {
    fn onclick(self) {
        match self {
            UiElement::Button(button) => button.onclick(),
            UiElement::Input(input) => input.onclick(),
            UiElement::Slider(slider) => (),
            UiElement::Chooser(chooser) => (),
        }
    }
    fn onkey(self, key: KeyCode, chr: Option<string>) {
        match self {
            UiElement::Button(button) => (),
            UiElement::Input(input) => input.onkey(key, chr),
            UiElement::Slider(slider) => slider.onkey(key, chr),
            UiElement::Chooser(chooser) => chooser.onkey(key, chr),
        }
    }
    fn draw(self, y: float, color: Color) {
        match self {
            UiElement::Button(button) => button.draw(y, color),
            UiElement::Input(input) => input.draw(y, color),
            UiElement::Slider(slider) => slider.draw(y, color),
            UiElement::Chooser(chooser) => chooser.draw(y, color),
        }
    }
}

impl UiButton {
    fn onclick(self) {
        let f = self.onclick;
        f(self.label);
    }
    fn draw(self, y: float, color: Color) {
        Tas::draw_text(DrawText {
            text: f"    {self.label.text}",
            color: color,
            x: 0.,
            y: y,
            scale: SETTINGS.ui_scale,
            scale_position: true,
        })
    }
}
impl Input {
    fn onclick(self) {
        let f = self.onclick;
        f(self.input);
    }
    fn onkey(mut self, key: KeyCode, chr: Option<string>) {
        if key.to_small() == KEY_BACKSPACE.to_small() {
            self.input = self.input.slice(0, -1);
        } else if key.to_small() == KEY_V.to_small() && (LCTRL_PRESSED || RCTRL_PRESSED) {
            self.input = f"{self.input}{Tas::get_clipboard()}";
        } else {
            match chr {
                Option::Some(s) => self.input = f"{self.input}{s}",
                Option::None => (),
            }
        }
        let onchange = self.onchange;
        onchange(self.input);
    }
    fn draw(self, y: float, color: Color) {
        Tas::draw_text(DrawText {
            text: f"    {self.label.text}: {self.input}",
            color: color,
            x: 0.,
            y: y,
            scale: SETTINGS.ui_scale,
            scale_position: true,
        })
    }
}
impl Slider {
    fn onkey(self, key: KeyCode, chr: Option<string>) {
        if key.to_small() == KEY_LEFT.to_small() {
            let f = self.onleft;
            f();
        } else if key.to_small() == KEY_RIGHT.to_small() {
            let f = self.onright;
            f();
        }
    }
    fn draw(self, y: float, color: Color) {
        Tas::draw_text(DrawText {
            text: f"    {self.label.text}: < {self.content.text} >",
            color: color,
            x: 0.,
            y: y,
            scale: SETTINGS.ui_scale,
            scale_position: true,
        })
    }
}
impl Chooser {
    fn onkey(mut self, key: KeyCode, chr: Option<string>) {
        if key.to_small() == KEY_RIGHT.to_small() {
            self.selected = if self.selected  == self.options.len()-1 {
                0
            } else {
                self.selected + 1
            };
            let f = self.onchange;
            f(self.selected);
        } else if key.to_small() == KEY_LEFT.to_small() {
            self.selected = if self.selected == 0 {
                self.options.len() - 1
            } else {
                self.selected - 1
            };
            let f = self.onchange;
            f(self.selected);
        }
    }
    fn draw(self, y: float, color: Color) {
        Tas::draw_text(DrawText {
            text: f"    {self.label.text}: < {self.options.get(self.selected).unwrap().text} >",
            color: color,
            x: 0.,
            y: y,
            scale: SETTINGS.ui_scale,
            scale_position: true,
        })
    }
}

