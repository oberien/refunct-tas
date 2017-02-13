require "prelude"

-- frame is a function you can call to easily execute one or multiple frames.
-- The first argument is a list of all keys pressed.
-- Allowed values are forward, backward, left, right, jump, crouch and menu.
-- The second and third arguments are for mouse x and y values respectively.
-- The fourth argument is the number of frames the previous values should be
--  executed.
-- Arguments are optional and may be omitted, but if you want to specify a rear
--  argument, all previous arguments need to be provided.
-- So if you want to specify the fourth argument, all 4 arguments need to be
--  provided.
-- For example `frame({forward, right, jump}, 69, 21, 10)` means that the
--  keys forward, right and jump will be pressed for the next 10 frames, while
--  the mouse moves to the right by 69 and down by 21 each of the 10 frames.
frame({}, 0, 0, 100)
frame({}, 1490, 0)
frame({}, 0, 0, 113)
frame({jump, forward}, 0, 0, 40)
frame({forward}, 0, 0, 11)
frame({}, -1000, 0)
frame({}, 0, 0, 6)
frame({forward}, 0, 0)
frame({forward, jump}, 0, 0, 11)
frame({}, 0, 0, 26)
frame({menu}, 0, 0)
frame({})
frame({jump})
frame({})
