require "prelude"

waitfornewgame()
setdelta(1/60)

function printstuff()
  local pitch, yaw, roll = getrotation()
  print(pitch, yaw, roll)
end

function rotate(degx, degy, frames)
  for i=1,frames do
    local pitch, yaw, roll = getrotation();
    setrotation(pitch + degy / frames, yaw + degx / frames)
    frame({}, 0, 0, 1)
  end
  printstuff()
end

printstuff()
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

printstuff()

