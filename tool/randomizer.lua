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

local dependants = {{}}
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

randomizer.difficulties = { "beginner", "intermediate", "advanced" }
-- randomizer.queue[1] is always the current running randomizer, while randomizer.queue[2] is always the planned next randomizer. Indexes 3 and beyond are optional
randomizer.queue = {{seed = ""}, {seed = "", difficulty = randomizer.difficulties[1]}}
randomizer.newgamenewseed = "Auto"
randomizer.newgamenewseedvalues = { "On", "Off", "Auto" }
randomizer.newgamenewseedui = { ["On"] = "Always ON", ["Off"] = "Always OFF", ["Auto"] = "Auto (ON for Random Seed / OFF for Set Seed)" }

local levelsequence
local levelindex = 1

function randomizer.hudlines()
    local currentseedline
    local nextseedline = "Next: " .. randomizer.queue[2].difficulty .. " "
    local progressline = ""

    if randomizer.queue[1].seed == "" then -- If the user isn't currently playing a randomizer
        currentseedline = "Press New Game to start"
    else
        currentseedline = "Current: " .. randomizer.queue[1].difficulty .. " " .. randomizer.queue[1].seedtype .. ": " .. randomizer.queue[1].seed
        currentseedline = currentseedline .. "   Progress " .. levelindex - 2 .. "/" .. #levelsequence + 1
    end

    if randomizer.queue[2].seed == "" then -- If the next seed will be random
        nextseedline = nextseedline .. randomizer.SEEDTYPE.RANDOMSEED
    else
        if randomizer.queue[1].seed == randomizer.queue[2].seed then -- If the next seed is the same as the current one
            nextseedline = nextseedline .. "Keep current seed"
        else
            nextseedline = nextseedline .. randomizer.queue[2].seedtype .. ": " .. randomizer.queue[2].seed
        end
    end

    local randomizerlines = {currentseedline, nextseedline}
    return randomizerlines
end

local function nextlevel()
    if levelindex <= #levelsequence then
        tas:set_level(levelsequence[levelindex])
    end
    levelindex = levelindex + 1
end

function randomizer.randomize()
    table.remove(randomizer.queue, 1)
    if randomizer.queue[1].seed == "" then
        randomizer.queue[1].seed = os.time() .. math.floor(os.clock()*10000)
        if randomizer.newgamenewseed == "Off" then
            table.insert(randomizer.queue, randomizer.queue[1])
        else
            table.insert(randomizer.queue, {seed = "", seedtype = randomizer.SEEDTYPE.RANDOMSEED, difficulty = randomizer.queue[1].difficulty})
        end
    else
        if #randomizer.queue == 1 then
            if randomizer.newgamenewseed == "On" then
                table.insert(randomizer.queue, {seed = "", seedtype = randomizer.SEEDTYPE.RANDOMSEED, difficulty = randomizer.queue[1].difficulty})
            else
                table.insert(randomizer.queue, randomizer.queue[1])
            end
        end
    end

    math.randomseed(tonumber(randomizer.queue[1].seed))

    if dependencies[randomizer.queue[1].difficulty] == nil then
        error("difficulty is not advanced, intermediate or beginner")
    end

    local levels = {}
    local workingset = {}
    local visited = { 0 }
    levelsequence = {}
    levelindex = 1

    for i=2,30 do
        if dependencies[randomizer.queue[1].difficulty][i] == nil then
            table.insert(workingset, i)
        end
    end

    while #workingset ~= 0 do
        local newlevelindex = math.random(#workingset)
        local newlevel = workingset[newlevelindex]
        table.insert(visited, newlevel)
        table.remove(workingset, newlevelindex)
        for _, nowvalid in pairs(dependants[randomizer.queue[1].difficulty][newlevel] or {}) do
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
    randomizer.queue = {{seed = ""}, {seed = "", difficulty = randomizer.difficulties[1]}}
end

return randomizer
