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
    KEEPSEED = "Keep current seed",
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

local levelsequence
local levelindex = 1
local difficulties = { "beginner", "intermediate", "advanced" }
-- queue[1] is always the current running randomizer
-- queue[2] is always the planned next randomizer
-- queue[3] and beyond are optional
local queue = {{seed = ""}, {seed = "", difficulty = difficulties[1]}}
-- nextseedindex is usually at 2 except when anything is being removed from the queue
local nextseedindex = 2
local newgamenewseed = "Auto"

function randomizer.hudlines()
    local currentseedline
    local nextseedline = "Next: " .. queue[2].difficulty .. " "
    local progressline = ""

    if queue[1].seed == "" then -- If the user isn't currently playing a randomizer
        currentseedline = "Press New Game to start"
    else
        currentseedline = "Current: " .. queue[1].difficulty .. " " .. queue[1].seedtype .. ": " .. queue[1].seed
        currentseedline = currentseedline .. "   Progress " .. levelindex - 2 .. "/" .. #levelsequence + 1
    end

    if queue[2].seed == "" then -- If the next seed will be random
        nextseedline = nextseedline .. randomizer.SEEDTYPE.RANDOMSEED
    else
        nextseedline = nextseedline .. queue[2].seedtype
        if queue[2].seedtype ~= randomizer.SEEDTYPE.KEEPSEED then
            nextseedline = nextseedline .. ": " .. queue[2].seed
        end
    end

    return {currentseedline, nextseedline}
end

function randomizer.getnewgamenewseed()
    local newgamenewseedui = {
        ["On"] = "Always ON",
        ["Off"] = "Always OFF",
        ["Auto"] = "Auto (ON for Random Seed / OFF for Set Seed)"
    }
    return newgamenewseedui[newgamenewseed]
end

function randomizer.cyclenewgamenewseed()
    local newgamenewseedvalues = { "On", "Off", "Auto" }
    local index = table.indexof(newgamenewseedvalues, newgamenewseed)
    index = ((index - 1 + 1) % #newgamenewseedvalues) + 1
    newgamenewseed = newgamenewseedvalues[index]
end

function randomizer.getdifficulty()
    return queue[2].difficulty
end

function randomizer.cycledifficulty()
    local index = table.indexof(difficulties, queue[2].difficulty)
    index = ((index - 1 + 1) % #difficulties) + 1
    queue[2].difficulty = difficulties[index]
end

function randomizer.setnextseed(seedtype, seed)
    seed = seed or ""
    local difficulty = queue[nextseedindex].difficulty
    queue = {queue[1], {seedtype = seedtype, difficulty = difficulty}}
    if seedtype == randomizer.SEEDTYPE.RANDOMSEED then
        queue[2].seed = ""
    elseif seedtype == randomizer.SEEDTYPE.SETSEED then
        if seed == "" then
            error("Cannot assign empty seed with SETSEED")
        end
        queue[2].seed = seed
    elseif seedtype == randomizer.SEEDTYPE.KEEPSEED then
        queue[2].seed = queue[1].seed
    else
        error("seedtype is not RANDOMSEED, SETSEED or KEEPSEED")
    end
end

function randomizer.setnextseedwithlogic()
    -- Update the next seed depending on newgamenewseed
    if newgamenewseed == "On" and #queue == nextseedindex then
        randomizer.setnextseed(randomizer.SEEDTYPE.RANDOMSEED)
    elseif queue[1].seed ~= "" then
        if newgamenewseed == "Off" then
            randomizer.setnextseed(randomizer.SEEDTYPE.KEEPSEED)
        elseif newgamenewseed == "Auto" then
            if queue[1].seedtype == randomizer.SEEDTYPE.RANDOMSEED then
                randomizer.setnextseed(randomizer.SEEDTYPE.RANDOMSEED)
            elseif queue[1].seedtype == randomizer.SEEDTYPE.SETSEED then
                if #queue == nextseedindex then
                    randomizer.setnextseed(randomizer.SEEDTYPE.KEEPSEED)
                end
            else
                error("Current seedtype is not RANDOMSEED or SETSEED")
            end
        else
            error("newgamenewseed is not On, Off or Auto")
        end
    end
end

local function nextlevel()
    if levelindex <= #levelsequence then
        tas:set_level(levelsequence[levelindex])
    end
    levelindex = levelindex + 1
end

function randomizer.randomize()
    if queue[2].seedtype == randomizer.SEEDTYPE.KEEPSEED then
        queue[2].seedtype = queue[1].seedtype
    end
    table.remove(queue, 1)
    nextseedindex = 1
    if #queue == 1 then -- If there is no seed planned for the next game
        if queue[1].seed == "" then -- If there is no seed planned for this game
            queue[1].seed = os.time() .. math.floor(os.clock()*10000)
        end
        randomizer.setnextseedwithlogic()
    end
    nextseedindex = 2

    math.randomseed(tonumber(queue[1].seed))

    if dependencies[queue[1].difficulty] == nil then
        error("difficulty is not advanced, intermediate or beginner")
    end

    local levels = {}
    local workingset = {}
    local visited = { 0 }
    levelsequence = {}
    levelindex = 1

    for i=2,30 do
        if dependencies[queue[1].difficulty][i] == nil then
            table.insert(workingset, i)
        end
    end

    while #workingset ~= 0 do
        local newlevelindex = math.random(#workingset)
        local newlevel = workingset[newlevelindex]
        table.insert(visited, newlevel)
        table.remove(workingset, newlevelindex)
        for _, nowvalid in pairs(dependants[queue[1].difficulty][newlevel] or {}) do
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
    queue = {{seed = ""}, {seed = "", difficulty = difficulties[1]}}
end

return randomizer
