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

function teleportplatform(num)
  -- button 1
  if num == 1 then cube(-1000, -500, 139) end
  if num == 2 then cube(-1750, -600, 89) end
  if num == 3 then cube(-1500, -1400, 214) end
  if num == 4 then cube(-1400, -1900, 339) end
  if num == 5 then cube(-1050, -1600, 589) end
  -- button 2
  if num == 6 then cube(-500, -250, 214) end
  if num == 7 then cube(-860, 360, 89) end
  if num == 8 then cube(-850, 500, 339) end
  if num == 9 then cube(-900, 100, 589) end
  if num == 10 then cube(-2200, 400, 339) end
  if num == 11 then cube(-2350, 50, 589) end
  -- button 3
  if num == 12 then cube(-150, 250, 589) end
  if num == 13 then cube(750, 300, 589) end
  if num == 14 then cube(750, -750, 58) end
  if num == 15 then cube(350, -725, 339) end
  if num == 16 then cube(500, -250, 89) end
  if num == 17 then cube(1400, 325, 339) end
  if num == 18 then cube(1525, -250, 839) end
  if num == 19 then cube(750, -1125, 339) end
  -- button 4 (+cube)
  if num == 20 then cube(-2750, -400, 39) end
  if num == 21 then cube(-2750, -100, 89) end
  if num == 22 then cube(-2550, -950, 714) end
  if num == 23 then cube(-2900, -850, 714) end
  if num == 24 then cube(-2650, -600, 839) end
  if num == 25 then cube(-2750, -1200, 589) end
  -- button 5
  if num == 26 then cube(-3100, -1250, 39) end
  if num == 27 then cube(-3500, -1200, 89) end
  if num == 28 then cube(-4000, -850, 214) end
  if num == 29 then cube(-4150, -625, 714) end
  if num == 30 then cube(-4300, -900, 839) end
  if num == 31 then cube(-4600, -1250, 714) end
  -- button 6
  if num == 32 then cube(-2450, -1700, 89) end
  if num == 33 then cube(-3000, -2750, 89) end
  if num == 34 then cube(-2350, -2700, 214) end
  if num == 35 then cube(-2000, -2250, 2089) end
  -- button 7
  if num == 36 then cube(-4275, -3400, 589) end
  -- button 8
  if num == 37 then cube(-3750, -3875, 214) end
  if num == 38 then cube(-3250, -4200, 464) end
  if num == 39 then cube(-2650, -4200, 339) end
  if num == 40 then cube(-2000, -4000, 214) end
  if num == 41 then cube(-1550, -3400, 89) end
  if num == 42 then cube(-2600, -3300, 339) end
  if num == 43 then cube(-3250, -3250, 464) end
  -- button 9
  if num == 44 then cube(-1400, -4000, 339) end
  if num == 45 then cube(-1000, -4000, 464) end
  if num == 46 then cube(-550, -4000, 514) end
  if num == 47 then cube(-400, -4500, 89) end
  if num == 48 then cube(450, -4400, 89) end
  if num == 49 then cube(450, -4000, 214) end
  if num == 50 then cube(0, -4100, 464) end
  if num == 51 then cube(-1000, -3500, 214) end
  -- button 10
  if num == 52 then cube(-75, -3000, 589) end
  if num == 53 then cube(-182, -2500, 714) end
  if num == 54 then cube(150, -2400, 714) end
  if num == 55 then cube(-50, 1800, 589) end
  if num == 56 then cube(0, -1400, 339) end
  if num == 57 then cube(-1100, -2175, 89) end
  -- button 11
  if num == 58 then cube(900, -2100, 89) end
  if num == 59 then cube(450, -1450, 89) end
  if num == 60 then cube(1150, -1600, 89) end
  if num == 61 then cube(1650, -1350, 89) end
  if num == 62 then cube(2025, -1650, 89) end
  if num == 63 then cube(1750, -750, 89) end
  if num == 64 then cube(1800, 500, 89) end
  -- button 12
  if num == 65 then cube(2500, 800, 339) end
  if num == 66 then cube(2500, 250, 589) end
  if num == 67 then cube(2650, -350, 839) end
  -- button 13
  if num == 68 then cube(350, 1500, 964) end
  if num == 69 then cube(925, 1400, 964) end
  -- button 14
  if num == 70 then cube(215, 2500, 89) end
  -- button 15
  if num == 71 then cube(-1450, 2075, 89) end
  if num == 72 then cube(-225, 1850, 39) end
  if num == 73 then cube(-1000, 1350, 839) end
  -- button 16
  if num == 74 then cube(-2800, 850, 214) end
  if num == 75 then cube(-3125, 975, 839) end
  -- button 17
  if num == 76 then cube(-2100, 1850, 89) end
  if num == 77 then cube(-1800, 1750, 839) end
  if num == 78 then cube(-1850, 2350, 964) end
  -- button 18
  if num == 79 then cube(-4475, -1625, 589) end
  if num == 80 then cube(-4450, -1900, 589) end
  if num == 81 then cube(-5000, -2000, 589) end
  if num == 82 then cube(-4950, -1600, 589) end
  if num == 83 then cube(-5550, -1850, 464) end
  if num == 84 then cube(-4950, -2550, 339) end
  if num == 85 then cube(-3700, -2200, 89) end
  if num == 86 then cube(-3675, -2750, 39) end
  if num == 87 then cube(-3700, -3150, 89) end
  -- button 19
  if num == 88 then cube(1300, -2900, 89) end
  if num == 89 then cube(1250, -4500, 89) end
  if num == 90 then cube(2000, -4650, 214) end
  if num == 91 then cube(2025, -4200, 714) end
  -- button 20
  if num == 92 then cube(2400, -3100, 214) end
  if num == 93 then cube(3250, -2850, 214) end
  if num == 94 then cube(3800, -3200, 89) end
  if num == 95 then cube(3750, -2150, 339) end
  if num == 96 then cube(4300, -1600, 214) end
  if num == 97 then cube(4800, -1725, 89) end
  if num == 98 then cube(4300, -2600, 589) end
  if num == 99 then cube(4750, -2050, 839) end
  if num == 100 then cube(4850, -1250, 714) end
  -- button 21
  if num == 101 then cube(4625, -2775, 964) end
  if num == 102 then cube(4400, -3000, 214) end
  if num == 103 then cube(4900, -3425, 839) end
  if num == 104 then cube(4375, -3500, 714) end
  if num == 105 then cube(4600, -4000, 89) end
  if num == 106 then cube(4900, -3850, 214) end
  if num == 107 then cube(4400, -4650, 214) end
  if num == 108 then cube(3900, -4550, 89) end
  if num == 109 then cube(3500, -45750, 339) end
  if num == 110 then cube(3500, -4100, 589) end
  if num == 111 then cube(2950, -4000, 714) end
  if num == 112 then cube(2625, -4262, 464) end
  -- button 22
  if num == 113 then cube(3000, 500, 89) end
  -- button 23
  if num == 114 then cube(1750, 1500, 89) end
  if num == 115 then cube(2125, 1750, 839) end
  -- button 24
  if num == 116 then cube(1900, 2400, 89) end
  if num == 117 then cube(1500, 3400, 89) end
  if num == 118 then cube(950, 3850, 214) end
  if num == 119 then cube(750, 3350, 339) end
  if num == 120 then cube(725, 4550, 89) end
  if num == 121 then cube(550, 3900, 464) end
  -- button 25
  if num == 122 then cube(1150, 4750, 839) end
  if num == 123 then cube(1225, 4200, 589) end
  if num == 124 then cube(1700, 3850, 464) end
  if num == 125 then cube(2200, 4100, 89) end
  if num == 126 then cube(2500, 4600, 39) end
  if num == 127 then cube(2000, 5150, 89) end
  if num == 128 then cube(3000, 2500, 214) end -- don't know when this went up
  if num == 129 then cube(3150, 1950, 89) end -- don't know when this went up
  if num == 130 then cube(1800, 4600, 964) end
  -- button 26
  if num == 131 then cube(3700, 5600, 89) end
  if num == 132 then cube(2850, 5600, 89) end
  if num == 133 then cube(2500, 6400, 89) end
  if num == 134 then cube(2250, 5800, 89) end
  if num == 135 then cube(1800, 6350, 89) end
  if num == 136 then cube(1600, 5850, 89) end
  if num == 137 then cube(700, 6350, 89) end
  if num == 138 then cube(1000, 5575, 89) end
  if num == 139 then cube(350, 5600, 89) end
  if num == 140 then cube(100, 6100, 89) end
  if num == 141 then cube(-350, 5850, 89) end
  -- button 27
  if num == 142 then cube(-1100, 450, 89) end
  if num == 143 then cube(-1500, 3800, 89) end
  if num == 144 then cube(-800, 3300, 214) end
  if num == 145 then cube(-250, 3350, 89) end
  if num == 146 then cube(-800, 4000, 464) end
  if num == 147 then cube(-450, 4700, 714) end
  -- button 28
  if num == 148 then cube(-3075, 2375, 714) end
  if num == 149 then cube(-4125, 1900, 839) end
  if num == 150 then cube(-3475, 1750, 1089) end
  if num == 151 then cube(-3750, -1075, 964) end
  if num == 152 then cube(-3500, 375, 964) end
  if num == 153 then cube(-4000, 0, 589) end
  if num == 154 then cube(-4125, 500, 714) end
  if num == 155 then cube(-4750, 750, 964) end
  if num == 156 then cube(-5275, 1075, 839) end
  if num == 157 then cube(-4750, 100, 1089) end
  if num == 158 then cube(-5650, -750, 714) end
  if num == 159 then cube(-5150, -400, 89) end
  if num == 160 then cube(-4700, 550, 89) end
  if num == 161 then cube(-4575, 1750, 89) end
  if num == 162 then cube(-3500, 1550, 89) end
  if num == 163 then cube(-3500, 550, 89) end
  if num == 164 then cube(-4400, 1250, 1214) end
  -- button 29
  if num == 165 then cube(3500, 2900, 89) end
  if num == 166 then cube(3500, 3550, 89) end
  if num == 167 then cube(3050, 4150, 1839) end
  if num == 168 then cube(4275, 3125, 89) end
  if num == 169 then cube(4750, 3750, 89) end
  if num == 170 then cube(5250, 3350, 89) end
  if num == 171 then cube(5350, 3000, 214) end
  if num == 172 then cube(5625, 2500, 214) end
  if num == 173 then cube(5000, 1800, 339) end
  if num == 174 then cube(4450, 1900, 339) end
  if num == 175 then cube(4075, 2350, 339) end
  if num == 176 then cube(3750, 2500, 214) end
  if num == 177 then cube(4400, 2750, 214) end
  if num == 178 then cube(4900, 3000, 214) end
  if num == 179 then cube(4500, 2275, 464) end
  -- button 30
  if num == 180 then cube(4900, 1050, 89) end
  if num == 181 then cube(4500, 150, 89) end
  if num == 182 then cube(4500, -725, 89) end
  if num == 183 then cube(3750, -1250, 89) end
  if num == 184 then cube(4600, -1000, 404) end
  if num == 185 then cube(4075, -500, 839) end
  if num == 186 then cube(3700, -650, 714) end
  if num == 187 then cube(3750, 0, 714) end
  if num == 188 then cube(3300, -350, 589) end
  if num == 189 then cube(4150, 0, 589) end
  if num == 190 then cube(3800, 650, 89) end
  if num == 191 then cube(3775, 200, 214) end
  -- button 31
  if num == 192 then cube(2875, -2200, 89) end
  if num == 193 then cube(2900, -2750, 1089) end
  if num == 194 then cube(2125, -2725, 964) end
  if num == 195 then cube(1850, -3000, 714) end
  -- uncategorized
  if num == 196 then cube(0, -1900, 589) end
  if num == 197 then cube(725, -750, 589) end
  if num == 198 then cube(-1100, 4800, 89) end
  if num == 199 then cube(-3750, 1150, 964) end
  if num == 200 then cube(-4675, 1750, 89) end
  if num == 201 then cube(3500, -4750, 339) end
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
