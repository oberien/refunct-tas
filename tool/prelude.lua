forward = "forward"
forwards = "forward"
backward = "backward"
backwards = "backward"
left = "left"
right = "right"
jump = "jump"
crouch = "crouch"
menu = "menu"

function num(var)
  return var and 1 or 0
end

function math.round(num, numDecimalPlaces)
  local mult = 10^(numDecimalPlaces or 0)
  return math.floor(num * mult + 0.5) / mult
end

Frame = {
  forward = false,
  backward = false,
  left = false,
  right = false,
  jump = false,
  crouch = false,
  menu = false,
  degx = 0,
  degy = 0,
}

function Frame:new(o)
  o = o or {}
  setmetatable(o, self)
  self.__index = self
  return o
end

local lastframe = Frame:new()
local playerstats = getmetatable(__stop())

function waitfornewgame()
  playerstats = getmetatable(__wait_for_new_game())
end

function setdelta(delta)
  __set_delta(delta)
end

function setrotation(pitch, yaw)
  __set_rotation(pitch, yaw, playerstats.roll)
end

function step()
  playerstats = getmetatable(__step())
end

function getplayerstats()
  return playerstats
end

function execframe(frame)
  -- new input
  if frame.forward and not lastframe.forward then
    __press_key(forward)
  end
  if frame.backward and not lastframe.backward then
    __press_key(backward)
  end
  if frame.left and not lastframe.left then
    __press_key(left)
  end
  if frame.right and not lastframe.right then
    __press_key(right)
  end
  if frame.jump and not lastframe.jump then
    __press_key(jump)
  end
  if frame.crouch and not lastframe.crouch then
    __press_key(crouch)
  end
  if frame.menu and not lastframe.menu then
    __press_key(menu)
  end

  -- old inputs
  if lastframe.forward and not frame.forward then
    __release_key(forward)
  end
  if lastframe.backward and not frame.backward then
    __release_key(backward)
  end
  if lastframe.left and not frame.left then
    __release_key(left)
  end
  if lastframe.right and not frame.right then
    __release_key(right)
  end
  if lastframe.jump and not frame.jump then
    __release_key(jump)
  end
  if lastframe.crouch and not frame.crouch then
    __release_key(crouch)
  end
  if lastframe.menu and not frame.menu then
    __release_key(menu)
  end

  -- rotation
  if frame.degx ~= 0 or frame.degy ~= 0 then
    local stats = getplayerstats()
    setrotation(stats.pitch + frame.degy, stats.yaw + frame.degx)
  end

  lastframe = frame

  step()
end

function frame(keys, degx, degy, repeatnum)
  keys = keys or {}
  degx = degx or 0
  degy = degy or 0
  degy = -degy
  repeatnum = repeatnum or 1
  stats = getplayerstats()
  startx = stats.yaw
  starty = stats.pitch

  for i=1,repeatnum do
    local currentframe = Frame:new()
    for _, key in pairs(keys) do
      currentframe[key] = true
    end
    framesleft = repeatnum - i + 1
    stats = getplayerstats()
    remainingx = startx + degx - stats.yaw
    remainingy = starty + degy - stats.pitch
    currentframe.degx = remainingx / framesleft
    currentframe.degy = remainingy / framesleft
    execframe(currentframe)
  end
end
