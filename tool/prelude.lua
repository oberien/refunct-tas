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
  mousex = 0,
  mousey = 0,
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

  -- mouse movements
  if frame.mousex ~= 0 or frame.mousey ~= 0 then
    __move_mouse(frame.mousex, frame.mousey)
  end

  lastframe = frame

  step()
end

function framefn(keysfn, degxfn, degyfn, framenum)
  for i=1,framenum do
    currentframe = Frame:new()
    keysfn(currentframe, framenum-i)
    currentframe.mousex = degxfn(framenum-i)
    currentframe.mousey = degyfn(framenum-i)
    execframe(currentframe)
  end
end

local mouseperroll = { value = nil }
local mouseperpitch = { value = nil }

local function generatedegfn(name, totaldeg, totalframes, mousepervalue, debug)
  local debug = debug or false
  if totaldeg == 0 then
    return function() return 0 end
  end


  local startvalue = getplayerstats()[name]
  local lastvalue = nil
  local mouse = nil
  -- how often we passed the 360↔0 border
  local dateline = 0;
  local direction = num(totaldeg > 0) - num(totaldeg < 0)
  -- We keep 10% of frames + one for the last corrections
  local correctionframes = totalframes * 0.10
  local function subtract(minuend, subtrahend)
    if name == "pitch" then
      local tmp = minuend
      minuend = subtrahend
      subtrahend = tmp
    end
    if direction > 0 then
      return (minuend - subtrahend) % 360
    else
      return -(-(minuend - subtrahend) % 360)
    end
  end
  local function degfn(framesleft)
    -- frames left taking into account correction frames
    local framesleft = math.max(framesleft - correctionframes, 1)
    if debug then if debug then print("lastvalue", lastvalue) end end
    if lastvalue == nil then
      -- first call
      if mousepervalue.value == nil then
        -- first call ever, need to initialize
        mouse = direction
      else
        -- we can already estimate a value
        mouse = totaldeg * mousepervalue.value / framesleft
      end
    else
      -- let's assume we'll never turn more than 359 degrees per frame
      if debug then print("getvalue", getplayerstats()[name]) end
      local delta = subtract(getplayerstats()[name], lastvalue)
      if delta == 0 then
        if debug then print("early return because delta is 0") end
        if debug then print() end
        return 0
      end
      if debug then print("delta", delta) end
      mousepervalue.value = mouse / delta
      if debug then print("mouseperval", mousepervalue.value) end
      if debug then print("1/mouseperval", 1/mousepervalue.value) end
      if name == "roll" then
        if direction > 0 and lastvalue + delta >= 360
            or direction < 0 and lastvalue + delta < 0 and -1 then
          dateline = dateline + direction
        end
      else
        if direction > 0 and lastvalue - delta >= 360
            or direction < 0 and lastvalue - delta < 0 and -1 then
          dateline = dateline + direction
        end
      end
      if debug then print("dateline", dateline) end
      local sofar = subtract(getplayerstats()[name], startvalue) + dateline * 360
      if debug then print("sofar", sofar) end
      local leftdeg = (totaldeg - sofar)
      if debug then print("leftdeg", leftdeg) end
      -- early return if we can't get any closer
      if math.abs(leftdeg) < 1/mousepervalue.value then
        if debug then print("early return") end
        if debug then print() end
        return 0
      end
      mouse = mousepervalue.value * leftdeg / framesleft
      mouse = math.round(mouse)
    end
    lastvalue = getplayerstats()[name]
    if debug then print(mouse) end
    if debug then print() end
    return mouse
  end
  return degfn
end

function frame(keys, degx, degy, repeatnum)
  keys = keys or {}
  degx = degx or 0
  degy = degy or 0
  repeatnum = repeatnum or 1

  function keysfn(frame, left)
    for _, key in pairs(keys) do
      frame[key] = true
    end
  end
  local degxfn = generatedegfn("roll", degx, repeatnum, mouseperroll)
  local degyfn = generatedegfn("pitch", degy, repeatnum, mouseperpitch)
  framefn(keysfn, degxfn, degyfn, repeatnum)
end
