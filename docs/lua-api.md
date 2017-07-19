# Lua API

There are a few functions which are exposed to lua from the Rust side.
They are rather low-level, but can be called from everywhere within lua scripts.
To provide an easier interface, these low level functions are wrapped inside
`prelude.lua`.

# Prelude

The file `prelude.lua` contains functions which wrap the exported
functions and provide a more accessible interface.
To use the prelude in your own scripts, insert `require "prelude"` as first line.

* `waitfornewgame()`: This function waits until "New Game" is pressed ingame.
  Further execution of the lua script will only continue after that event.
  This function should be used to start a TAS in order to always start from the
  very beginning.
  This function will update the stats returned by `getplayerstats`.

  **Warning**: This function will execute the first frame after pressing
  "New Game" before returning to the Lua script.
  Therefore, you should make sure to execute `setdelta` before calling this
  function to avoid desync.
* `setdelta(delta)`: Set the delta time between frames.
  Unreal Engine uses that delta time for calculations of physics.
  If this function is not used, the time between frames used by UE may differ
  after every frame.
  For example the time between the first and second frame may be 17ms, but the
  time between the second and third frame might be 20ms.
  On a different computer with less FPS, the time between two frames can be
  in the order of 50ms.
  This means that on the faster computer, the character moves 12cm between two
  frames, while it moves nearly three times as much, namely 35cm, on the slower
  PC.
  Thus, solely frame-perfect input doesn't make TASing possible.
  With this function, the delta time can be fixed to the passed value.
  If you specify 17ms (1/60s), the game will seem smooth to the high-end PC.
  On the low-end PC, the game will seem to run at 1/3 of its usual speed,
  because with only 20 frames but a deltatime of 1/60, only one third of a
  second will actually pass.
  Anyhow, this is what enables kind of "smooth frames", i.e. frames, which
  will always and under all condition have the exact same length.
* `setrotation(pitch, yaw)`: Sets the rotation of your character to the
  passed float values in degrees.
  Pitch changes the y-axis with positive values making the character look down
  and negative values look up.
  Yaw is controlling the x-axis with positive values turning right and negative
  ones left.
* `setlocation(x, y, z)`: Sets the location of the character to the passed
  x, y and z values.
* `setvelocity(velx, vely, velz)`: Sets the velocity of the character to the
  passed x, y and z values.
* `step()`: Continues execution until before the next frame.
  This function will update the stats returned by `getplayerstats`.
* `getplayerstats()`: Returns the player stats object containing the following
  values of your character:
    + `pitch`: Pitch of your player (left / right rotation).
    + `yaw`: Yaw of your character (up / down rotation).
    + `roll`: Roll of your character (tilt of your head).
    + `x`: X-coordinate of your character in the world.
    + `y`: Y-coordinate of your character in the world.
    + `z`: Z-coordinate of your character in the world.
    + `velx`: Velocity of your character in the X direction.
    + `vely`: Velocity of your character in the Y direction.
    + `velz`: Velocity of your character in the Z direction.
    + `accx`: Acceleration of your character in the X direction.
      This is only the acceleration caused by keyboard inputs.
    + `accy`: Acceleration of your character in the Y direction.
      This is only the acceleration caused by keyboard inputs.
* `frame(keys, degx, degy, repeatnum)`: Executes `repeatnum` frames.
  During those frames, the character is turned by `degx` degrees to the
  left / right and `degy` degrees up / down.
  It also keeps all keys inside the `keys` list pressed, which can contain
  any combination of the following keys:
    + `forward`
    + `backward`
    + `left`
    + `right`
    + `jump`
    + `crouch`
    + `menu`
  This function will update the stats returned by `getplayerstats`.

# Exposed functions

These functions are exposed from Rust and should only be used if you know
exactly what you're doing:

* `__stop()`: Stops execution of Refunct before the next frame will be calculated
  and rendered.
  Returns the player stats object as returned by `getplayerstats()`
* `__step()`: Continues execution until before the next frame.
  As `__stop`, this function returns the player stats object.
* `__press_key(key)`: Presses the passed key as configured in `Config.toml`.
  Allowed values are:
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
* `__set_delta(delta)`: Sets the delta time between two frames in seconds used
  by UE for physics calculations / in its `Tick` functions.
* `__set_rotation(pitch, yaw, roll)`: Sets the rotation of the character to
  the passed float values in degrees.
* `__set_location(x, y, z)`: Sets the location of the character.
* `__set_velocity(velx, vely, velz)`: Sets the velocity of the character.
* `__wait_for_new_game()`: Waits until the "New Game" button is pressed ingame.
