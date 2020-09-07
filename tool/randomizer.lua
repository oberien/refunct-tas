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
    RANDOMSEED = "Random seed",
    SETSEED = "Set seed",
    KEEPSEED = "Keep current seed/sequence",
    SETSEQUENCE = "Set sequence"
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
local difficulty = difficulties[1]
-- queue[1] is always the current running randomizer
-- queue[2] is always the planned next randomizer
-- queue[3] and beyond are optional
queue = {{seed = ""}, {seedtype = randomizer.SEEDTYPE.KEEPSEED}}
local newgamenewseed = "Auto"

function randomizer.hudlines()
    local currentseedline
    local nextseedline = "Next: "
    local progressline = ""

    if queue[1].seed == "" then -- If the user isn't currently playing a randomizer
        currentseedline = "Press New Game to start"
    elseif queue[1].seedtype == randomizer.SEEDTYPE.SETSEQUENCE then
        currentseedline = "Current: " .. queue[1].seedtype .. ": " .. queue[1].sequencestr
        currentseedline = currentseedline .. "   Progress " .. levelindex - 2 .. "/" .. #levelsequence + 1
    else
        currentseedline = "Current: " .. queue[1].difficulty .. " " .. queue[1].seedtype .. ": " .. queue[1].seed
        currentseedline = currentseedline .. "   Progress " .. levelindex - 2 .. "/" .. #levelsequence + 1
    end

    if queue[2].seedtype == randomizer.SEEDTYPE.KEEPSEED then
        nextseedline = nextseedline .. queue[2].seedtype
    elseif queue[2].seedtype == randomizer.SEEDTYPE.SETSEQUENCE then
        nextseedline = nextseedline .. queue[2].seedtype .. ": " .. queue[2].sequencestr
    elseif queue[2].seedtype == randomizer.SEEDTYPE.RANDOMSEED then
        nextseedline = nextseedline .. difficulty .. " " .. queue[2].seedtype
    elseif queue[2].seedtype == randomizer.SEEDTYPE.SETSEED then
            nextseedline = nextseedline .. queue[2].difficulty .. " " .. queue[2].seedtype .. ": " .. queue[2].seed
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
    return difficulty
end

function randomizer.cycledifficulty()
    local index = table.indexof(difficulties, difficulty)
    index = ((index - 1 + 1) % #difficulties) + 1
    difficulty = difficulties[index]
end

function randomizer.setnextseedlogic()
    if newgamenewseed == "On" then
        queue[2] = {seedtype = randomizer.SEEDTYPE.RANDOMSEED}
    elseif newgamenewseed == "Off" then
        queue[2] = {seedtype = randomizer.SEEDTYPE.KEEPSEED}
    elseif newgamenewseed == "Auto" then
        if queue[1].seedtype == randomizer.SEEDTYPE.RANDOMSEED then
            queue[2] = {seedtype = randomizer.SEEDTYPE.RANDOMSEED}
        else
            queue[2] = {seedtype = randomizer.SEEDTYPE.KEEPSEED}
        end
    else
        error("newgamenewseed was not \"On\", \"Off\", or \"Auto\"")
    end
end

function randomizer.createsequence(seed, difficulty)
    math.randomseed(tonumber(seed))

    if dependencies[difficulty] == nil then
        error("difficulty is not advanced, intermediate or beginner")
    end

    local workingset = {}
    local visited = { 0 }
    local sequence = {}

    for i=2,30 do
        if dependencies[difficulty][i] == nil then
            table.insert(workingset, i)
        end
    end

    while #workingset ~= 0 do
        local newlevelindex = math.random(#workingset)
        local newlevel = workingset[newlevelindex]
        table.insert(visited, newlevel)
        table.remove(workingset, newlevelindex)
        for _, nowvalid in pairs(dependants[difficulty][newlevel] or {}) do
            if not table.contains(visited, nowvalid) and not table.contains(workingset, nowvalid) then
                table.insert(workingset, nowvalid)
            end
        end
        table.insert(sequence, newlevel - 2)
    end
    table.insert(sequence, 31 - 2)
    return sequence
end

function randomizer.randseed()
    local difficulty = difficulty
    local seed = os.time() .. math.floor(os.clock()*10000)
    local sequence = randomizer.createsequence(seed, difficulty)
    queue = {queue[1], {difficulty = difficulty, seed = seed, sequence = sequence, seedtype = randomizer.SEEDTYPE.RANDOMSEED}}
end

function randomizer.setseed(seed)
    local difficulty = difficulty
    local sequence = randomizer.createsequence(seed, difficulty)
    if queue[2].seedtype == randomizer.SEEDTYPE.KEEPSEED or queue[2].seedtype == randomizer.SEEDTYPE.RANDOMSEED then
        queue[2] = {difficulty = difficulty, seed = seed, sequence = sequence, seedtype = randomizer.SEEDTYPE.SETSEED}
    else
        table.insert(queue, {difficulty = difficulty, seed = seed, sequence = sequence, seedtype = randomizer.SEEDTYPE.SETSEED})
    end
end

function randomizer.setsequence(sequence, sequencestr)
    if queue[2].seedtype == randomizer.SEEDTYPE.KEEPSEED or queue[2].seedtype == randomizer.SEEDTYPE.RANDOMSEED then
        queue[2] = {sequence = sequence, seedtype = randomizer.SEEDTYPE.SETSEQUENCE, sequencestr = sequencestr}
    else
        table.insert(queue,{sequence = sequence, seedtype = randomizer.SEEDTYPE.SETSEQUENCE, sequencestr = sequencestr})
    end
end

local function nextlevel()
    if levelindex <= #levelsequence then
        tas:set_level(levelsequence[levelindex])
    end
    levelindex = levelindex + 1
end

function randomizer.run()
    
    if queue[2].seedtype == randomizer.SEEDTYPE.RANDOMSEED then
        randomizer.randseed()
    elseif queue[2].seedtype == randomizer.SEEDTYPE.KEEPSEED then
        queue[2] = queue[1]
    end

    table.remove(queue,1)

    levelsequence = queue[1].sequence

    if #queue == 1 then
        randomizer.setnextseedlogic()
    end

    levelindex = 1

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
    queue = {{seed = ""}}
end

return randomizer
