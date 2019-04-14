require "prelude-virtual"

-- cd desktop\tas
-- refunct-tas.exe TAS.lua
-- frame({}, , , )
-- printstats()

setdelta(1/60)
waitfornewgame()

--[ Turn before the first SJ.
frame({forward}, 59, 10, 214)
frame({forward}, 0, 0, 2)

-- Button 1 + turn to jump.
frame({forward, jump}, 0, 0, 79)
frame({forward}, -9, 0, 21)
frame({forward}, -3, 0, 2)
frame({forward}, -6, 0, 4)

-- Button 2 + turn.
frame({forward, jump}, 0, 0, 72)
frame({forward, left}, -132, 0, 26)

-- Walk to Button 3.
frame({forward}, 0, 0, 8)
frame({forward, jump}, 0, 0, 2)
frame({forward}, 0, 0, 78)
frame({forward, jump}, 0, 0, 2)
frame({forward}, 0, 0, 68)
frame({forward, jump}, 0, 0, 2)
frame({forward}, 0, 0, 44)
frame({forward}, -8, -9, 9)
frame({forward}, 0, 0, 8)

-- Jump and turnaround on 3.
frame({forward, jump}, 0, 0, 75)
frame({forward}, 0, 0, 3)
frame({forward, jump}, 0, 0, 16)
frame({forward}, -163, 15, 13)

-- Walk to Button 4.
frame({forward}, 0, 0, 151)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 45)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 126)
frame({forward}, -2, 0, 12)
frame({forward}, 0, 0, 12)

-- Slide extend onto 4.
frame({forward, jump}, 0, 0, 1)
frame({forward}, -7, 0, 30)
frame({forward, crouch}, 0, 0, 20)
frame({forward, crouch, jump}, 0, 0, 1)
frame({forward, crouch}, 0, 0, 20)
frame({forward, crouch, jump}, 0, 0, 1)
frame({forward, crouch}, 0, 0, 30)
frame({forward, jump}, 0, 0, 1)

-- Jump to and turn on Button 5.
frame({forward}, 0, 0, 45)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 87)
frame({forward, right}, 156, 0, 30)

-- Getting to and onto the elevator.
frame({forward}, 4, 0, 10)
frame({forward}, 0, 0, 55)
frame({forward, jump}, 0, 0, 65)
frame({forward, jump}, -10, 0, 12)
frame({forward}, 0, 0, 5)
frame({forward, jump}, 2, 0, 50)
frame({forward}, 1, 0, 18)

-- Turning, jumping off elevator, getting onto Button 6.
frame()
frame({forward, left}, -148, -12, 25)
frame({forward, jump}, 0, 6, 45)
frame({forward}, 0, 0, 2)

-- Jumping to Buttons 7 and 8 and getting both.
frame({forward}, 21, 3, 10)
frame({forward}, 0, 0, 154)
frame({forward, jump}, 0, 0, 6)
frame({forward}, 0, 0, 4)
frame({forward, right}, 70, 5, 30)
frame({forward, right}, 81, 0, 16)
frame({forward}, 0, -30, 37)

-- Timing the elevator to Button 9.
frame({jump}, 0, 0, 1)
frame({}, 0, -40, 90)
frame({}, 0, 0, 62)
frame({forward}, 0, 60, 34)
frame({forward, jump}, 0, 0, 6)
frame({forward, jump}, -40, 0, 20)
frame({forward}, 0, 0, 17)
frame({forward}, 40, 20, 19)
frame({forward, right}, 6, 10, 3)
frame({forward, right, jump}, 0, 0, 18)

-- Going from 9 to 10.
frame({forward, right}, 2, -1, 6)
frame({forward}, 2, -2, 6)
frame({forward}, 3, -18, 56)
frame({forward, jump}, 0, 0, 113)
frame({forward, right}, 30, 0, 14)
frame({forward}, -39, -10, 10)
frame({forward, right}, 180, 0, 54)
frame({forward}, 0, 0, 24)
frame({forward, left}, -8, 0, 4)

-- D I V E + Button 11.
frame({forward, left}, -169, 50, 70)
frame({forward}, 0, 0, 14)
frame({forward, crouch}, 0, 0, 2)
frame({forward, jump}, -8, -65, 80)
frame({forward}, 0, 0, 6)

-- Pipejump + Button 12.
frame({forward, jump}, 0, 0, 23)
frame({forward}, 6, 0, 26)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 5, 0, 53)
frame({forward, jump}, -12, 0, 30)
frame({forward}, 0, 0, 30)
frame({forward, right}, 98, 0, 25)

-- Going to Button 13.
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 60)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 80)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 70)
frame({forward, jump}, 0, 0, 31)
frame({forward, left}, -160, 0, 30)

-- Into and out of the pit + Button 14.
frame({forward}, -10, 0, 20)
frame({forward}, -10, 0, 27)
frame({forward, crouch}, 0, 20, 16) -- Slide in.
frame({crouch}, -180, 0, 22)
frame({forward}, 0, -33, 36)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 1)
frame({forward, left, jump}, 0, 0, 1)
frame({forward, left}, 0, 0, 9)
frame({forward, right, jump}, 0, 0, 1)
frame({forward, right}, 0, 0, 9)
frame({forward, left, jump}, 0, 0, 1)
frame({forward, left}, 10, 0, 9)
frame({forward, right, jump}, 26, 23, 20) -- Last walljump and climb out.

-- Corner jump to Button 15.
frame({forward}, 0, 0, 20)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 55)
frame({forward}, 0, 0, 35)
frame({forward}, 0, 0, 37)
frame({forward, jump}, 0, 0, 1)
frame({forward}, -22, 0, 34)
frame({forward, right, jump}, 0, 0, 1)
frame({forward, right}, 0, 0, 10)
frame({forward, right}, 40, 0, 15)
frame({forward, right}, 0, 0, 15)
frame({forward, right}, 0, 0, 9)
frame({forward, right, jump}, 0, 0, 1)
frame({forward}, 19, 0, 57)

-- 16.
frame({forward}, 15, 0, 20)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 8, 0, 11)
frame({forward}, 0, 0, 27)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 35)
frame({forward, right}, 110, -15, 31)

-- 17.
frame({forward}, 0, 0, 25)
frame({forward, jump}, -5, 0, 25)
frame({forward, left}, -88, 0, 15)

-- 18 (90).
frame({forward}, 0, 0, 11)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 40, 40)
frame({forward, jump}, 0, 0, 1)
frame({forward}, -9, 0, 36)
frame({forward}, -4, 0, 38)
frame({forward}, -1, 0, 6)
frame({forward, left, jump}, -16, -50, 25)
frame({forward, jump}, 0, 0, 20)
frame({forward}, -36, 0, 10)
frame({forward}, 0, 0, 2)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 16)
frame({forward, right, jump}, 0, 0, 1)
frame({forward, right}, -3, 0, 13)
frame({forward, left, jump}, 0, 0, 1)
frame({forward, left}, -10, 0, 9)
frame({forward, right, jump}, 0, 0, 1)
frame({forward}, -25, 0, 9)
frame({forward, jump}, -60, 40, 19)
frame({forward, jump}, -2, 0, 16)
frame({forward, left, jump}, -35, 0, 10)
frame({forward, jump}, -36, 0, 10)

-- 19.
frame({forward, jump}, 0, 0, 40)
frame({forward, left, jump}, -97.1, 0, 12)

-- 20.
frame({forward}, 0, 0, 200)
frame({forward, crouch}, 0, 0, 1)
frame({forward}, 0, 0, 9)
frame({forward}, -5, 0, 20)
frame({forward}, 0, 0, 6)
frame({forward, jump}, 0, 0, 30)
frame({forward}, -10, 0, 8)
frame({forward, left}, -10, 0, 8)
frame({forward}, 0, 0, 5)
frame({forward, crouch}, 0, 0, 7)
frame({forward, jump}, 0, 0, 10)
frame({forward, jump}, -11, 0, 10)
frame({forward, jump}, 0, 0, 80)
frame({forward, right}, 90, 0, 19)

-- 21.
frame({forward, crouch}, 0, 0, 45)
frame({forward, jump}, 0, 0, 31)
frame({forward, right, jump}, 33, -16, 30)
frame({forward, jump}, 0, 0, 34)
frame({forward}, 0, 0, 20)
frame({forward, jump}, 0, 0, 14)
frame({forward}, -2, 0, 10)
frame({forward}, 0, 0, 34)
frame({forward, jump}, 0, 0, 12)
frame({forward, right}, 39, 0, 9)
frame({forward}, 0, 0, 7)
frame({forward, jump}, 0, 0, 12)
frame({forward}, 0, 0, 7)
frame({forward, left}, -158, 0, 27)
frame({forward, jump}, 0, 0, 46)
frame({forward, right}, 166, 0, 28)

-- Leap of Faith.
frame({forward, right}, 0, 0, 3)
frame({forward}, 0, 0, 146)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 60)
frame({forward, jump}, 0, 0, 1)
frame({forward, right}, 7, 0, 10)
frame({forward}, 0, 0, 106)
frame({forward, right}, 8, 0, 8)
frame({forward}, -19, 0, 6)
frame({forward}, 0, 0, 41)
frame({forward, jump}, 0, 30, 125)
frame({forward}, -5, 0, 14)
frame({forward, left}, 0, 0, 5)

-- 22 + Spiral.
frame({forward, right}, 44, 10, 8)
frame({forward}, 0, 0, 9)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 75)
frame({forward}, -15, -40, 23)
frame({forward, jump}, 0, 0, 100)

-- 23.
frame({forward}, 0, 0, 25)
frame({forward, jump}, 0, 0, 8)
frame({forward, jump}, 50, 0, 8)
frame({forward}, 87, 0, 11)
frame({forward, jump}, 0, 0, 43)
frame({forward, right}, 69, 20, 21)

-- 24.
frame({forward}, 0, 0, 95)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 48)
frame({crouch}, 0, 0, 10)
frame({crouch}, -139, 0, 30)

-- 25.
frame({forward}, 0, 0, 47)
frame({forward}, -21, 0, 10)
frame({forward}, 0, 0, 40)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 87)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 30, 0, 38)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 2, 0, 4)
frame({forward}, 0, 0, 30)
frame({forward, jump}, 0, 0, 60)

-- 26.
frame({forward}, 0, 0, 50)
frame({forward, jump}, 0, 0, 65)
frame({forward}, 0, 0, 10)
frame({forward, jump}, 30, 0, 20)
frame({forward}, -10, 0, 30)
frame({forward}, 0, 0, 20)
frame({forward, left, jump}, -30, 0, 10)
frame({forward, left}, 0, 0, 12)
frame({forward}, 0, 0, 18)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 30)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 20, 0, 30)
frame({forward}, 0, 0, 14)
frame({forward}, 34, 0, 20)
frame({forward}, 0, 0, 13)

-- 27.
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 86)
frame({forward, jump}, -8, 0, 75)
frame({forward}, 0, 0, 14)
frame({forward, jump}, 0, 0, 1)
frame({forward, left}, -70, 0, 76)
frame({forward, left}, -55, 0, 25)
frame({forward, left, jump}, -10, 0, 5)

-- 28.
frame({forward}, -1, 0, 10)
frame({forward}, 0, 0, 105)
frame({forward, jump}, 0, 0, 1)
frame({forward}, -1, 0, 185)
frame({forward, right}, 137, 0, 30)

-- 29-31.
frame({forward}, 0, 0, 30)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 80)
frame({forward, jump}, 0, 0, 22) -- 29.
frame({forward}, 27, 0, 7)
frame({forward}, 0, 0, 14)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 60)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 16, 0, 36)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 5, 0, 25) -- 30.
frame({forward}, 14, 0, 40)
frame({forward}, 0, 0, 44)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 52)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 8, 0, 56)
frame({forward, jump}, 0, 0, 4)
frame({forward}, 50, 0, 25) -- 31.

-- 32.
frame({forward}, 0, 0, 15)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 65) --]]

--[[ THIS IS TO TELEPORT RIGHT IN FRONT OF BUTTON 33

teleportbutton(27)
setlocation(-1105.95, 4662.04, 89.15)
setrotation(-21.00, -103.10)
setvelocity(-187.00, -805.10, 0.00) --]]

frame({forward}, 0, 0, 15)  
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 43)
frame({forward, jump}, 0, 0, 1)
frame({forward}, -3, 0, 74)
frame({forward, left}, -10, 0, 6)
frame({forward}, -15, 0, 12)

-- 33.
frame({forward}, 0, 0, 10)
frame({forward, jump}, 0, 0, 1)
frame({forward}, -6, 0, 30)
frame({forward}, 0, 0, 31) -- stupid jump
frame({forward, jump}, 0, 0, 68)
frame({forward}, -28, 0, 6)
frame({forward}, 0, 0, 36)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 30)
frame({forward, jump}, 0, 0, 1)
frame({forward, right}, -30, 0, 28)
frame({forward, left}, 0, 0, 5)
frame({forward}, 0, 0, 10)
frame({forward, left, jump}, -20, 0, 30)
frame({forward}, -4, 0, 20)
frame({forward}, 0, 0, 10)
frame({forward, jump}, 0, 0, 28)

-- 34.
frame({forward, jump}, 129, 0, 11)
frame({forward}, 0, 0, 6)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 66)
frame({forward}, -6, 0, 24)
frame({forward, jump}, -7, 0, 16)
frame({forward, left}, -24, 0, 16)
frame({forward}, 0, 0, 6)
frame({forward, jump}, 0, 0, 23)
frame({forward}, 134, 0, 12)

-- 35.
frame({forward}, 0, 0, 17)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 5, 0, 70)
frame({forward}, 3, 0, 20)
frame({forward}, 0, 0, 24)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 80)
frame({forward, jump}, 0, 0, 20)
frame({forward}, 0, 0, 112)
frame({forward}, -8, 0, 10)
frame({forward}, 0, 0, 70)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 100)
frame({forward}, 10, 0, 10)
frame({forward}, 1, 0, 15)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 70)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 2, 0, 40)
frame({forward}, 11, 0, 6)
frame({forward, jump}, 0, 0, 1)
frame({forward}, -2, 0, 118)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 8)
frame({forward}, -21, 0, 16)
frame({forward, jump}, 0, 0, 20)
frame({forward}, 0, 0, 22)
frame({forward, jump}, 0, 0, 47)
frame({forward, left}, -120, 0, 24)

-- 36.
frame({forward}, 0, 0, 55)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 100)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 30)
frame({forward, crouch}, 0, 0, 10)
frame({forward, crouch, jump}, 0, 0, 1)
frame({forward, crouch}, 0, 0, 30)
frame({forward, crouch, jump}, 0, 0, 1)
frame({forward, crouch}, 0, 0, 40)
frame({forward, crouch, jump}, 0, 0, 15)

-- 37.
frame({forward}, 0, 0, 50)
frame({forward}, -15, 0, 10)
frame({forward, jump}, 0, 0, 1)
frame({forward}, 0, 0, 54)
frame({forward}, 31, -60, 10)
frame({forward, left, jump}, 0, 0, 1) -- Start final climb.
frame({forward, left}, 0, 0, 20)
frame({forward, left, jump}, 0, 0, 1)
frame({forward, left}, 0, 0, 11)
frame({forward, left, jump}, 0, 0, 1)
frame({forward, left}, 0, 0, 10)
frame({forward, left, jump}, 0, 0, 1)
frame({left}, 6, 0, 9)
frame({forward, left, jump}, 0, 0, 1)
frame({left}, 0, 0, 9)
frame({forward, jump}, 0, 0, 1)
frame({}, 0, 0, 9)
frame({forward, jump}, 0, 0, 1)
frame({}, 0, 0, 9)
frame({forward, jump}, 0, 0, 1)
frame({}, 0, 0, 9)
frame({forward, jump}, 0, 0, 14)
frame({forward}, -160, 0, 10)
frame({forward, jump}, 0, 0, 29)

-- cd desktop\tas
-- refunct-tas.exe TAS.lua
-- frame({}, , , )
-- printstats()

frame()

--]]

printframes()
