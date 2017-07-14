echo Preparing build
md build
md build\windows
echo Building
cd lib
rustup run nightly cargo build --target=i686-pc-windows-msvc
cd ..\tool
rustup run nightly cargo build --target=i686-pc-windows-msvc
cd ..
echo Copying files
copy lib\target\i686-pc-windows-msvc\debug\rtil.dll build\windows
copy tool\target\i686-pc-windows-msvc\debug\refunct-tas.exe build\windows
copy tool\Config.toml build\windows
copy tool\prelude.lua build\windows
copy tool\ngg.lua build\windows
copy tool\turn.lua build\windows
copy tool\rotation.lua build\windows
copy tool\printlocation.lua build\windows
copy tool\teleportbuttons.lua build\windows
echo Converting lf to crlf
call :convert Config.toml
call :convert prelude.lua
call :convert ngg.lua
call :convert turn.lua
call :convert rotation.lua
call :convert printlocation.lua
call :convert teleportbuttons.lua
echo Don't forget to change default bindings in Config.toml and to create a zip

exit /b 0

:convert
type "build\windows\%~1" | find /v "" > "build\windows\%~1.crlf"
move "build\windows\%~1.crlf" "build\windows\%~1"
exit /b 0
