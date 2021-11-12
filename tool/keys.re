static win = Tas::is_windows();

// Windows: https://docs.microsoft.com/en-us/windows/desktop/inputdev/virtual-key-codes
// Linux (SDL): https://wiki.libsdl.org/SDLKeycodeLookup
// On linux some MOD keys need the mask (1<<30) added to them when passed to
// `tas:press_key`. The input arguments of `onkeydown` / `onkeyup` do not have
// that mask though.
// If a single number is given, `onkeydown/up` input is equal to `tas:press_key` input.
// Otherwise the first element is received from `onkeydown/up` and the
// second needs to be passed to `tas:press_key`.

// 1 << 30
static LARGE_KEY_MASK = 0x40000000;

struct KeyCode {
    large_value: int,
}
impl KeyCode {
    fn from_large(large: int) -> KeyCode {
        KeyCode { large_value: large }
    }
    fn to_small(self) -> int {
        if self.large_value > LARGE_KEY_MASK {
            self.large_value - LARGE_KEY_MASK
        } else {
            self.large_value
        }
    }
}

static KEY_BACKSPACE = KeyCode::from_large(0x08);
static KEY_TAB = KeyCode::from_large(0x09);
static KEY_RETURN = KeyCode::from_large(0x0d);
static KEY_PAUSE = KeyCode::from_large(if win { 0x13 } else { 72 + LARGE_KEY_MASK });
static KEY_CAPS_LOCK = KeyCode::from_large(if win { 0x14 } else { 57 + LARGE_KEY_MASK });
static KEY_ESCAPE = KeyCode::from_large(0x1b);
static KEY_SPACE = KeyCode::from_large(0x20);
static KEY_PAGE_UP = KeyCode::from_large(if win { 0x21 } else { 75 + LARGE_KEY_MASK });
static KEY_PAGE_DOWN = KeyCode::from_large(if win { 0x22 } else { 78 + LARGE_KEY_MASK });
static KEY_END = KeyCode::from_large(if win { 0x23 } else { 77 + LARGE_KEY_MASK });
static KEY_HOME = KeyCode::from_large(if win { 0x24 } else { 74 + LARGE_KEY_MASK });
static KEY_LEFT = KeyCode::from_large(if win { 0x25 } else { 80 + LARGE_KEY_MASK });
static KEY_UP = KeyCode::from_large(if win { 0x26 } else { 82 + LARGE_KEY_MASK });
static KEY_RIGHT = KeyCode::from_large(if win { 0x27 } else { 79 + LARGE_KEY_MASK });
static KEY_DOWN = KeyCode::from_large(if win { 0x28 } else { 81 + LARGE_KEY_MASK });
static KEY_INSERT = KeyCode::from_large(if win { 0x2d } else { 73 + LARGE_KEY_MASK });
static KEY_DELETE = KeyCode::from_large(if win { 0x2e } else { 127 });
static KEY_0 = KeyCode::from_large(0x30);
static KEY_1 = KeyCode::from_large(0x31);
static KEY_2 = KeyCode::from_large(0x32);
static KEY_3 = KeyCode::from_large(0x33);
static KEY_4 = KeyCode::from_large(0x34);
static KEY_5 = KeyCode::from_large(0x35);
static KEY_6 = KeyCode::from_large(0x36);
static KEY_7 = KeyCode::from_large(0x37);
static KEY_8 = KeyCode::from_large(0x38);
static KEY_9 = KeyCode::from_large(0x39);
static KEY_A = KeyCode::from_large(if win { 0x41 } else { 97 });
static KEY_B = KeyCode::from_large(if win { 0x42 } else { 98 });
static KEY_C = KeyCode::from_large(if win { 0x43 } else { 99 });
static KEY_D = KeyCode::from_large(if win { 0x44 } else { 100 });
static KEY_E = KeyCode::from_large(if win { 0x45 } else { 101 });
static KEY_F = KeyCode::from_large(if win { 0x46 } else { 102 });
static KEY_G = KeyCode::from_large(if win { 0x47 } else { 103 });
static KEY_H = KeyCode::from_large(if win { 0x48 } else { 104 });
static KEY_I = KeyCode::from_large(if win { 0x49 } else { 105 });
static KEY_J = KeyCode::from_large(if win { 0x4a } else { 106 });
static KEY_K = KeyCode::from_large(if win { 0x4b } else { 107 });
static KEY_L = KeyCode::from_large(if win { 0x4c } else { 108 });
static KEY_M = KeyCode::from_large(if win { 0x4d } else { 109 });
static KEY_N = KeyCode::from_large(if win { 0x4e } else { 110 });
static KEY_O = KeyCode::from_large(if win { 0x4f } else { 111 });
static KEY_P = KeyCode::from_large(if win { 0x50 } else { 112 });
static KEY_Q = KeyCode::from_large(if win { 0x51 } else { 113 });
static KEY_R = KeyCode::from_large(if win { 0x52 } else { 114 });
static KEY_S = KeyCode::from_large(if win { 0x53 } else { 115 });
static KEY_T = KeyCode::from_large(if win { 0x54 } else { 116 });
static KEY_U = KeyCode::from_large(if win { 0x55 } else { 117 });
static KEY_V = KeyCode::from_large(if win { 0x56 } else { 118 });
static KEY_W = KeyCode::from_large(if win { 0x57 } else { 119 });
static KEY_X = KeyCode::from_large(if win { 0x58 } else { 120 });
static KEY_Y = KeyCode::from_large(if win { 0x59 } else { 121 });
static KEY_Z = KeyCode::from_large(if win { 0x5a } else { 122 });
static KEY_LEFT_WIN = KeyCode::from_large(if win { 0x5b } else { 227 + LARGE_KEY_MASK });
static KEY_RIGHT_WIN = KeyCode::from_large(if win { 0x5c } else { 231 + LARGE_KEY_MASK });
static KEY_NUMPAD0 = KeyCode::from_large(if win { 0x60 } else { 98 + LARGE_KEY_MASK });
static KEY_NUMPAD1 = KeyCode::from_large(if win { 0x61 } else { 89 + LARGE_KEY_MASK });
static KEY_NUMPAD2 = KeyCode::from_large(if win { 0x62 } else { 90 + LARGE_KEY_MASK });
static KEY_NUMPAD3 = KeyCode::from_large(if win { 0x63 } else { 91 + LARGE_KEY_MASK });
static KEY_NUMPAD4 = KeyCode::from_large(if win { 0x64 } else { 92 + LARGE_KEY_MASK });
static KEY_NUMPAD5 = KeyCode::from_large(if win { 0x65 } else { 93 + LARGE_KEY_MASK });
static KEY_NUMPAD6 = KeyCode::from_large(if win { 0x66 } else { 94 + LARGE_KEY_MASK });
static KEY_NUMPAD7 = KeyCode::from_large(if win { 0x67 } else { 95 + LARGE_KEY_MASK });
static KEY_NUMPAD8 = KeyCode::from_large(if win { 0x68 } else { 96 + LARGE_KEY_MASK });
static KEY_NUMPAD9 = KeyCode::from_large(if win { 0x69 } else { 97 + LARGE_KEY_MASK });
static KEY_NUMPAD_MULTIPLY = KeyCode::from_large(if win { 0x6a } else { 85 + LARGE_KEY_MASK });
static KEY_NUMPAD_ADD = KeyCode::from_large(if win { 0x6b } else { 87 + LARGE_KEY_MASK });
static KEY_NUMPAD_SUBTRACT = KeyCode::from_large(if win { 0x6d } else { 86 + LARGE_KEY_MASK });
static KEY_NUMPAD_DECIMAL = KeyCode::from_large(if win { 0x6e } else { 99 + LARGE_KEY_MASK });
static KEY_NUMPAD_DIVIDE = KeyCode::from_large(if win { 0x6f } else { 84 + LARGE_KEY_MASK });
static KEY_F1 = KeyCode::from_large(if win { 0x70 } else { 58 + LARGE_KEY_MASK });
static KEY_F2 = KeyCode::from_large(if win { 0x71 } else { 59 + LARGE_KEY_MASK });
static KEY_F3 = KeyCode::from_large(if win { 0x72 } else { 60 + LARGE_KEY_MASK });
static KEY_F4 = KeyCode::from_large(if win { 0x73 } else { 61 + LARGE_KEY_MASK });
static KEY_F5 = KeyCode::from_large(if win { 0x74 } else { 62 + LARGE_KEY_MASK });
static KEY_F6 = KeyCode::from_large(if win { 0x75 } else { 63 + LARGE_KEY_MASK });
static KEY_F7 = KeyCode::from_large(if win { 0x76 } else { 64 + LARGE_KEY_MASK });
static KEY_F8 = KeyCode::from_large(if win { 0x77 } else { 65 + LARGE_KEY_MASK });
static KEY_F9 = KeyCode::from_large(if win { 0x78 } else { 66 + LARGE_KEY_MASK });
static KEY_F10 = KeyCode::from_large(if win { 0x79 } else { 67 + LARGE_KEY_MASK });
static KEY_F11 = KeyCode::from_large(if win { 0x7a } else { 68 + LARGE_KEY_MASK });
static KEY_F12 = KeyCode::from_large(if win { 0x7b } else { 69 + LARGE_KEY_MASK });
static KEY_NUMLOCK = KeyCode::from_large(if win { 0x90 } else { 83 + LARGE_KEY_MASK });
static KEY_SCROLL_LOCK = KeyCode::from_large(if win { 0x91 } else { 71 + LARGE_KEY_MASK });
static KEY_LEFT_SHIFT = KeyCode::from_large(if win { 0xa0 } else { 225 + LARGE_KEY_MASK });
static KEY_RIGHT_SHIFT = KeyCode::from_large(if win { 0xa1 } else { 229 + LARGE_KEY_MASK });
static KEY_LEFT_CTRL = KeyCode::from_large(if win { 0xa2 } else { 224 + LARGE_KEY_MASK });
static KEY_RIGHT_CTRL = KeyCode::from_large(if win { 0xa3 } else { 228 + LARGE_KEY_MASK });

