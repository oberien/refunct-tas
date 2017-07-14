require "prelude"

while true do
  stats = getplayerstats()
  print(string.format("x: %-6.2f\ty: %-6.2f\tz: %-6.2f\tvelx: %-6.2f\tvely: %-6.2f\tvelz: %-6.2f\tpitch: %-6.2f\tyaw: %-6.2f\taccx: %-6.2f\taccy: %-6.2f\t",
		stats.x, stats.y, stats.z, stats.velx, stats.vely, stats.velz, stats.pitch, stats.yaw, stats.accx, stats.accy))
  step()
end
