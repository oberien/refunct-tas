require "prelude"
require "teleportbutton-prelude"

local scale = 2
local lines = {}
local drawstats = false
local drawmenu = false
local drawpractice = false
local drawsettings = false
local practicefunction = nil
function drawline(s, r, g, b)
  table.insert(lines, {s or "", r or 0, g or 0, b or 0})
end

function drawoptions(opts, selection)
  for i, opt in pairs(opts) do
    if i == selection then
      drawline(opt, 1, 0, 0)
    else
      drawline(opt, 0, 0, 0)
    end
  end
end

local menu = {}
menu.selection = 1
menu.draw = function()
  opts = {
    "Practice",
    "Settings",
    "Back",
  }
  drawoptions(opts, menu.selection)
end
menu.onkeydown = function(key, char, rep)
  sel = menu.selection
  if key == 13 then
    -- enter
    drawmenu = false;
    menu.selection = 1;
    if sel == 1 then drawpractice = true
    elseif sel == 2 then drawsettings = true 
    end
  elseif key == 81 or key == 40 then
    -- down
    menu.selection = math.min(sel + 1, 3)
  elseif key == 82 or key == 38 then
    -- up
    menu.selection = math.max(sel - 1, 1)
  end
end
menu.onkeyup = function(key, char, rep)
end

local practice = {}
practice.selection = 1
practice.draw = function()
  function selected(val)
    return practicefunction == val and " (Selected)" or ""
  end
  opts = {
    "None" .. selected(nil) ,
	"Dive Skip" .. selected(dive),
    "LoF & Spiral Skip" .. selected(spiral),
	"Final Climb / 31" .. selected(finalclimb),
    "Back"
  }
  drawoptions(opts, practice.selection)
end
practice.onkeydown = function(key, char, rep)
  sel = practice.selection
  if key == 13 then
    -- enter
    drawpractice = false
    practice.selection = 1
    if sel == 1 then practicefunction = nil
    elseif sel == 2 then practicefunction = dive
	elseif sel == 3 then practicefunction = spiral
	elseif sel == 4 then practicefunction = finalclimb
    elseif sel == 5 then drawmenu = true 
    end
  elseif key == 81 or key == 40 then
    -- down
    practice.selection = math.min(sel + 1, 5)
  elseif key == 82 or key == 38 then
    -- up
    practice.selection = math.max(sel - 1, 1)
  end
end
practice.onkeyup = function(key, char, rep)
end

local settings = {}
settings.selection = 1
settings.draw = function()
  opts = {
    "Font Scale < " .. scale .. " >",
    "Show Stats   " .. (drawstats and "On" or "Off"),
    "Back",
  }
  drawoptions(opts, settings.selection)
end
settings.onkeydown = function(key, char, rep)
  sel = settings.selection
  if key == 13 then
    -- enter
    if sel == 2 then drawstats = not drawstats
    elseif sel == 3 then drawsettings = false; settings.selection = 1; drawmenu = true
    end
  elseif key == 79 or key == 39 then
    -- right
    if sel == 1 then scale = scale + 1
    elseif sel == 2 then drawstats = not drawstats
    end
  elseif key == 80 or key == 37 then
    -- left
    if sel == 1 then scale = math.max(1, scale - 1)
    elseif sel == 2 then drawstats = not drawstats
    end
  elseif key == 81 or key == 40 then
    -- down
    settings.selection = math.min(sel + 1, 3)
  elseif key == 82 or key == 38 then
    -- up
    settings.selection = math.max(sel - 1, 1)
  end
end
settings.onkeyup = function(key, char, rep)
end

local stats = {}
stats.draw = function()
  local x,y,z = getlocation()
  local velx, vely, velz = getvelocity()
  local pitch, yaw, roll = getrotation()
  local accx, accy, accz = getacceleration()
  drawline(string.format("x: %6.2f    y: %6.2f    z: %6.2f", x, y, z))
  drawline(string.format("velx: %6.2f    vely: %6.2f    velz: %6.2f", velx, vely, velz))
  drawline(string.format("velxy: %6.2f", math.sqrt(velx*velx + vely*vely)))
  drawline(string.format("velxyz: %6.2f", math.sqrt(velx*velx + vely*vely + velz*velz)))
  drawline(string.format("pitch: %6.2f    yaw: %6.2f    roll: %6.2f", pitch, yaw, roll))
end

function tochar(char)
  if char >= 0x20 and char <= 0x7e then
    return string.lower(string.char(char))
  else
    return nil
  end
end
drawhud = function()
  lines = {}
  
  if drawmenu then
    menu.draw()
  elseif drawpractice then
    practice.draw()
  elseif drawsettings then
    settings.draw()
  elseif drawstats then
    stats.draw()
  end

  local y = 0
  for k, v in pairs(lines) do
    s, r, g, b = v[1], v[2], v[3], v[4]
    tas:draw_text(s, r, g, b, 1, 0, y, scale, true)
    y = y + 10
  end
end
onkeydown = function(key, char, rep)
  char = tochar(char)
  if char == "m" then
    if drawmenu or drawpractice or drawsettings then
      drawmenu = false
      menu.selection = 1
      drawpractice = false
      menu.selection = 1
      drawsettings = false
      settings.selection = 1
    else
      drawmenu = true
    end
    return
  end

  if drawmenu then
    menu.onkeydown(key, char, rep)
  elseif drawpractice then
    practice.onkeydown(key, char, rep)
  elseif drawsettings then
    settings.onkeydown(key, char, rep)
  end
end
onkeyup = function(key, char, rep)
  char = tochar(char)
  if drawmenu then
    menu.onkeyup(key, char, rep)
  elseif drawpractice then
    practice.onkeyup(key, char, rep)
  elseif drawsettings then
    settings.onkeyup(key, char, rep)
  end
end

function spiral()
  setdelta(1/60)
  teleportbutton(19)
  setrotation(0, 0, 0)
  setlocation(-1065, -3842, 464)
  setdelta(0)
end
function dive()
  setdelta(1/60)
  teleportbutton(8)
  setdelta(1/2)
  wait(5)
  setrotation(0, 0, 0)
  setlocation(-1065, -3842, 464)
  wait(5)
  setdelta(0)
end
function finalclimb()
  setdelta(1/60)
  teleportbutton(31)
  setrotation(0, 240, 0)
  setlocation(3535, -1000, 89)
  setdelta(0)
end

while true do
  waitfornewgame()
  if practicefunction ~= nil then
    practicefunction()
  end
end

