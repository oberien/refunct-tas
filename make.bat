echo Preparing build
md build
md build\windows
del /Q build\windows\*
echo Building
cd rtil
rustup run nightly cargo build --release --target=i686-pc-windows-msvc
cd ..\tool
rustup run nightly cargo build --target=i686-pc-windows-msvc
cd ..
echo Copying files
copy rtil\target\i686-pc-windows-msvc\release\rtil.dll build\windows
copy tool\target\i686-pc-windows-msvc\debug\refunct-tas.exe build\windows
copy tool\Config.toml build\windows
copy tool\prelude.lua build\windows
copy tool\ngg.lua build\windows
copy tool\turn.lua build\windows
copy tool\rotation.lua build\windows
copy tool\printstats.lua build\windows
copy tool\teleportbutton-prelude.lua build\windows
copy tool\teleportbuttons.lua build\windows
copy tool\spiral.lua build\windows
copy tool\setvelocity.lua build\windows
copy tool\setposition.lua build\windows
copy tool\menu.lua build\windows
copy tool\record.lua build\windows
copy tool\keys.lua build\windows
copy tool\ui.lua build\windows
copy tool\multiplayer.lua build\windows
echo Converting lf to crlf
call :convert Config.toml
call :convert prelude.lua
call :convert ngg.lua
call :convert turn.lua
call :convert rotation.lua
call :convert printstats.lua
call :convert teleportbutton-prelude.lua
call :convert teleportbuttons.lua
call :convert spiral.lua
call :convert setvelocity.lua
call :convert setposition.lua
call :convert menu.lua
call :convert record.lua
call :convert keys.lua
call :convert ui.lua
call :convert multiplayer.lua

powershell -Command "(gc build\windows\Config.toml) -replace \"forward = 'v'\", \"forward = 'W'\" -replace \"backward = 'i'\", \"backward = 'S'\" -replace \"left = 'u'\", \"left = 'A'\" -replace \"right = 'a'\", \"right = 'D'\" -replace \"crouch = 1073742049\", \"crouch = 160\" | sc build\windows\Config.toml"

echo Don't forget to create a zip
exit /b 0

:convert
type "build\windows\%~1" | find /v "" > "build\windows\%~1.crlf"
move "build\windows\%~1.crlf" "build\windows\%~1"
exit /b 0
