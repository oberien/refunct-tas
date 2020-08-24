require "prelude"
ui = require "ui"

local randomizer = {}

local dependencies = {}
dependencies.advanced = {
    [13] = { 2, 3, 10, 11, 14, 15, 23, 24, 27 },
    [22] = { 3, 10, 11, 12, 20, 30 },
}
dependencies.intermediate = {
    [13] = { 3, 11, 14, 15, 23, 24, 27 },
    [18] = { 8 },
    [22] = { 3, 11, 12, 20, 30 },
}
dependencies.beginner = {
    [13] = { 3, 11, 14, 15, 23, 24, 27 },
    [16] = { 2, 17, 28 },
    [18] = { 8 },
    [22] = { 3, 11, 12, 20, 30 },
}

local dependants = {}
for proficiency, deps in pairs(dependencies) do
    dependants[proficiency] = {}
    for level, requirements in pairs(deps) do
        for _, requirement in ipairs(requirements) do
            local list = dependants[proficiency][requirement] or {}
            table.insert(list, level)
            dependants[proficiency][requirement] = list
        end
    end
end

randomizer.KINDS = {
    UNSEEDED = {},
    SEEDED = {},
}

randomizer.kind = {}
randomizer.seed = ""
randomizer.proficiency = "beginner"
randomizer.proficiencies = { "beginner", "intermediate", "advanced" }
randomizer.newgamenewseed = false

local levelsequence
local levelindex

function randomizer.drawhud()
    local info = {}
    table.insert( info, "Randomizer " .. randomizer.proficiency .. " Seed " .. tostring(randomizer.seed) )
    if levelindex > 1 then
        table.insert( info, "Progress " .. levelindex - 2 .. "/" .. #levelsequence + 1 )
    end
    ui.drawlines(info)
end

local function nextlevel()
    if levelindex <= #levelsequence then
        tas:set_level(levelsequence[levelindex])
    end
    levelindex = levelindex + 1
end

function randomizer.new()
    if randomizer.kind == randomizer.KINDS.UNSEEDED then
        randomizer.seed = ""
        randomizer.randomize()
    elseif randomizer.kind == randomizer.KINDS.SEEDED then
        local seed = nil
        local error = ""
        while type(seed) ~= "number" do
            local input = ui.input(error .. "Input Seed", randomizer.seed)
            seed = tonumber(input)
            error = "Invalid Number. "
        end
        randomizer.seed = seed
        randomizer.randomize()
    else
        error("invalid randomizer kind (internal error)")
    end
end

function randomizer.randomize()
    if not randomizer.seed or randomizer.seed == "" then
        randomizer.seed = os.time() .. math.floor(os.clock()*10000)
    end
    math.randomseed(tonumber(randomizer.seed))

    if dependencies[randomizer.proficiency] == nil then
        error("proficiency is not advanced, intermediate or beginner")
    end

    local levels = {}
    local workingset = {}
    local visited = { 0 }
    levelsequence = {}
    levelindex = 1

    for i=2,30 do
        if dependencies[randomizer.proficiency][i] == nil then
            table.insert(workingset, i)
        end
    end

    while #workingset ~= 0 do
        local newlevelindex = math.random(#workingset)
        local newlevel = workingset[newlevelindex]
        table.insert(visited, newlevel)
        table.remove(workingset, newlevelindex)
        for _, nowvalid in pairs(dependants[randomizer.proficiency][newlevel] or {}) do
            if not table.contains(visited, nowvalid) and not table.contains(workingset, nowvalid) then
                table.insert(workingset, nowvalid)
            end
        end
        table.insert(levelsequence, newlevel - 2)
    end
    table.insert(levelsequence, 31 - 2)

    _G.onlevelchange = function(level)
        if level > 0 then
            nextlevel()
        end
    end

    _G.onreset = function()
        levelindex = 1
        nextlevel()
    end
end

function randomizer.reset()
    _G.onlevelchange = nil
    _G.onreset = nil
    randomizer.kind = {}
end

return randomizer