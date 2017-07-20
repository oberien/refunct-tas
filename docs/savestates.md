# "Savestates"

Actual savestates are not (yet) possible with refunct-tas.
This results from savestates usually being created inside an emulator which
emulates the actual game.
This is not feasible for the Unreal Engine.

The current approach to achieve a savestate-like experience is to use the
[Teleportbutton Prelude][tp] to teleport to the button before the skip you'd
like to train and after that set your location, rotation and velocity to
the desired values.
Those values can be acquired by executing [`printstats.lua`][pl] with
`refunct-tas.exe`.
The file can be downloaded into the directory containing all other files
and executed as described in [Running](/README.md#running).

One example of a working script file to train the spiral skip can be found
[here](/tool/spiral.lua):

```lua
require "prelude"
require "teleportbutton-prelude"

while true do
  waitfornewgame()
  setdelta(1/60)
  teleportbutton(19)
  setrotation(0, 0, 0)
  setlocation(-1065, -3842, 464)
  setdelta(0)
end
```

In the first two lines we load required scripts which provide us with all
functions we need.
Then, we start an infinite loop, which will wait until the button "New Game" is
pressed ingame, teleport us to to the required button and set our character's
values.
The `setdelta` methods are required to ensure equal execution without desync.

After creating a lua script file to get to your "savestate", rename that file
to `tas.lua`.
This is the file `refunct-tas.exe` will load and execute if no other parameter
is passed.
That means that you can just double-click on the executable file and it'll
execute the file named `tas.lua`, which is your script.
Whenever you press "New Game", it'll "restore" your "savestate".

[tp]: /docs/lua-api.md#teleportbutton-prelude
[pl]: /tool/printstats.lua
