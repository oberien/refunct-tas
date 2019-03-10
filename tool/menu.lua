require "prelude"
require "teleportbutton-prelude"
local record = require "record"
local ui = require "ui"
local KEYS = require "keys"

local STATES = {
  FIRSTSTART = {},
  NONE = {},
  MENU = {},
  PRACTICE = {},
  SETTINGS = {},
  LOAD_REPLAY = {},
}

local state = STATES.FIRSTSTART
local drawstats = false
local practicefunction = nil
local replay = {}

local function firststart()
  ui.drawlines({"Press 'm' for menu, 'r' to record, 't' to replay the record"})
end

local function menu()
  local selected = ui.select({"Practice", "Load Replay", "Settings", "Back"});
  if selected == 1 then
    state = STATES.PRACTICE
  elseif selected == 2 then
    state = STATES.LOAD_REPLAY
  elseif selected == 3 then
    state = STATES.SETTINGS
  elseif selected == 4 or selected == nil then
    state = STATES.NONE
  else
    error("invalid selection (internal error)")
  end
end

local function practice()
  local function sel(val)
    return practicefunction == val and " (Selected)" or ""
  end
  local selected = ui.select({
    "None" .. sel(nil) ,
    "Dive Skip" .. sel(dive),
    "LoF & Spiral Skip" .. sel(spiral),
    "Final Climb / 31" .. sel(finalclimb),
    "Back"
  })
  if selected == 1 then practicefunction = nil
  elseif selected == 2 then practicefunction = dive
  elseif selected == 3 then practicefunction = spiral
  elseif selected == 4 then practicefunction = finalclimb
  elseif selected == 5 or selected == nil then
    state = STATES.MENU
  else
    error("invalid selection (internal error)")
  end
end

local function load_replay()
  local replays = record.listall()
  local query = {}
  for _,rep in ipairs(replays) do
    if replay.saved_as == rep then
      table.insert(query, rep .. " (loaded)")
    else
      table.insert(query, rep)
    end
  end
  table.insert(query, "Back")
  local selected = ui.select(query)
  if selected == #query then
    state = STATES.MENU
  else
    replay = record.load(replays[selected])
  end
end

local function settings()
  local selected = ui.select({
    "Font Scale (" .. ui.scale .. ")",
    "Show Stats    " .. (drawstats and "On" or "Off"),
    "Back",
  })
  if selected == 1 then
    local scale = nil
    local error = ""
    while type(scale) ~= "number" do
      local input = ui.input(error .. "Input Font Scale")
      scale = tonumber(input)
      error = "Invalid Number. "
    end
    ui.scale = scale
  elseif selected == 2 then
    drawstats = true
  elseif selected == 3 or selected == nil then
    state = STATES.MENU
  else
    error("invalid selection (internal error)")
  end
end

local function stats()
  local x,y,z = getlocation()
  local velx, vely, velz = getvelocity()
  local pitch, yaw, roll = getrotation()
  local accx, accy, accz = getacceleration()
  ui.drawlines({
    string.format("x: %6.2f    y: %6.2f    z: %6.2f", x, y, z),
    string.format("velx: %6.2f    vely: %6.2f    velz: %6.2f", velx, vely, velz),
    string.format("velxy: %6.2f", math.sqrt(velx*velx + vely*vely)),
    string.format("velxyz: %6.2f", math.sqrt(velx*velx + vely*vely + velz*velz)),
    string.format("pitch: %6.2f    yaw: %6.2f    roll: %6.2f", pitch, yaw, roll),
  })
end

drawhud = function()
  if state == STATES.NONE then
    if drawstats then
      stats()
    end
  elseif state == STATES.FIRSTSTART then
    firststart()
  elseif state == STATES.MENU then
    menu()
  elseif state == STATES.PRACTICE then
    practice()
  elseif state == STATES.LOAD_REPLAY then
    load_replay()
  elseif state == STATES.SETTINGS then
    settings()
  else
    error("invalid state (internal error)")
  end
end

onkeydown = function(key, char, rep)
  if key == KEYS.KEY_R then
    print("start recording")
    replay = record.record(KEYS.KEY_R)
    print("stopped recording")
    local filename = ui.input("Save Replay as")
    if filename ~= nil and filename ~= "" then
      record.save(replay, filename)
      print("replay saved as replays/" .. filename)
    end
  end
  if key == KEYS.KEY_T then
    print("start playing")
    record.play(replay, KEYS.KEY_T)
    print("stopped playing")
  end

  if key == KEYS.KEY_M then
    if state == STATES.NONE or state == STATES.FIRSTSTART then
      state = STATES.MENU
    else
      state = STATES.NONE
    end
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

