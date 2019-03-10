local KEYS = require "keys"

local ui = {}

ui.scale = 2

local function tochar(char)
  if char >= 0x20 and char <= 0x7e then
    return string.lower(string.char(char))
  else
    return nil
  end
end

local function saveglobals()
  local old_onkeydown = _G.onkeydown
  local old_onkeyup = _G.onkeyup
  local old_drawhud = _G.drawhud
  return {old_onkeydown, old_onkeyup, old_drawhud}
end

local function restoreglobals(saved)
  _G.onkeydown = saved[1]
  _G.onkeyup = saved[2]
  _G.drawhud = saved[3]
end

--- Asks the user for textual input, returning the input after the user presses Return.
--- If the user presses Escape, input is aborted and `nil` is returned.
function ui.input(text)
  local old = saveglobals()

  local input = ""
  local stop_input = false
  local lshift_pressed = false
  local rshift_pressed = false

  _G.onkeydown = function(key, char, rep)
    if key == KEYS.KEY_LEFT_SHIFT then
      lshift_pressed = true
    elseif key == KEYS.KEY_RIGHT_SHIFT then
      rshift_pressed = true
    elseif key == KEYS.KEY_BACKSPACE then
      input = string.sub(input, 0, -2)
    elseif key == KEYS.KEY_ESCAPE then
      input = nil
      stop_input = true
    elseif key == KEYS.KEY_RETURN then
      stop_input = true
    else
      char = tochar(char)
      if lshift_pressed or rshift_pressed then
        char = string.upper(char)
      end
      if char ~= nil then
        input = input .. char
      end
    end
  end

  _G.onkeyup = function(key, char, rep)
    if key == KEYS.KEY_LEFT_SHIFT then
      lshift_pressed = false
    elseif key == KEYS.KEY_RIGHT_SHIFT then
      rshift_pressed = false
    end
  end

  _G.drawhud = function()
    tas:draw_text(text .. ": " .. input, 0, 0, 0, 1, 0, 0, ui.scale, true)
  end

  while not stop_input do
    tas:step()
  end

  restoreglobals(old)
  return input
end

--- Lets the user select among one of the given options.
--- Returns the index of the selected item, or `nil` if Escape was pressed.
function ui.select(options)
  local old = saveglobals()

  local selected = 1
  local stop_selection = false

  _G.onkeydown = function(key, char, rep)
    if key == KEYS.KEY_RETURN then
      stop_selection = true
    elseif key == KEYS.KEY_ESCAPE then
      selected = nil
      stop_selection = true
    elseif key == KEYS.KEY_DOWN then
      selected = (selected % #options) + 1
    elseif key == KEYS.KEY_UP then
      selected = ((selected - 2) % #options) + 1
    end
  end

  _G.drawhud = function()
    for i,opt in ipairs(options) do
      local red = selected == i and 1 or 0
      local y = (i-1) * 10
      tas:draw_text(opt, red, 0, 0, 1, 0, y, ui.scale, true)
    end
  end

  while not stop_selection do
    tas:step()
  end

  restoreglobals(old)
  return selected
end

function ui.drawlines(lines)
  for i,opt in ipairs(lines) do
    local y = (i-1) * 10
    tas:draw_text(opt, 0, 0, 0, 1, 0, y, ui.scale, true)
  end
end

return ui
