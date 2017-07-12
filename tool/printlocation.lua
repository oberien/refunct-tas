require "prelude"

while true do
  stats = getplayerstats()
  print(stats.x, stats.y, stats.z)
  step()
end
