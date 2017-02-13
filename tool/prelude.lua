forward = "forward"
forwards = "forward"
backward = "backward"
backwards = "backward"
left = "left"
right = "right"
jump = "jump"
crouch = "crouch"
menu = "menu"

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

function execframe(frame)
  -- new input
  if frame.forward and not lastframe.forward then
    presskey(forward)
  end
  if frame.backward and not lastframe.backward then
    presskey(backward)
  end
  if frame.left and not lastframe.left then
    presskey(left)
  end
  if frame.right and not lastframe.right then
    presskey(right)
  end
  if frame.jump and not lastframe.jump then
    presskey(jump)
  end
  if frame.crouch and not lastframe.crouch then
    presskey(crouch)
  end
  if frame.menu and not lastframe.menu then
    presskey(menu)
  end

  -- old inputs
  if lastframe.forward and not frame.forward then
    releasekey(forward)
  end
  if lastframe.backward and not frame.backward then
    releasekey(backward)
  end
  if lastframe.left and not frame.left then
    releasekey(left)
  end
  if lastframe.right and not frame.right then
    releasekey(right)
  end
  if lastframe.jump and not frame.jump then
    releasekey(jump)
  end
  if lastframe.crouch and not frame.crouch then
    releasekey(crouch)
  end
  if lastframe.menu and not frame.menu then
    releasekey(menu)
  end

  -- mouse movements
  if frame.mousex ~= 0 or frame.mousey ~= 0 then
    movemouse(frame.mousex, frame.mousey)
  end

  lastframe = frame

  step()
end

function frame(keys, mousex, mousey, repeatnum)
  keys = keys or {}
  mousex = mousex or 0
  mousey = mousey or 0
  repeatnum = repeatnum or 1
  currentframe = Frame:new()

  for i=1,repeatnum do
    for _, key in pairs(keys) do
      currentframe[key] = true
    end
    currentframe.mousex = mousex
    currentframe.mousey = mousey
    execframe(currentframe)
  end
end
