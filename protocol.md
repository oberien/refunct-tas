# Protocol

The protocol used to communicate between the tool and the library uses raw bytes
  sent over a TCP connection.
The library listens on `localhost:21337` and accepts only one connection at a
  time.
After one connection is established, the listener no longer waits for another
  connection until the current one is finished.
  
Packets have a variable length.
The first byte defines which command is sent.
For each request to the library there will be a response after execution,
  usually indicating success or containing an error code.
Any unexpected behaviour results in a disconnect.

Endianess of multibyte primitive types is little endian.

Requests:
* `0`: Stop execution before the next frame.
* `1`: Continue execution until the next frame.
* `2`: Continue execution without stopping.
* `3`: Press the key in the following int32.
* `4`: Release the key in the following int32.
* `5`: Move the mouse by the following int32 `x` and int32 `y`.
* `6`: Set all following time deltas between frames to the following float64.
        If the given delta is 0, no custom delta will be used.

Responses:
* `0`: Command executed successfully
* `1`: New Game detected
* `255`: Error occured. The error code can be found in the next byte.

Error Codes:
* `0`: Unknown command.
