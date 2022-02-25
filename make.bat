echo Preparing build
md build
md build\practice-windows
md build\practice-windows
del /Q build\practice-windows\*
echo Building
cd rtil
cargo build --release --target=i686-pc-windows-msvc
cd ..\tool
cargo build --target=i686-pc-windows-msvc
cd ..
echo Copying files
copy rtil\target\i686-pc-windows-msvc\release\rtil.dll build\practice-windows
copy tool\target\i686-pc-windows-msvc\debug\refunct-tas.exe build\practice-windows
copy tool\Config.toml build\practice-windows
copy tool\main.re build\practice-windows
copy tool\component.re build\practice-windows
copy tool\keys.re build\practice-windows
copy tool\newgame.re build\practice-windows
copy tool\practice.re build\practice-windows
copy tool\randomizer.re build\practice-windows
copy tool\teleport.re build\practice-windows
copy tool\ui.re build\practice-windows
echo Converting lf to crlf
call :convert Config.toml
call :convert main.re
call :convert component.re
call :convert keys.re
call :convert newgame.re
call :convert practice.re
call :convert randomizer.re
call :convert teleport.re
call :convert ui.re

powershell -Command "(gc build\practice-windows\Config.toml) -replace \"forward = 'v'\", \"forward = 'W'\" -replace \"backward = 'i'\", \"backward = 'S'\" -replace \"left = 'u'\", \"left = 'A'\" -replace \"right = 'a'\", \"right = 'D'\" -replace \"crouch = 1073742049\", \"crouch = 160\" | sc build\practice-windows\Config.toml"

echo Don't forget to create a zip
exit /b 0

:convert
type "build\practice-windows\%~1" | find /v "" > "build\practice-windows\%~1.crlf"
move "build\practice-windows\%~1.crlf" "build\practice-windows\%~1"
exit /b 0
