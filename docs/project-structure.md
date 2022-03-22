# Project Structure

This project consists of two parts:
A library which is injected into a running Refunct instance, where it'll control UE,
and a tool, which is the "user interface" interacting with the lib.

## Lib

The lib is logically separated into a platform-dependent hooking part and two
loops which execute in parallel.
One loop, the handler loop, is used as interface for the tool, the other one is the Unreal Engine
update-render loop.

During initialization, which is achieved as described in [Library Injection][li],
we find the functions inside the mapped binary as documented in
[Function Pointers / Signatures][fps].
Then, we hook them and start our main loop.
The hooked functions are explained in [Function Hooking][fh].
As function hooking is platform dependent, this is the only major part of this
project where we differentiate between linux and windows.
There is a `native` mod, which contains two mods: `linux` and `windows`.
The main file of the native mod is still platform independent.
In the linux and windows mods, we find code which manages pointers and hooks
on their respective platform.
Their only purpose is to provide functions which can be called to get / set
memory values (like location and rotation) or call into existing functions
(e.g. `tick_intercept`).

The main idea of the handler loop is to convert raw data received from the tool
on a TCP connection as described in [protocol][p] to events,
which it sends to UE's loop via a channel.
We hold state if the tool wants us to stop after each frame or not.
If we need to stop after each frame, we wait for the `Stopped` event from UE's
loop and write the character stats packet to the socket.
Anyway, we then read a packet from the socket and convert it to an event, which
we forward to the UE loop.
If we receive a `NewGame` event from UE's loop, we'll send the according packet
over the socket.

UE's loop is intercepted after each frame by hooking a `Tick` function as
described in [Function Hooking][fh].
With this method, a method `tick_intercept` is called after each frame.
We have an internal state to keep track if the tool wants us to stop after each
frame, or if we can continue.
If we don't need to stop, we test if there is an event by the handler loop, which
we need to handle.
If that's not the case, we let UE continue its execution.
If we do need to stop, instead of testing if there is an event from the handler
loop, we wait until we get an event.
If there is an event, we handle it and repeat.
No matter of the path taken, before we continue execution of UE, we set
deltatime to the according value.

## Tool

The tool injects the library into the game
on Windows (must be done with `LD_PRELOAD` manually on Linux), reads the rebo script
file (defaulting to `main.re` if none was passed) and sends it to the in-game lib.
In the `config` mod we parse the config.
In `inject.rs`, we inject the lib into Refunct on Windows.
The `tas` mod wraps the lib's TCP socket protocol.

[li]: /docs/library-injection.md
[fps]: /docs/function-signatures.md
[fh]: /docs/function-hooking.md
[p]: /docs/protocol.md
