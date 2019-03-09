function tochar(char)
  if char >= 0x20 and char <= 0x7e then
    return string.lower(string.char(char))
  else
    return nil
  end
end

function record_replay(until_char)
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
    local char_char = tochar(char)
    if char_char == until_char then
      stop_initiated = true
      return
    end
    if key == 225 then
      -- weird fix for linux where key + (1 << 30) is required
      key = 1073742049
    end
    table.insert(down_this_frame, {key, char, rep})
  end
  _G.onkeyup = function(key, char, rep)
    if key == 225 then
      -- weird fix for linux where key + (1 << 30) is required
      key = 1073742049
    end
    table.insert(up_this_frame, {key, char, rep})
  end
  _G.drawhud = function()
    tas:draw_text("Recording", 0, 0, 0, 1, 0, 0, 1, true)
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


function play_replay(replay)
  tas:set_location(replay.x, replay.y, replay.z)
  tas:set_rotation(replay.pitch, replay.yaw, replay.roll)

  local stop_initiated = false
  old_onkeydown = _G.onkeydown
  old_onkeyup = _G.onkeyup
  _G.onkeydown = function(key, char, rep)
    char = tochar(char)
    if char == "t" then
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
