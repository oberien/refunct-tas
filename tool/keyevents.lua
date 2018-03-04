require "prelude"

onkeydown = function(key, code, _repeat)
  print("onkeydown", key, code, _repeat)
end
onkeyup = function(key, code, _repeat)
  print("onkeyup", key, code, _repeat)
end

tas:press_key(forward)
tas:release_key(forward)

while true do
  waitfornewgame()
end
