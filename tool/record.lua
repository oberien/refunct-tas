local keys = require "keys"
local ui = require "ui"

local win = tas:is_windows()

local record = {}

function record.record(until_key)
  local old_onkeydown = _G.onkeydown
  local old_onkeyup = _G.onkeyup
  local old_drawhud = _G.drawhud

  local x,y,z = tas:get_location()
  local pitch,yaw,roll = tas:get_rotation()
  local rotations = {}
  local down = {}
  local up = {}
  local len = 0
  local down_this_frame = {}
  local up_this_frame = {}
  local stop_initiated = false
  _G.onkeydown = function(key, char, rep)
    if key == until_key then
      stop_initiated = true
      return
    end
    local input = keys.keytoinput(key)
    table.insert(down_this_frame, {input, char == 0 and 0 or input, rep})
  end
  _G.onkeyup = function(key, char, rep)
    local input = keys.keytoinput(key)
    table.insert(up_this_frame, {input, char == 0 and 0 or input, rep})
  end
  _G.drawhud = function()
    ui.drawlines({"Recording"})
  end

  while not stop_initiated do
    tas:set_delta(1/60)
    tas:step()
    table.insert(down, down_this_frame)
    down_this_frame = {}
    table.insert(up, up_this_frame)
    up_this_frame = {}
    local pitch,yaw,roll = tas:get_rotation()
    table.insert(rotations, {pitch, yaw, roll})
    len = len + 1
  end

  _G.onkeydown = old_onkeydown
  _G.onkeyup = old_onkeyup
  _G.drawhud = old_drawhud
  assert(#down == len and #up == len, "invalid recording")
  return {
    x = x,
    y = y,
    z = z,
    pitch = pitch,
    yaw = yaw,
    roll = roll,
    down = down,
    up = up,
    rotations = rotations,
    len = len,
  }
end


function record.play(replay, abort_key)
  tas:set_location(replay.x, replay.y, replay.z)
  tas:set_rotation(replay.pitch, replay.yaw, replay.roll)

  local stop_initiated = false
  local old_onkeydown = _G.onkeydown
  local old_onkeyup = _G.onkeyup
  _G.onkeydown = function(key, char, rep)
    if key == abort_key then
      stop_initiated = true
    end
  end
  _G.onkeyup = function (key, char, rep) end

  for i = 1, 20 do
    tas:step()
  end

  for i = 1, replay.len do
    if stop_initiated then
      break
    end

    tas:set_delta(1/60)
    tas:step()
    for _,key in ipairs(replay.down[i]) do
      tas:key_down(key[1], key[2], key[3])
    end
    for _,key in ipairs(replay.up[i]) do
      tas:key_up(key[1], key[2], key[3])
    end
    local rot = replay.rotations[i]
    tas:set_rotation(rot[1], rot[2], rot[3])
  end
  _G.onkeydown = old_onkeydown
  _G.onkeyup = old_onkeyup
end

function record.save(replay, filename)
  local folder = tas:working_dir() .. "/replays/"
  local exists = os.rename(folder, folder)
  if not exists then
    os.execute("mkdir " .. folder)
  end

  local function serialize_keys(file, keys)
    for _,frame in ipairs(keys) do
      file:write("  {")
      for _,key in ipairs(frame) do
        file:write("{", key[1], ", ", key[2], ", ", tostring(key[3]), "},")
      end
      file:write("},\n")
    end

  end

  local file = io.open(folder .. filename .. ".lua", "w+")
  file:write("return {\n")
  file:write("x = ", replay.x, ",\n")
  file:write("y = ", replay.y, ",\n")
  file:write("z = ", replay.z, ",\n")
  file:write("pitch = ", replay.pitch, ",\n")
  file:write("yaw = ", replay.yaw, ",\n")
  file:write("roll = ", replay.roll, ",\n")
  file:write("down = {\n")
  serialize_keys(file, replay.down)
  file:write("},\n")
  file:write("up = {\n")
  serialize_keys(file, replay.up)
  file:write("},\n")
  file:write("rotations = {\n")
  for _,rot in ipairs(replay.rotations) do
    file:write("  {", rot[1], ", ", rot[2], ", ", rot[3], "},\n")
  end
  file:write("},\n")
  file:write("len = ", replay.len, "\n")
  file:write("}\n")
  file:close()
  replay.saved_as = filename .. ".lua"
end

function record.load(filename)
  local folder = tas:working_dir() .. "/replays/"
  local name = string.sub(filename, 0, -5)
  local replay = dofile(folder .. filename)
  replay.saved_as = filename
  return replay
end

function record.listall()
  local folder = tas:working_dir() .. "/replays/"
  local dirs = {}
  local pfile = nil
  if win then
    pfile = io.popen('dir "' .. folder .. '" /b')
  else
    pfile = io.popen('ls "' .. folder .. '"')
  end

  for filename in pfile:lines() do
    table.insert(dirs, filename)
  end
  pfile:close()
  return dirs
end

return record
