require "prelude"

waitfornewgame()
setdelta(1/60)
-- frame is a function you can call to easily execute one or multiple frames.
-- The first argument is a list of all keys pressed.
-- Allowed values are forward, backward, left, right, jump, crouch and menu.
-- The second and third arguments are for pitch (x-axis) and yaw (y-axis) values
--  respectively.
-- The fourth argument is the number of frames the previous values should be
--  executed.
-- Arguments are optional and may be omitted, but if you want to specify a rear
--  argument, all previous arguments need to be provided.
-- So if you want to specify the fourth argument, all 4 arguments need to be
--  provided.
-- For example `frame({forward, right, jump}, 69, 21, 10)` means that the
--  keys forward, right and jump will be pressed for the next 10 frames, while
--  the character turns right by 69 degrees and down by 21 degrees by moving
--  the mouse accordingly over the span of the next 10 frames.
frame({}, 90, -40, 214)
frame({jump, forward}, 0, 90, 40)
frame({}, -40, 0, 7)
frame({}, 0, 0, 7)
frame({forward})
frame({forward, jump}, 0, 0, 11)
frame({}, 0, 0, 26)
frame({menu})
frame()
frame({jump})
frame()
