require "prelude"

waitfornewgame()
setdelta(1/60)

local stats = getplayerstats()
print(stats.pitch, stats.roll, stats.yaw)
frame({}, 720, 0, 120)
frame({}, -720, 0, 120)
frame({}, -90, 0, 30)
frame({}, -30, 0, 20)
frame({}, 60, 0, 20)
frame({}, -60, 0, 20)
frame({}, 30, 0, 20)

frame({}, 0, 20, 20)
frame({}, 0, -40, 20)
frame({}, 0, 40, 20)
frame({}, 0, -20, 20)
frame({}, 0, 90, 20)
frame({}, 0, -180, 20)

local stats = getplayerstats()
print(stats.pitch, stats.roll, stats.yaw)

