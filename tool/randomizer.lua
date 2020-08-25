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

randomizer.SEEDTYPE = {
    RANDOMSEED = "Random Seed",
    SETSEED = "Set Seed",
}

local dependants = {}
for difficulty, deps in pairs(dependencies) do
    dependants[difficulty] = {}
    for level, requirements in pairs(deps) do
        for _, requirement in ipairs(requirements) do
            local list = dependants[difficulty][requirement] or {}
            table.insert(list, level)
            dependants[difficulty][requirement] = list
        end
    end
end

-- FALSE when the hud is displaying info about the current seed, TRUE when it's displaying info about the next game's seed
randomizer.hudisfornextseed = true
-- randomizer.seedqueue[1] is the seed of the current game, while randomizer.seedqueue[2] and beyond are seeds for the next games
randomizer.seedqueue = {""}
randomizer.seedtype = ""
randomizer.difficulty = "beginner"
randomizer.difficulties = { "beginner", "intermediate", "advanced" }
randomizer.newgamenewseed = "Auto"
randomizer.newgamenewseedvalues = { "On", "Off", "Auto" }
randomizer.newgamenewseedui = { ["On"] = "ON", ["Off"] = "OFF", ["Auto"] = "Auto (Random seed: ON / Set seed: OFF)" }

local levelsequence
local levelindex = 1

function randomizer.hudlines()
    local randomizerlines = {}
    local firstline = "Randomizer " .. randomizer.difficulty .. " " .. randomizer.seedtype
    if randomizer.hudisfornextseed then
        if #randomizer.seedqueue >= 2 and randomizer.seedqueue[2] ~= "" then
            firstline = firstline .. ": " .. randomizer.seedqueue[2]
        end
    else
        firstline = firstline .. ": " .. randomizer.seedqueue[1]
    end
    table.insert(randomizerlines, firstline)
    if randomizer.hudisfornextseed then
        table.insert(randomizerlines, "Press New Game to start")
    else
        table.insert(randomizerlines, "Progress " .. levelindex - 2 .. "/" .. #levelsequence + 1)
    end
    return randomizerlines
end

local function nextlevel()
    if levelindex <= #levelsequence then
        tas:set_level(levelsequence[levelindex])
    end
    levelindex = levelindex + 1
end

function randomizer.randomize()
    if #randomizer.seedqueue == 1 then
        if
            randomizer.newgamenewseed == "On"
            or (randomizer.newgamenewseed == "Off" and randomizer.seedqueue[1] == "")
            or (randomizer.newgamenewseed == "Auto" and randomizer.seedtype == randomizer.SEEDTYPE.RANDOMSEED)
        then
            randomizer.seedqueue[1] = os.time() .. math.floor(os.clock()*10000)
            randomizer.seedtype = randomizer.SEEDTYPE.RANDOMSEED
        end
    else
        table.remove(randomizer.seedqueue, 1)
    end
    math.randomseed(tonumber(randomizer.seedqueue[1]))
    randomizer.hudisfornextseed = false

    if dependencies[randomizer.difficulty] == nil then
        error("difficulty is not advanced, intermediate or beginner")
    end

    local levels = {}
    local workingset = {}
    local visited = { 0 }
    levelsequence = {}
    levelindex = 1

    for i=2,30 do
        if dependencies[randomizer.difficulty][i] == nil then
            table.insert(workingset, i)
        end
    end

    while #workingset ~= 0 do
        local newlevelindex = math.random(#workingset)
        local newlevel = workingset[newlevelindex]
        table.insert(visited, newlevel)
        table.remove(workingset, newlevelindex)
        for _, nowvalid in pairs(dependants[randomizer.difficulty][newlevel] or {}) do
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
    randomizer.hudisfornextseed = true
    randomizer.seedqueue = {""}
end

return randomizer
