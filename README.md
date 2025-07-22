# Refunct TaS Tool

This project is a tool enabling Tool assisted Speedruns in the game
[Refunct](http://refunctgame.com/).

# Building

Refunct-tas is supported on Linux and Windows.

## Linux

Make sure to have rust installed e.g. via [rustup](https://www.rustup.rs/).
Execute `make`.
This will create the directory `build/linux/` containing `refunct-tas`,
`librtil.so` and some rebo script files.

## Windows

Make sure to have Rust installed with [rustup](https://www.rustup.rs/) and
installed the latest stable-i686-pc-windows-msvc.
Execute the file `make.bat` either with a double-click, or preferably in cmd.
It will create a directory `build\practice-windows\ ` containing `refunct-tas.exe`,
`rtil.dll`, `Cargo.toml` and some rebo script files.

# Running

Refunct must already be running before executing these steps.

## Linux

On Linux `LD_PRELOAD` is used to inject the refunct tas runtime library (rtil)
into the process.
This can be done in Steam in Library → Right click on Refunct → Properties →
Set Launch Options... .
There, specify `LD_PRELOAD=/absolute/path/to/build/linux/librtil.so %command%`.

To run the default practice functionality, execute `refunct-tas`, which will run
`main.re` by default.  
To run your own rebo script file, execute `refunct-tas <file>`.

## Windows

To run the default practice functionality, double-click on `refunct-tas.exe`, which
will run `main.re` by default.
To run your own rebo script file, open cmd (e.g. with Win+R → cmd → Enter) and `cd` into
the directory containing the TAS tool files (e.g. with `cd C:\Users\User\refunct-tas\`).
There, execute `refunct-tas.exe <file>` to run that rebo script file.

# Writing Script Files

Script files are written in the [rebo](https://github.com/oberien/rebo#readme) programming language.
The tool exposes an API which can be used by scripts to change different things within Refunct.
The exposed API can be seen in [rebo_init.rs](/rtil/src/threads/rebo/rebo_init.rs).
You can find all code of the practice functionality in the [tool directory](tool/).

# Troubleshooting

* **thread 'main' panicked at 'Failed to decode config: Error**:
  Your config file is invalid and couldn't be parsed correctly.
  Please make sure to correctly configure it.
* **thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value [...] "Connection Refused"**:
  The tool couldn't connect to the game.
  Please make sure that Refunct is started before running the tool.
  On Linux also make sure that you configured `LD_PRELOAD` correctly.
* **thread 'main' panicked at 'Cannot get pid of Refunct: Error { kind: NotFound, message: "program not found" }'**:
  WMIC (Windows Management Instrumentation Command-Line) either doesn't exist or its directory isn't in PATH.
  To put it in path, run `control sysdm.cpl,,3` in Run (WIN+R) -> Environment Variables -> [Under "User variables for (user)] -> Double click "Path". Add this entry to it: `%SystemRoot%\System32\Wbem`. Start the tool, and it should work.
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
* **I got EAC banned**:
  No, you didn't. Refunct does not come with EAC. Period.

If your issue is not mentioned in this section, or you couldn't resolve it,
please contact me and provide the file `%temp%/refunct-tas.log` as well as all
output of the tool on the command line.
Either open an issue here or tag me (oberien) on
[the Refunct Speedrunners Discord Guild](https://discord.gg/Df8pHA7).
