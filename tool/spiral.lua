require "prelude"
require "teleportbutton-prelude"

while true do
  waitfornewgame()
  setdelta(1/60)
  teleportbutton(19)
  setrotation(0, 0, 0)
  setlocation(-1065, -3842, 464)
  setdelta(0)
end
