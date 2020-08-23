require "prelude"

local allbuttons = {}

function allbuttons.start()
    _G.onlevelchange = function(level)
        if level == 0 then
            tas:set_level(29)
        end
    end

    _G.onreset = function(reset)
        tas:set_level(0)
    end
end

function allbuttons.reset()
    _G.onlevelchange = nil
    _G.onreset = nil
end

return allbuttons