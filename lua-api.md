# Exposed functions

There are a few major functions which are exposed from the Rust side.
They are rather low-level, but can be called from everywhere within lua.
They are wrapped inside `prelude.lua` to provide an easier interface.
These are the exposed functions:

* `__stop()`: Stops execution of Refunct before the next frame will be calculated
  and rendered.
  This function returns a player stats object, which for now just contains
  the character's pitch, yaw and roll.
* `__step()`: Continues execution until before the next frame.
  As `__stop`, this function returns the player stats.
* `__press_key(key)`: Presses the passed key. Allowed values are:
    + `"forward"`
    + `"backward"`
    + `"left"`
    + `"right"`
    + `"jump"`
    + `"crouch"`
    + `"menu"`
* `__release_key(key)`: Releases the passed key.
  Allowed values match the ones of `__press_key`.
* `__move_mouse(x, y)`: Moves the mouse by the specified integer values.
  The distance moved is influenced by the game's sensitivity settings.
  Also, for some reason this function does not move the mouse by the same amount
  on equal executions, so it should be avoided for now.
* `__set_delta(delta)`: Sets the delta time between two frames in seconds.
  A value of 1/60 will "emulate" 60 FPS.
  For example if you play on 120 FPS, the game will run twice as fast.
  This can be used to smooth out frames, as each frame will be calculated using
  the exact same delta time.
  It should be used to make inputs more reliable.
  In fact, this function should *always* be called to make TASing bearable / 
  possible.
* `__set_rotation(pitch, yaw, roll)`: Sets the rotation of the character to
  the passed float values in degrees.
  This is done by memory-editing the values and is therefore super consistent.
  It should be preferred over `__move_mouse` for the time being.
  Pitch is the x-axis with positive values turning right and negative ones
  turning left.
  Yaw is the y-axis with positive values making the character look down and
  negative values look up.
* `__wait_for_new_game()`: This function does what its name says:
  It stops execution of the lua script until the `New Game` button was clicked
  in Refunct.
  It should be used to start execution frame-perfectly with the game as initial
  synchronization step.

# Prelude

The file `prelude.lua` contains some functions which wrap the exported
functions and provide a nicer interface.

* `waitfornewgame()`: Simple wrapper around `__wait_for_new_game`.
* `setdelta(delta)`: Simple wrapper around `__set_delta`.
* `setrotation(pitch, yaw, roll)`: Simple wrapper around `__set_rotation`.
* `step()`: Smart wrapper around `__step`.
* `getplayerstats()`: Returns the player stats object containing the character's
  pitch, yaw and roll.
* `frame(keys, degx, degy, repeatnum)`: Executes `repeatnum` frames.
  During those frames, it turns the character by `degx` degrees on the x-axis
  and `degy` degrees on the y-axis.
  It also keeps all keys inside the `keys` list pressed, which can contain
  any combination of the following keys:
    + `forward`
    + `backward`
    + `left`
    + `right`
    + `jump`
    + `crouch`
    + `menu`
