require "prelude"

waitfornewgame()
setdelta(1/60)

local stats = getplayerstats()
print(stats.pitch, stats.yaw, stats.roll)

function rotate(degx, degy, frames)
  for i=1,frames do
    local stats = getplayerstats();
    setrotation(stats.pitch + degy / frames, stats.yaw + degx / frames, stats.roll)
    frame({}, 0, 0, 1)
  end
end

rotate(720, 0, 120)
rotate(-720, 0, 120)
rotate(-90, 0, 30)
rotate(-30, 0, 20)
rotate(60, 0, 20)
rotate(-60, 0, 20)
rotate(30, 0, 20)

rotate(0, 20, 20)
rotate(0, -40, 20)
rotate(0, 40, 20)
rotate(0, -20, 20)
rotate(0, 90, 20)
rotate(0, -180, 20)

local stats = getplayerstats()
print(stats.pitch, stats.yaw, stats.roll)

