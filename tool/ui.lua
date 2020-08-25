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
function ui.input(text, prefill)
  local old = saveglobals()

  local input = prefill or ""
  local stop_input = false
  local lshift_pressed = false
  local rshift_pressed = false
  local lctrl_pressed = false
  local rctrl_pressed = false

  _G.onkeydown = function(key, char, rep)
    print("onkeydown", key, char, rep)
    if key == KEYS.KEY_LEFT_SHIFT then
      lshift_pressed = true
    elseif key == KEYS.KEY_RIGHT_SHIFT then
      rshift_pressed = true
    elseif key == KEYS.KEY_LEFT_CTRL then
      lctrl_pressed = true
    elseif key == KEYS.KEY_RIGHT_CTRL then
      rctrl_pressed = true
    elseif key == KEYS.KEY_BACKSPACE then
      input = string.sub(input, 0, -2)
    elseif key == KEYS.KEY_ESCAPE then
      input = nil
      stop_input = true
    elseif key == KEYS.KEY_RETURN then
      stop_input = true
    elseif key == KEYS.KEY_V and (lctrl_pressed or rctrl_pressed) then
      input = input .. tas:get_clipboard()
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
    print("onkeyup", key, char, rep)
    if key == KEYS.KEY_LEFT_SHIFT then
      lshift_pressed = false
    elseif key == KEYS.KEY_RIGHT_SHIFT then
      rshift_pressed = false
    elseif key == KEYS.KEY_LEFT_CTRL then
      lctrl_pressed = true
    elseif key == KEYS.KEY_RIGHT_CTRL then
      rctrl_pressed = true
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

function ui.draw3dcapsule(x, y, z)
  z = z - 100
  local ax,ay,az = tas:project(x-50, y, z)
  local bx,by,bz = tas:project(x+50, y, z)
  local cx,cy,cz = tas:project(x-50, y, z+200)
  local dx,dy,dz = tas:project(x+50, y, z+200)

  local ex,ey,ez = tas:project(x, y-50, z)
  local fx,fy,fz = tas:project(x, y+50, z)
  local gx,gy,gz = tas:project(x, y-50, z+200)
  local hx,hy,hz = tas:project(x, y+50, z+200 )

  local function drawline(x1, y1, z1, x2, y2, z2)
    if z1 > 0 and z2 > 0 then
      tas:draw_line(x1, y1, x2, y2, 0, 0, 0, 1, 3)
    end
  end

  drawline(ax, ay, az, bx, by, bz)
  drawline(bx, by, bz, cx, cy, cz)
  drawline(cx, cy, cz, dx, dy, dz)
  drawline(dx, dy, dz, ax, ay, az)

  drawline(ex, ey, ez, fx, fy, fz)
  drawline(fx, fy, fz, gx, gy, gz)
  drawline(gx, gy, gz, hx, hy, hz)
  drawline(hx, hy, hz, ex, ey, ez)
end

return ui
