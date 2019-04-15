require "prelude"
local ui = require "ui"

local multiplayer = {}

local connected = false
--- array of player_id -> {pawn_id, x, y, z}
local players = {}

function multiplayer.connect()
  tas:tcp_connect("novalis.oberien.de:6337")
  connected = true
end

function multiplayer.disconnect()
  tas:tcp_disconnect()
  _G.tcpjoined = nil
  _G.tcpleft = nil
  _G.tcpmoved = nil
  connected = false
end

function multiplayer.join(room)
  if not connected then
    return
  end
  local x,y,z = getlocation()
  tas:tcp_join_room(room, x,y,z)
  _G.tcpjoined = function(id, x, y, z)
    print("tcpjoined", id, x, y, z)
    local pawn_id = tas:spawn_pawn()
    tas:move_pawn(pawn_id, x, y, z)
    players[id] = {pawn_id, x, y, z}
  end
  _G.tcpleft = function(id)
    print("tcpleft", id)
    tas:destroy_pawn(players[id][1])
    players[id] = nil
  end
  _G.tcpmoved = function(id, x, y, z)
    local pawn_id = players[id][1]
    tas:move_pawn(pawn_id, x, y, z)
    players[id] = {pawn_id, x, y, z}
  end
end

--- must be called in drawhud
function multiplayer.draw()
  if not connected then
    return
  end
  local x,y,z = getlocation()
  tas:tcp_move(x, y, z)
  for _,v in pairs(players) do
    local x,y,z = v[2], v[3], v[4]
    ui.draw3dcapsule(x, y, z)
  end
end

return multiplayer
