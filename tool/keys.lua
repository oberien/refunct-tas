local win = tas:is_windows()

-- Windows: https://docs.microsoft.com/en-us/windows/desktop/inputdev/virtual-key-codes
-- Linux (SDL): https://wiki.libsdl.org/SDLKeycodeLookup
-- On linux some MOD keys need the mask (1<<30) added to them when passed to
-- `tas:press_key`. The input arguments of `onkeydown` / `onkeyup` do not have
-- that mask though.
-- If a single number is given, `onkeydown/up` input is equal to `tas:press_key` input.
-- Otherwise the first element is received from `onkeydown/up` and the
-- second needs to be passed to `tas:press_key`.
local key_map = {
  KEY_BACKSPACE = 0x08,
  KEY_TAB = 0x09,
  KEY_RETURN = 0x0d,
  KEY_PAUSE = win and 0x13 or {72, 72 + (1<<30)},
  KEY_CAPS_LOCK = win and 0x14 or {57, 57 + (1<<30)},
  KEY_ESCAPE = 0x1b,
  KEY_SPACE = 0x20,
  KEY_PAGE_UP = win and 0x21 or {75, 75 + (1<<30)},
  KEY_PAGE_DOWN = win and 0x22 or {78, 78 + (1<<30)},
  KEY_END = win and 0x23 or {77, 77 + (1<<30)},
  KEY_HOME = win and 0x24 or {74, 74 + (1<<30)},
  KEY_LEFT = win and 0x25 or {80, 80 + (1<<30)},
  KEY_UP = win and 0x26 or {82, 82 + (1<<30)},
  KEY_RIGHT = win and 0x27 or {79, 79 + (1<<30)},
  KEY_DOWN = win and 0x28 or {81, 81 + (1<<30)},
  KEY_INSERT = win and 0x2d or {73, 73 + (1<<30)},
  KEY_DELETE = win and 0x2e or 127,
  KEY_0 = 0x30,
  KEY_1 = 0x31,
  KEY_2 = 0x32,
  KEY_3 = 0x33,
  KEY_4 = 0x34,
  KEY_5 = 0x35,
  KEY_6 = 0x36,
  KEY_7 = 0x37,
  KEY_8 = 0x38,
  KEY_9 = 0x39,
  KEY_A = win and 0x41 or 97,
  KEY_B = win and 0x42 or 98,
  KEY_C = win and 0x43 or 99,
  KEY_D = win and 0x44 or 100,
  KEY_E = win and 0x45 or 101,
  KEY_F = win and 0x46 or 102,
  KEY_G = win and 0x47 or 103,
  KEY_H = win and 0x48 or 104,
  KEY_I = win and 0x49 or 105,
  KEY_J = win and 0x4a or 106,
  KEY_K = win and 0x4b or 107,
  KEY_L = win and 0x4c or 108,
  KEY_M = win and 0x4d or 109,
  KEY_N = win and 0x4e or 110,
  KEY_O = win and 0x4f or 111,
  KEY_P = win and 0x50 or 112,
  KEY_Q = win and 0x51 or 113,
  KEY_R = win and 0x52 or 114,
  KEY_S = win and 0x53 or 115,
  KEY_T = win and 0x54 or 116,
  KEY_U = win and 0x55 or 117,
  KEY_V = win and 0x56 or 118,
  KEY_W = win and 0x57 or 119,
  KEY_X = win and 0x58 or 120,
  KEY_Y = win and 0x59 or 121,
  KEY_Z = win and 0x5a or 122,
  KEY_LEFT_WIN = win and 0x5b or {227, 227 + (1<<30)},
  KEY_RIGHT_WIN = win and 0x5c or {231, 231 + (1<<30)},
  KEY_NUMPAD0 = win and 0x60 or {98, 98 + (1<<30)},
  KEY_NUMPAD1 = win and 0x61 or {89, 89 + (1<<30)},
  KEY_NUMPAD2 = win and 0x62 or {90, 90 + (1<<30)},
  KEY_NUMPAD3 = win and 0x63 or {91, 91 + (1<<30)},
  KEY_NUMPAD4 = win and 0x64 or {92, 92 + (1<<30)},
  KEY_NUMPAD5 = win and 0x65 or {93, 93 + (1<<30)},
  KEY_NUMPAD6 = win and 0x66 or {94, 94 + (1<<30)},
  KEY_NUMPAD7 = win and 0x67 or {95, 95 + (1<<30)},
  KEY_NUMPAD8 = win and 0x68 or {96, 96 + (1<<30)},
  KEY_NUMPAD9 = win and 0x69 or {97, 97 + (1<<30)},
  KEY_NUMPAD_MULTIPLY = win and 0x6a or {85, 85 + (1<<30)},
  KEY_NUMPAD_ADD = win and 0x6b or {87, 87 + (1<<30)},
  KEY_NUMPAD_SUBTRACT = win and 0x6d or {86, 86 + (1<<30)},
  KEY_NUMPAD_DECIMAL = win and 0x6e or {99, 99 + (1<<30)},
  KEY_NUMPAD_DIVIDE = win and 0x6f or {84, 84 + (1<<30)},
  KEY_F1 = win and 0x70 or {58, 58 + (1<<30)},
  KEY_F2 = win and 0x71 or {59, 59 + (1<<30)},
  KEY_F3 = win and 0x72 or {60, 60 + (1<<30)},
  KEY_F4 = win and 0x73 or {61, 61 + (1<<30)},
  KEY_F5 = win and 0x74 or {62, 62 + (1<<30)},
  KEY_F6 = win and 0x75 or {63, 63 + (1<<30)},
  KEY_F7 = win and 0x76 or {64, 64 + (1<<30)},
  KEY_F8 = win and 0x77 or {65, 65 + (1<<30)},
  KEY_F9 = win and 0x78 or {66, 66 + (1<<30)},
  KEY_F10 = win and 0x79 or {67, 67 + (1<<30)},
  KEY_F11 = win and 0x7a or {68, 68 + (1<<30)},
  KEY_F12 = win and 0x7b or {69, 69 + (1<<30)},
  KEY_NUMLOCK = win and 0x90 or {83, 83 + (1<<30)},
  KEY_SCROLL_LOCK = win and 0x91 or {71, 71 + (1<<30)},
  KEY_LEFT_SHIFT = win and 0xa0 or {225, 225 + (1<<30)},
  KEY_RIGHT_SHIFT = win and 0xa1 or {229, 229 + (1<<30)},
  KEY_LEFT_CTRL = win and 0xa2 or {224, 224 + (1<<30)},
  KEY_RIGHT_CTRL = win and 0xa3 or {228, 228 + (1<<30)},
}

local keys = {}

for key,code in pairs(key_map) do
  if type(code) == "table" then
    keys[key] = code[1]
  else
    keys[key] = code
  end
end

--- Transforms a key received by `onkey{down,up}` into a key to be fed
--- into `tas:{press,release}_key` / `tas:key_{down,up}`.
function keys.keytoinput(key_code)
  for key,code in pairs(key_map) do
    if type(code) == "table" then
      if code[1] == key_code then
        return code[2]
      end
    elseif code == key_code then
      return code
    end
  end
  print("unknown key code ", key_code)
  return key_code
end

return keys