require "prelude"

setdelta(1/60)
i = 1
while true do
  print(i)
  waitfornewgame()
  frame({}, 0, 0, 214)
  setlocation(-1000, -1000, 732)
  step()
  setlocation(-2000, 0, -200)
  step()
  setlocation(2125, -250, -200)
  step()
  setlocation(-2725, -875, -1040)
  -- need to wait here for some time
  --step()
  --setlocation(-5000, -875, -400)
  --step()
  --setlocation(-3250, -2250, -400)
end
