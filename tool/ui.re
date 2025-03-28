struct Ui {
    name: Text,
    elements: List<UiElement>,
    on_draw: Option<fn()>,
    selected: int,
}
impl Ui {
    fn new(name: string, elements: List<UiElement>) -> Ui {
        Ui { name: Text { text: name }, elements: elements, on_draw: Option::None, selected: 0 }
    }
    fn new_with_selected(name: string, selected: int, elements: List<UiElement>) -> Ui {
        Ui { name: Text { text: name }, elements: elements, on_draw: Option::None, selected: selected }
    }

    fn new_filechooser(name: string, file_list: List<string>, onclick: fn(string)) -> Ui {
        let mut files = List::of(
            UiElement::Button(UiButton {
                label: Text { text: "Back" },
                onclick: fn(label: Text) { leave_ui() },
            }),
            UiElement::Input(Input {
                label: Text { text: "File name" },
                input: "",
                onclick: fn(input: string) {
                    if input.len_utf8() == 0 {
                        return;
                    }
                    onclick(input);
                },
                onchange: fn(input: string) {}
            }),
        );
        for file in file_list {
            files.push(UiElement::Button(UiButton {
                label: Text { text: file },
                onclick: fn(label: Text) {
                    onclick(label.text);
                },
            }));
        }
        Ui::new(name, files)
    }
}

enum UiElement {
    Button(UiButton),
    Input(Input),
    FloatInput(FloatInput),
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
struct FloatInput {
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
    if UI_STACK.len() > 1 {
        UI_STACK.pop();
    }
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
    if key.to_small() == KEY_M.to_small() && !TAS_STATE.step_frame_mode && UI_STACK.len() == 1 {
        enter_ui(create_base_menu());
    }
    match UI_STACK.last() {
        Option::Some(ui) => ui.onkey(key, chr),
        Option::None => (),
    }
    for comp in CURRENT_COMPONENTS {
        let on_key_down_always = comp.on_key_down_always;
        on_key_down_always(key, is_repeat);
        // don't trigger key events while in the menu
        if UI_STACK.len() == 1 {
            let on_key_down = comp.on_key_down;
            on_key_down(key, is_repeat);
        }
    }
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
    for comp in CURRENT_COMPONENTS {
        let on_key_up_always = comp.on_key_up_always;
        on_key_up_always(key);
        // don't trigger key events while in the menu
        if UI_STACK.len() == 1 {
            let on_key_up = comp.on_key_up;
            on_key_up(key);
        }
    }
}
fn on_mouse_move(x: int, y: int) {
    for comp in CURRENT_COMPONENTS {
        let on_mouse_move = comp.on_mouse_move;
        on_mouse_move(x, y);
    }
}
fn draw_hud() {
    for component in CURRENT_COMPONENTS {
        let draw_hud_always = component.draw_hud_always;
        draw_hud_always();
    }
    match UI_STACK.last() {
        Option::Some(ui) => ui.draw(),
        Option::None => (),
    }
    draw_log_messages();
}

fn on_resolution_change() {
    for component in CURRENT_COMPONENTS {
        let on_resolution_change = component.on_resolution_change;
        on_resolution_change();
    }
}

fn on_menu_open() {
    for component in CURRENT_COMPONENTS {
        let on_menu_open = component.on_menu_open;
        on_menu_open();
    }
    leave_ui();
    leave_ui();
    leave_ui();
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
        // This padding dictates how much space there will be between elements. Got no clue why it's done like this.
        let padding = 48.;
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
            element.draw(padding + i.to_float() * padding, color);
            i = i + 1;
        }
    }
}

impl UiElement {
    fn onclick(self) {
        match self {
            UiElement::Button(button) => button.onclick(),
            UiElement::Input(input) => input.onclick(),
            UiElement::FloatInput(input) => input.onclick(),
            UiElement::Slider(slider) => (),
            UiElement::Chooser(chooser) => (),
        }
    }
    fn onkey(self, key: KeyCode, chr: Option<string>) {
        match self {
            UiElement::Button(button) => (),
            UiElement::Input(input) => input.onkey(key, chr),
            UiElement::FloatInput(input) => input.onkey(key, chr),
            UiElement::Slider(slider) => slider.onkey(key, chr),
            UiElement::Chooser(chooser) => chooser.onkey(key, chr),
        }
    }
    fn draw(self, y: float, color: Color) {
        match self {
            UiElement::Button(button) => button.draw(y, color),
            UiElement::Input(input) => input.draw(y, color),
            UiElement::FloatInput(input) => input.draw(y, color),
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
        } else if key.to_small() == KEY_D.to_small() && (LCTRL_PRESSED || RCTRL_PRESSED) {
            self.input = "";
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
impl FloatInput {
    fn onclick(self) {
        let f = self.onclick;
        f(self.input);
    }
    fn onkey(mut self, key: KeyCode, chr: Option<string>) {
        let mut input = self.input;
        if key.to_small() == KEY_BACKSPACE.to_small() {
            input = self.input.slice(0, -1);
        } else if key.to_small() == KEY_V.to_small() && (LCTRL_PRESSED || RCTRL_PRESSED) {
            input = f"{self.input}{Tas::get_clipboard()}";
        } else if key.to_small() == KEY_D.to_small() && (LCTRL_PRESSED || RCTRL_PRESSED) {
            input = "";
        } else {
            match chr {
                Option::Some(s) => input = f"{self.input}{s}",
                Option::None => (),
            }
        }
        self.input = "";
        let mut i = 0;
        while i < input.len_utf8() {
            let chr = match input.slice(i, i+1) {
                "0" => "0",
                "1" => "1",
                "2" => "2",
                "3" => "3",
                "4" => "4",
                "5" => "5",
                "6" => "6",
                "7" => "7",
                "8" => "8",
                "9" => "9",
                "+" => "+",
                "-" => "-",
                "." => ".",
                _ => "",
            };
            self.input = f"{self.input}{chr}";
            i += 1;
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

