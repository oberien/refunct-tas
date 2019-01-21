# Refunct TaS Tool

This project is a tool enabling Tool assisted Speedruns in the game
[Refunct](http://refunctgame.com/).

# Configuration

You can configure your inputs in `Config.toml`.
Make sure to set the inputs to the same keys you use ingame.
Otherwise the TAS won't work as expected.

All keys can be either a single character inside single-ticks or the decimal
number belonging to the virtual-key code.
A list of virtual-key codes on Windows can be found at
http://cherrytree.at/misc/vk.htm .
Characters must be uppercase on Windows and lowercase on Linux.

For example if you want to use RMB for Jump on Windows, the according virtual-key
code is called VK_RBUTTON and its decimal value is 2. So you'll need to specify
`jump = 2`.
If instead you would like to have space as jump, you can use the character
`jump = ' '`.

While on Windows Unreal Engine relies on the Virtual-Key Codes, on Linux it uses
its own key definitions.
Therefore it might be needed to add `(1<<30)` to the modifier's value.
For example left shift on Windows is `160` while it's
`(1<<30) + 225 = 1073742049` on Linux.

# Building

Refunct-tas is supported on Linux and Windows.

## Linux

Make sure to have rust installed either with [rustup](https://www.rustup.rs/)
containing the latest nightly compiler.
Execute `make`.
This will create the directory `build/linux/` containing `refunct-tas`,
`librtil.so` and some lua script files.

## Windows

Make sure to have Rust installed with [rustup](https://www.rustup.rs/) and
installed the latest nightly-i686-pc-windows-msvc.
Execute the file `make.bat` either with a double-click, or preferably in cmd.
It will create a directory `build\windows\ ` containing `refunct-tas.exe`,
`rtil.dll` and some lua script files.

# Running

Refunct must already be running before executing these steps.

## Linux

On Linux `LD_PRELOAD` is used to inject the refunct tas runtime library (rtil)
into the process.
This can be done in Steam in Library → Right click on Refunct → Properties →
Set Launch Options... .
There, specify `LD_PRELOAD=/absolute/path/to/build/linux/librtil.so %command%`.

To run a lua TAS script file, execute `refunct-tas <file>`.

## Windows

On windows open cmd (e.g. with Win+R → cmd → return) and `cd` into the directory
containing the TAS tool files (e.g. with `cd C:\Users\User\refunct-tas\ `).
There, execute `refunct-tas.exe <file>` to run a lua TAS script file.

# Writing TAS files

TAS files are written in the [LUA](https://www.lua.org/) programming language.
The TAS tool exposes an API which can be used to perform the TAS.
The API documentation of the exposed API can be found [in lua-api.md](/docs/lua-api.md).
You can find code examples in the [tool directory](tool/).

# Troubleshooting

* **thread 'main' panicked at 'Failed to decode config: Error**:
  Your config file is invalid and couldn't be parsed correctly.
  Please make sure to correctly configure it.
* **thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value [...] "Connection Refused"**:
  The tool couldn't connect to the game.
  Please make sure that Refunct is started before running the tool.
  On Linux also make sure that you configured `LD_PRELOAD` correctly.
* **Refunct crashes with (or without) FATAL ERROR**:
  On Windows this can happen from time to time when you start the tool.
  Try restarting Refunct and the tool.
  If the game continues to FATAL ERROR after multiple tries (2-3 should be enough),
  try to change your ingame FPS to a fixed value (e.g. 60 FPS) and try again 2-3 times.
  If it continues to crash, it could be due to multiple causes:
    1. You are not using the latest version of Refunct.
        Please verify your game files with Steam: Library → Right Click on Refunct →
        Properties → Local Files → Verify Integrity of Game Files...
    1. It could mean that I was too lazy to update all pointers to the latest version
        of Refunct.
        Currently refunct-tas is updated for Refunct BuildID 1964685.
        You can find your BuildID in Steam: Library → Right Click on Refunct →
        Properties → Local Files → bottom left
* **Refunct / the TAS tool crashes and the file `refunct-tas.exe` disappears:**
  Refunct-tas uses library injection, which some antivirus programs see as malicious
  action.
  Therefore your antivirus might have stopped execution and moves the executable
  into quarantine.
  Redownload the zip file and either whitelist `refunct-tas.exe` or disable it
  while you are using the TAS tool.
* **The TAS tool doesn't move correctly**:
  Please make sure that the keys configured in `Config.toml` match the ones
  configured ingame.
* **I got EAC banned**:
  No, you didn't. Refunct does not come with EAC. Period.

If your issue is not mentioned in this section, or you couldn't resolve it,
please contact me and provide the file `%temp%/refunct-tas.log` as well as all
output of the tool on the command line.
Either open an issue here or tag me (oberien) on
[the Refunct Speedrunners Discord Guild](https://discord.gg/Df8pHA7).
