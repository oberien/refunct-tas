forward = "forward"
forwards = "forward"
backward = "backward"
backwards = "backward"
left = "left"
right = "right"
jump = "jump"
crouch = "crouch"
menu = "menu"

local deltatime = 0;

function num(var)
  return var and 1 or 0
end

function math.round(num, numDecimalPlaces)
  local mult = 10^(numDecimalPlaces or 0)
  return math.floor(num * mult + 0.5) / mult
end

function contains(list, element)
  for k,v in pairs(list) do
    if v == element then
      return true
    end
  end
  return false
end

function print(...)
  local res = "";
  local args = table.pack(...)
  for i=1,args.n do
    if i > 1 then
      res = res .. " "
    end
    res = res .. tostring(args[i])
  end
  tas:print(res)
end

function step()
  return tas:step()
end

function waitfornewgame()
  tas:wait_for_new_game()
end

function getdelta()
  return tas:get_delta()
end

function setdelta(delta)
  deltatime = delta
end

function getlocation()
  return tas:get_location()
end

function setlocation(x, y, z)
  tas:set_location(x, y, z)
end

function getrotation()
  return tas:get_rotation()
end

function setrotation(pitch, yaw)
  local _, _, roll = getrotation()
  tas:set_rotation(pitch, yaw, roll)
end

function getvelocity()
  return tas:get_velocity()
end

function setvelocity(x, y, z)
  tas:set_velocity(x, y, z)
end

function getacceleration()
  return tas:get_acceleration()
end

function drawline(startx, starty, endx, endy, red, green, blue, alpha, thickness)
  tas:draw_line(startx, starty, endx, endy, red, green, blue, alpha, thickness)
end

function drawtext(text, red, green, blue, alpha, x, y, scale, scaleposition)
  tas:draw_text(text, red, green, blue, alpha, x, y, scale, scaleposition)
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

function execframe(frame)
  -- new input
  if frame.forward and not lastframe.forward then
    tas:press_key(forward)
  end
  if frame.backward and not lastframe.backward then
    tas:press_key(backward)
  end
  if frame.left and not lastframe.left then
    tas:press_key(left)
  end
  if frame.right and not lastframe.right then
    tas:press_key(right)
  end
  if frame.jump and not lastframe.jump then
    tas:press_key(jump)
  end
  if frame.crouch and not lastframe.crouch then
    tas:press_key(crouch)
  end
  if frame.menu and not lastframe.menu then
    tas:press_key(menu)
  end

  -- old inputs
  if lastframe.forward and not frame.forward then
    tas:release_key(forward)
  end
  if lastframe.backward and not frame.backward then
    tas:release_key(backward)
  end
  if lastframe.left and not frame.left then
    tas:release_key(left)
  end
  if lastframe.right and not frame.right then
    tas:release_key(right)
  end
  if lastframe.jump and not frame.jump then
    tas:release_key(jump)
  end
  if lastframe.crouch and not frame.crouch then
    tas:release_key(crouch)
  end
  if lastframe.menu and not frame.menu then
    tas:release_key(menu)
  end

  -- rotation
  if frame.degx ~= 0 or frame.degy ~= 0 then
    local pitch, yaw, _ = getrotation()
    setrotation(pitch + frame.degy, yaw + frame.degx)
  end

  lastframe = frame

  if deltatime ~= 0 then
    tas:set_delta(deltatime)
  end

  step()
end

function frame(keys, degx, degy, repeatnum)
  keys = keys or {}
  degx = degx or 0
  degy = degy or 0
  degy = -degy
  repeatnum = repeatnum or 1
  local pitch, yaw, _ = getrotation()
  startx = yaw
  starty = pitch

  for i=1,repeatnum do
    local currentframe = Frame:new()
    for _, key in pairs(keys) do
      currentframe[key] = true
    end
    if degx ~= 0 or degy ~= 0 then
      framesleft = repeatnum - i + 1
      local pitch, yaw, _ = getrotation()
      remainingx = startx + degx - yaw
      remainingy = starty + degy - pitch
      currentframe.degx = remainingx / framesleft
      currentframe.degy = remainingy / framesleft
    end
    execframe(currentframe)
  end
end


function printstats()
  local x, y, z = getlocation()
  local velx, vely, velz = getvelocity()
  local pitch, yaw, roll = getrotation()
  local accx, accy, accz = getacceleration()
  print(string.format("x: %-6.2f\ty: %-6.2f\tz: %-6.2f\tvelx: %-6.2f\tvely: %-6.2f\tvelz: %-6.2f\tpitch: %-6.2f\tyaw: %-6.2f\troll: %-6.2f\taccx: %-6.2f\taccy: %-6.2f\taccz: %-6.2f\t",
		x, y, z, velx, vely, velz, pitch, yaw, roll, accx, accy, accz))
end
