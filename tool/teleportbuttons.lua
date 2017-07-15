require "prelude"

direction = 1
function wait(num)
  num = num or 1
  __move_mouse(direction, direction)
  direction = direction * -1
  frame({},0,0,num)
end

function button(x, y, z, waittime)
  waittime = waittime or 4
  setlocation(x, y, z)
  setvelocity(0,0,0)
  -- wait for button to register
  wait(3)
  -- wait for new platform to rise
  setdelta(1/2)
  wait(waittime)
  setdelta(1/60)
end

setdelta(1/60)
while true do
  waitfornewgame()
  -- button 1
  setdelta(1/2)
  wait(9)
  setdelta(1/60)
  button(-1000, -1000, 732)
  -- button 2
  button(-2000, 0, 857)
  -- button 3
  button(2125, -250, 1107)
  -- button 4
  button(-2725, -875, 193)
  -- button 5
  button(-5000, -875, 857, 6)
  -- button 6
  button(-3250, -2250, 1800)
  -- button 7/7.5
  setlocation(-4625, -3000, 107)
  wait()
  button(-4625, -3625, 107)
  -- button 8
  button(-2750, -3750, 1607)
  -- button 9
  button(-625, -3375, 1607, 10)
  -- button 10/10.5
  setlocation(0, -2375, 107)
  wait()
  button(2000, -2375, 232)
  -- button 11
  button(2000, -2375, 232)
  -- button 12
  button(1875, 975, 232)
  -- button 13
  button(2375, -500, 107)
  -- button 14
  button(600, 2625, 232)
  -- button 15
  button(-875, 2500, 232)
  -- button 16
  button(-375, 1625, 732)
  -- button 17
  button(-2750, 1500, 857)
  -- button 18
  button(-1875, 1125, 1107, 7)
  -- button 19/19.5
  setlocation(-5125, -1750, 107)
  wait()
  button(-4250, -4000, 1607, 5)
  -- button 20
  button(2000, -3875, 1232)
  -- button 21 - Spiral
  button(4250, -2125, 1107)
  -- button 22
  button(2750, -4100, 68)
  -- button 23
  button(3000, -1000, 232)
  --  button 24
  button(2500, 2250, 607, 5)
  -- button 25
  button(375, 4750, 1357)
  -- button 26
  button(4500, 4625, 232)
  -- button 27/27.3/27.6
  setlocation(3125, 6120, 232)
  wait()
  setlocation(1375, 6500, 232)
  wait()
  button(-875, 5625, 232)
  -- button 28
  button(-1375, 3000, 982, 6)
  -- button 29/29.5
  setlocation(-4875, 1750, 1357)
  wait()
  button(-5250, -250, 1357)
  -- button 30
  button(4888, 2500, 607)
  -- button 31
  button(3750, -500, 318, 7)
  -- button 32
  setlocation(2625, -2250, 1357)
  wait()
end
