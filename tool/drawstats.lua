require 'prelude'

function drawhud()
  local x,y,z = getlocation()
  local velx, vely, velz = getvelocity()
  local pitch, yaw, roll = getrotation()
  local accx, accy, accz = getacceleration()
  local s = string.format("x: %6.2f    y: %6.2f    z: %6.2f\nvelx: %6.2f    vely: %6.2f    velz: %6.2f\nvelxy: %6.2f\nvelxyz: %6.2f\npitch: %6.2f    yaw: %6.2f    roll: %6.2f",
  x, y, z, velx, vely, velz, math.sqrt(velx*velx + vely*vely), math.sqrt(velx*velx + vely*vely + velz*velz), pitch, yaw, roll)
  local size = 2;
  drawtext(s, 0, 0, 0, 1, 0, 0, size, false)
end

while true do
  step()
end
