require "prelude"

local direction = 1
function wait(num)
  num = num or 1
  -- mouse movement needed to update rendering viewport
  tas:move_mouse(direction, direction)
  direction = direction * -1
  -- all keys pressed to disable user input
  frame({forward, backward, left, right},0,0,num-1)
  frame({},0,0,1)
end

local button = function(x, y, z, waittime)
  waittime = waittime or 8
  setlocation(x, y, z)
  setvelocity(0,0,0)
  -- wait for button to register
  wait(3)
  -- wait for new platform to rise
  setdelta(1/2)
  wait(waittime)
  setdelta(1/60)
end
local cube = function(x, y, z)
  setlocation(x, y, z)
  setvelocity(0, 0, 0)
  wait(3)
end

function teleportcube(num)
  if num == 1 then cube(-2250, -1250, 1089) end
  if num == 2 then cube(-4800, -3375, 714) end
  if num == 3 then cube(-3250, -4625, 90) end
  if num == 4 then cube(-2375, -3750, 2090) end
  if num == 5 then cube(-125, -3500, 90) end
  if num == 6 then cube(-500, -2000, 1590) end
  if num == 7 then cube(2375, -1125, 965) end
  if num == 8 then cube(875, 1900, 714) end
  if num == 9 then cube(-500, 2875, 964) end
  if num == 10 then cube(-4500, -2225, 1339) end
  if num == 11 then cube(5000, -2625, 90) end
  if num == 12 then cube(4125, -4250, 840) end
  if num == 13 then cube(2750, 1250, 1089) end
  if num == 14 then cube(-1625, 4375, 964) end
  if num == 15 then cube(-5625, 375, 714) end
  if num == 16 then cube(3425, 5100, 1839) end
  if num == 17 then cube(5375, 1875, 214) end
  if num == 18 then cube(4750, -350, 964) end
end

function teleportexact(num)
  if num == 1 then
    button(-1000, -1000, 732)
  end
  if num == 2 then
    button(-2000, 0, 857)
  end
  if num == 3 then
    button(2125, -250, 1107)
  end
  if num == 4 then
    button(-2725, -875, 193)
  end
  if num == 5 then
    button(-5000, -875, 857, 6)
  end
  if num == 6 then
    button(-3250, -2250, 1800)
  end
  if num == 7 then
    setlocation(-4625, -3000, 107)
    wait()
    button(-4625, -3625, 107)
  end
  if num == 8 then
    button(-2750, -3750, 1607)
  end
  if num == 9 then
    button(-625, -3375, 1607, 10)
  end
  if num == 10 then
    setlocation(0, -2375, 107)
    wait()
    button(2000, -2375, 232)
  end
  if num == 11 then
    button(1875, 975, 232)
  end
  if num == 12 then
    button(2375, -500, 107)
  end
  if num == 13 then
    button(600, 2625, 232)
  end
  if num == 14 then
    button(-875, 2500, 232)
  end
  if num == 15 then
    button(-375, 1625, 732)
  end
  if num == 16 then
    button(-2750, 1500, 857)
  end
  if num == 17 then
    button(-1875, 1125, 1107, 7)
  end
  if num == 18 then
    setlocation(-5125, -1750, 107)
    wait()
    button(-4250, -4000, 1607, 5)
  end
  if num == 19 then
    button(2000, -3875, 1232)
  end
  if num == 20 then
    button(4250, -2125, 1107)
  end
  if num == 21 then
    button(2750, -4100, 68)
  end
  if num == 22 then
    button(3000, -1000, 232)
  end
  if num == 23 then
    button(2500, 2250, 607, 5)
  end
  if num == 24 then
    button(375, 4750, 1357)
  end
  if num == 25 then
    button(4500, 4625, 232)
  end
  if num == 26 then
    setlocation(3125, 6120, 232)
    wait()
    setlocation(1375, 6500, 232)
    wait()
    button(-875, 5625, 232)
  end
  if num == 27 then
    button(-1375, 3000, 982, 6)
  end
  if num == 28 then
    setlocation(-4875, 1750, 1357)
    wait()
    button(-5250, -250, 1357)
  end
  if num == 29 then
    button(4888, 2500, 607)
  end
  if num == 30 then
    button(3750, -500, 318, 7)
  end
  if num == 31 then
    setlocation(2625, -2250, 1357)
    wait()
  end
end

setdelta(1/60)
function teleportbutton(num)
  buttonmax = num
  -- button 1
  setdelta(1/2)
  wait(9)
  setdelta(1/60)
  button(-1000, -1000, 732)
  if num == 1 then return end
  -- button 2
  button(-2000, 0, 857)
  if num == 2 then return end
  -- button 3
  button(2125, -250, 1107)
  if num == 3 then return end
  -- button 4
  button(-2725, -875, 193)
  if num == 4 then return end
  -- button 5
  button(-5000, -875, 857, 6)
  if num == 5 then return end
  -- button 6
  button(-3250, -2250, 1800)
  if num == 6 then return end
  -- button 7/7.5
  setlocation(-4625, -3000, 107)
  wait()
  button(-4625, -3625, 107)
  if num == 7 then return end
  -- button 8
  button(-2750, -3750, 1607)
  if num == 8 then return end
  -- button 9
  button(-625, -3375, 1607, 10)
  if num == 9 then return end
  -- button 10/10.5
  setlocation(0, -2375, 107)
  wait()
  button(2000, -2375, 232)
  if num == 10 then return end
  -- button 11
  button(1875, 975, 232)
  if num == 11 then return end
  -- button 12
  button(2375, -500, 107)
  if num == 12 then return end
  -- button 13
  button(600, 2625, 232)
  if num == 13 then return end
  -- button 14
  button(-875, 2500, 232)
  if num == 14 then return end
  -- button 15
  button(-375, 1625, 732)
  if num == 15 then return end
  -- button 16
  button(-2750, 1500, 857)
  if num == 16 then return end
  -- button 17
  button(-1875, 1125, 1107, 7)
  if num == 17 then return end
  -- button 18/18.5
  setlocation(-5125, -1750, 107)
  wait()
  button(-4250, -4000, 1607, 5)
  if num == 18 then return end
  -- button 19
  button(2000, -3875, 1232)
  if num == 19 then return end
  -- button 20 - Spiral
  button(4250, -2125, 1107)
  if num == 20 then return end
  -- button 21
  button(2750, -4100, 68)
  if num == 21 then return end
  -- button 22
  button(3000, -1000, 232)
  if num == 22 then return end
  -- button 23
  button(2500, 2250, 607, 5)
  if num == 23 then return end
  -- button 24
  button(375, 4750, 1357)
  if num == 24 then return end
  -- button 25
  button(4500, 4625, 232)
  if num == 25 then return end
  -- button 26/26.3/26.6
  setlocation(3125, 6120, 232)
  wait()
  setlocation(1375, 6500, 232)
  wait()
  button(-875, 5625, 232)
  if num == 26 then return end
  -- button 27
  button(-1375, 3000, 982, 6)
  if num == 27 then return end
  -- button 28/28.5
  setlocation(-4875, 1750, 1357)
  wait()
  button(-5250, -250, 1357)
  if num == 28 then return end
  -- button 29
  button(4888, 2500, 607)
  if num == 29 then return end
  -- button 30
  button(3750, -500, 318, 7)
  if num == 30 then return end
  -- button 31
  setlocation(2625, -2250, 1357)
  wait()
end
