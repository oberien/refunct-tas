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

Strings are u32-length-prefixed and UTF-8 encoded.

Tool to Rtil:

* `0`: Rebo Code as String
* `1`: Stop execution of Rebo and reset game values
* `2`: Set configured keys in this order as i32. Needs to be called before executing a Tas.
    + `forward`
    + `backward`
    + `left`
    + `right`
    + `jump`
    + `crouch`
    + `menu`
* `3`: Current absolute working directory path of the tool as String.
       This is used to resolve `includes` from.
* `255`: Error occured. Error code following.

Rtil to Tool:

* `0`: Print following String to stdout
* `1`: Rebo Execution finished
* `255`: Error occured. Error code following.

Error Codes:

* `0`: Unknown command.
* `1`: There is already an open connection.
* `2`: Invalid data.
