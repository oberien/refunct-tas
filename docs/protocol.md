# Protocol

The protocol used to communicate between the tool and the library uses raw bytes
  sent over a TCP connection.
The library listens on `localhost:21337` and accepts only one connection at a
  time.
After one connection is established, the listener no longer waits for another
  connection until the current one is finished.

Packets have a variable length.
The first byte defines which command is sent.
Only some requests result in a response.
If an error occurs, it'll be sent as soon as possible.
Any unexpected behaviour results in a disconnect.

Endianess of multibyte primitive types is little endian.

Requests:
* `0`: Stop execution before the next frame.
* `1`: Continue execution until the next frame.
        This request will get response `0`.
* `2`: Continue execution without stopping.
* `3`: Press the key in the following int32.
* `4`: Release the key in the following int32.
* `5`: Move the mouse by the following int32 `x` and int32 `y`.
* `6`: Set all following time deltas between frames to the following float64.
        If the given delta is 0, no custom delta will be used.
* `7`: Set the rotation of the player to the following 3 float32: Pitch, Yaw and Roll.
* `8`: Set the location of the character to the following 3 float32: x, y, z.
* `9`: Set the velocity of the character to the following 3 float32: x, y, z.

Responses:
* `0`: Response to `1`, followed by the player's status:
    + Location: 3 float32: x, y, z.
    + Rotation: 3 float32: Pitch, Yaw and Roll.
    + Velocity: 3 float32: x, y, z.
    + Acceleration: 2 float32: x, y.
* `1`: New Game detected
* `255`: Error occured. The error code can be found in the next byte.

Error Codes:
* `0`: Unknown command.
