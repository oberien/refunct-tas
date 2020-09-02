require "prelude"
require "teleportbutton-prelude"
local mp = require "multiplayer"
local record = require "record"
local ui = require "ui"
local KEYS = require "keys"
local randomizer = require "randomizer"
local allbuttons = require "allbuttons"

local STATES = {
  FIRSTSTART = {},
  NONE = {},
  MENU = {},
  PRACTICE = {},
  RANDOMIZERMENU = {},
  ALLBUTTONS = {},
  LOAD_REPLAY = {},
  MULTIPLAYER = {},
  SETTINGS = {},
}

local state = STATES.FIRSTSTART
local drawstats = false
local drawrandomizer = false
local resetfunction = nil
local replay = {}

local function firststart()
  ui.drawlines({"Press 'm' for menu, 'r' to record, 't' to replay the record"})
end

local function menu()
  local selected = ui.select({"Practice", "Randomizer", "All Buttons", "Load Replay", "Multiplayer", "Settings", "Back"})
  if selected == 1 then
    state = STATES.PRACTICE
  elseif selected == 2 then
    state = STATES.RANDOMIZERMENU
  elseif selected == 3 then
    state = STATES.ALLBUTTONS
  elseif selected == 4 then
    state = STATES.LOAD_REPLAY
  elseif selected == 5 then
    state = STATES.MULTIPLAYER
  elseif selected == 6 then
    state = STATES.SETTINGS
  elseif selected == 7 or selected == nil then
    state = STATES.NONE
  else
    error("invalid selection (internal error)")
  end
end

local function practice()
  local function sel(val)
    return resetfunction == val and " (Selected)" or ""
  end
  local selected = ui.select({
    "None" .. sel(nil) ,
    "Dive Skip" .. sel(dive),
    "LoF & Spiral Skip" .. sel(spiral),
    "Final Climb / Hdnoftr" .. sel(finalclimb),
    "Ls Jump" .. sel(lsjump),
    "Pit" .. sel(pit),
    "Pillars" .. sel(pillars),
    "5 Turn & 6 Elevator" .. sel(firstele),
    "16" .. sel(sixteen),
    "21" .. sel(spiralslide),
    "Button 2" .. sel(button2),
    "Button 3" .. sel(button3),
    "Button 4" .. sel(button4),
    "Button 5" .. sel(button5),
    "Button 6" .. sel(button6),
    "Button 7" .. sel(button7),
    "Button 8" .. sel(button8),
    "Button 9" .. sel(button9),
    "Button 10" .. sel(button10),
    "Button 11" .. sel(button11),
    "Button 12" .. sel(button12),
    "Button 13" .. sel(button13),
    "Button 14" .. sel(button14),
    "Button 15" .. sel(button15),
    "Button 16" .. sel(button16),
    "Button 17" .. sel(button17),
    "Button 18" .. sel(button18),
    "Button 19" .. sel(button19),
    "Button 20" .. sel(button20),
    "Button 21" .. sel(button21),
    "Button 22" .. sel(button22),
    "Button 23" .. sel(button23),
    "Button 24" .. sel(button24),
    "Button 25" .. sel(button25),
    "Button 26" .. sel(button26),
    "Button 27" .. sel(button27),
    "Button 28" .. sel(button28),
    "Button 29" .. sel(button29),
    "Button 30" .. sel(button30),
    "Button 31" .. sel(button31),
    "Button 32" .. sel(button32),
    "Back"
  })
  if selected == 1 then resetfunction = nil
  elseif selected == 2 then resetfunction = dive
  elseif selected == 3 then resetfunction = spiral
  elseif selected == 4 then resetfunction = finalclimb
  elseif selected == 5 then resetfunction = lsjump
  elseif selected == 6 then resetfunction = pit
  elseif selected == 7 then resetfunction = pillars
  elseif selected == 8 then resetfunction = firstele
  elseif selected == 9 then resetfunction = sixteen
  elseif selected == 10 then resetfunction = spiralslide
  elseif selected == 11 then resetfunction = button2
  elseif selected == 12 then resetfunction = button3
  elseif selected == 13 then resetfunction = button4
  elseif selected == 14 then resetfunction = button5
  elseif selected == 15 then resetfunction = button6
  elseif selected == 16 then resetfunction = button7
  elseif selected == 17 then resetfunction = button8
  elseif selected == 18 then resetfunction = button9
  elseif selected == 19 then resetfunction = button10
  elseif selected == 20 then resetfunction = button11
  elseif selected == 21 then resetfunction = button12
  elseif selected == 22 then resetfunction = button13
  elseif selected == 23 then resetfunction = button14
  elseif selected == 24 then resetfunction = button15
  elseif selected == 25 then resetfunction = button16
  elseif selected == 26 then resetfunction = button17
  elseif selected == 27 then resetfunction = button18
  elseif selected == 28 then resetfunction = button19
  elseif selected == 29 then resetfunction = button20
  elseif selected == 30 then resetfunction = button21
  elseif selected == 31 then resetfunction = button22
  elseif selected == 32 then resetfunction = button23
  elseif selected == 33 then resetfunction = button24
  elseif selected == 34 then resetfunction = button25
  elseif selected == 35 then resetfunction = button26
  elseif selected == 36 then resetfunction = button27
  elseif selected == 37 then resetfunction = button28
  elseif selected == 38 then resetfunction = button29
  elseif selected == 39 then resetfunction = button30
  elseif selected == 40 then resetfunction = button31
  elseif selected == 41 then resetfunction = button32
  elseif selected == 42 or selected == nil then
   state = STATES.MENU
  else
    error("invalid selection (internal error)")
  end
end

local function randomizermenu()
  local selected = ui.select({
    "New Seed when starting New Game: " .. randomizer.getnewgamenewseed(),
    "Difficulty: " .. randomizer.getdifficulty(),
    "Random Seed",
    "Set Seed",
    "Set Sequence",
    "Reset",
    "Back",
  })
  if selected == 1 then -- New Seed on New Game
    randomizer.cyclenewgamenewseed()
    randomizer.setnextseedwithlogic()

  elseif selected == 2 then -- Difficulty
    randomizer.cycledifficulty()

  elseif selected == 3 then -- Random seed
    randomizer.setnextseed(randomizer.SEEDTYPE.RANDOMSEED)
    drawrandomizer = true
    resetfunction = randomizer.randomize
    state = STATES.NONE

  elseif selected == 4 then -- Set seed
    local seed = nil
    local error = ""
    while type(seed) ~= "number" do
      local input = ui.input(error .. "Input Seed", randomizer.seed)
      seed = tonumber(input)
      error = "Invalid Number. "
    end
    randomizer.setnextseed(randomizer.SEEDTYPE.SETSEED, seed)
    drawrandomizer = true
    resetfunction = randomizer.randomize
    state = STATES.NONE

  elseif selected == 5 then -- Set Sequence
    local sequence = nil
    local check = false
    local error = ""
    while check == false do
      local input = ui.input(error .. "Input Sequence")
      sequence = nil
      for i in string.gmatch(input, "%d+") do
        table.insert(sequence, math.floor(i - 2))
      end
      if #sequence > 0 and table.checkifinbounds(sequence, 0, 29) and table.checkifduplicate(sequence) and table.contains(sequence, 29) then
        check = true
      end
    end
    randomizer.setnextseed(randomizer.SEEDTYPE.SETSEQUENCE, sequence)
    drawrandomizer = true
    resetfunction = randomizer.randomize
    state = STATES.NONE

  elseif selected == 6 then -- Reset
    randomizer.reset()
    drawrandomizer = false
    resetfunction = nil
    state = STATES.MENU

  elseif selected == 7 or selected == nil then -- Back
    state = STATES.MENU

  else
    error("invalid selection (internal error)")
  end
end

local function allbuttonsmenu()
  local selected = ui.select({
    "Start",
    "Reset",
    "Back"
  })
  if selected == 1 then
    allbuttons.start()
    state = STATES.NONE
  elseif selected == 2 then
    allbuttons.reset()
    state = STATES.NONE
  elseif selected == 3 or selected == nil then
    state = STATES.NONE
  else
    error("invalid selection (internal error)")
  end
end

local function loadreplay()
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
  if selected == #query or selected == nil then
    state = STATES.MENU
  else
    replay = record.load(replays[selected])
  end
end

local function multiplayer()
  local selected = ui.select({"Join/Create Room", "Disconnect", "Back"})
  if selected == 1 then
    -- continue
  elseif selected == 2 then
    mp.disconnect()
    state = STATES.MENU
    return
  else
    state = STATES.MENU
    return
  end

  local room = ui.input("Room name to join/create")
  if room == "" or room == nil then
    state = STATES.MENU
    return
  end
  mp.connect()
  mp.join(room)
  state = STATES.MENU
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
    drawstats = not drawstats
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
  return {
    string.format("x: %6.2f    y: %6.2f    z: %6.2f", x, y, z),
    string.format("velx: %6.2f    vely: %6.2f    velz: %6.2f", velx, vely, velz),
    string.format("velxy: %6.2f", math.sqrt(velx*velx + vely*vely)),
    string.format("velxyz: %6.2f", math.sqrt(velx*velx + vely*vely + velz*velz)),
    string.format("pitch: %6.2f    yaw: %6.2f    roll: %6.2f", pitch, yaw, roll),
  }
end

drawhud = function()
  mp.draw()

  if state == STATES.NONE then
    local randomizerlines, statslines = {}, {}
    if drawrandomizer then
      randomizerlines = randomizer.hudlines()
    end
    if drawstats then
      statslines = stats()
    end
    for _,line in ipairs(statslines) do
      table.insert(randomizerlines, line)
  end
    ui.drawlines(randomizerlines)
  elseif state == STATES.FIRSTSTART then
    firststart()
  elseif state == STATES.MENU then
    menu()
  elseif state == STATES.PRACTICE then
    practice()
  elseif state == STATES.RANDOMIZERMENU then
    randomizermenu()
  elseif state == STATES.ALLBUTTONS then
    allbuttonsmenu()
  elseif state == STATES.LOAD_REPLAY then
    loadreplay()
  elseif state == STATES.MULTIPLAYER then
    multiplayer()
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

function tp_to(button, rotation, location, waittime)
  waittime = waittime or 0
  setdelta(1/60)
  teleportbutton(button)
  setdelta(1/2)
  if waittime ~= 0 then
    wait(waittime)
  end
  setrotation(rotation[1], rotation[2], rotation[3])
  setlocation(location[1], location[2], location[3])
  setvelocity(0,0,0)
  setacceleration(0,0,0)
  if waittime ~= 0 then
    wait(waittime)
  end
  setdelta(0)
end

function spiral()
  tp_to(18, {0,0,0}, {-1065, -3842, 464})
end
function button2()
  tp_to(1, {327.65,135.33,0}, {-1037.57, -955.68, 732.16})
end
function button3()
  tp_to(2, {344.88,359.73,0}, {-1904.85, -8.17, 857.28})
end
function button4()
  tp_to(3, {338.98,187.46,0}, {2074.04, -260.32, 1107.16})
end
function button5()
  tp_to(4, {340.15,179.73,0}, {-2728.39, -837.92, 193.16})
end
function button6()
  tp_to(5, {337.15,333.42,0}, {-4891.93, -892.98, 857.16})
end
function button7()
  tp_to(6, {305.20,204.11,0}, {-3241.14, -2295.33, 1607.15})
end
function button8()
  tp_to(7, {340.17,357.06,0}, {-4663.23, -3636.14, 107.16})
end
function button9()
  tp_to(8, {339.01,0.22,0}, {-2827.18, -3767.32, 1607.25})
end
function button10()
  tp_to(9, {320.00,89.51,0}, {-648.95, -3328.46, 1607.16})
end
function button11()
  tp_to(10, {343.86,92,0}, {1950, -2312.88, 232.16})
end
function button12()
  tp_to(11, {355.37,90.26,0}, {1910.94, 859.68, 239.98})
end
function button13()
  tp_to(12, {344.65,90.26,0}, {2382.06, -431.27, 107.16})
end
function button14()
  tp_to(13, {346.40,169.93,0}, {607.75, 2504.56, 228.53})
end
function button15()
  tp_to(14, {0.90,298.41,0}, {-865.92, 2487.93, 232.35})
end
function button16()
  tp_to(15, {339.76,301.56,0}, {-465.44, 1604.85, 732.16})
end
function button17()
  tp_to(16, {328.47,339.28,0}, {-2652.97, 1453.47, 857.21})
end
function button18()
  tp_to(17, {325.51,231.20,0}, {-1895.30, 1134.02, 1107.64})
end
function button19()
  tp_to(18, {340.55,357.75,0}, {-4147.88, -4007.69, 1607.26})
end
function button20()
  tp_to(19, {334.01,30.91,0}, {2026.23, -3783.01, 1232.17})
end
function button21()
  tp_to(20, {332.07,232.09,0}, {4226, -2202.19, 1107.16})
end
function button22()
  tp_to(21, {355.22,70.25,0}, {2737, -4020.95, 68.16})
end
function button23()
  tp_to(22, {352.48,99.27,0}, {3034.37, -985.16, 232.30})
end
function button24()
  tp_to(23, {341.05,140.52,0}, {2412.56, 2271.34, 607.15})
end
function button25()
  tp_to(24, {318.08,358.64,0}, {492.67, 4725.55, 1355.44})
end
function button26()
  tp_to(25, {338.07,135.37,0}, {4477.55, 4711.60, 232.16})
end
function button27()
  tp_to(26, {331.94,257.52,0}, {-883.96, 5552.63, 232.16})
end
function button28()
  tp_to(27, {344.67,228.22,0}, {-1411.66, 2970.87, 982.16})
end
function button29()
  tp_to(28, {342.90,13.59,0}, {-5176.97, -222.32, 1357.16})
end
function button30()
  tp_to(29, {345.10,247.02,0}, {4846.33, 2449.16, 607.32})
end
function button31()
  tp_to(30, {350.36,243.57,0}, {3740.51, -534.68, 318.16})
end
function button32()
  tp_to(31, {300.36,107.72,0}, {2617.49, -2265.24, 1357.16})
end
function dive()
  tp_to(8, {0,0,0}, {-1065, -3842, 464}, 5)
end
function finalclimb()
  tp_to(29, {0, 247, 0}, {4741, 2294, 588}, 5)
end
function lsjump()
  tp_to(6, {0, 180, 0}, {-4265, -2989, 90})
end
function pit()
  tp_to(10, {0, 90, 0}, {1859, -869, 89})
end
function pillars()
  tp_to(26, {0, 256, 0}, {-847, 5589, 231})
end
function firstele()
  tp_to(4, {0, 180, 0}, {-4284, -806, 840}, 10)
end
function sixteen()
  tp_to(15, {0, 200, 0}, {-752, 1513, 839})
end
function spiralslide()
  tp_to(19, {0, 35, 0}, {4015, -2743, 589})
end

while true do
  waitfornewgame()
  if resetfunction ~= nil then
    resetfunction()
  end
end

