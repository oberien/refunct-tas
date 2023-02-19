echo Preparing build
md build
md build\practice-windows
md build\practice-windows
del /Q build\practice-windows\*
echo Building
cd rtil
cargo +nightly build --release --target=i686-pc-windows-msvc || exit /b
cd ..\tool
cargo build --target=i686-pc-windows-msvc || exit /b
cd ..
echo Copying files
copy rtil\target\i686-pc-windows-msvc\release\rtil.dll build\practice-windows
copy tool\target\i686-pc-windows-msvc\debug\refunct-tas.exe build\practice-windows
copy tool\main.re build\practice-windows
copy tool\prelude.re build\practice-windows
copy tool\component.re build\practice-windows
copy tool\keys.re build\practice-windows
copy tool\newgame.re build\practice-windows
copy tool\practice.re build\practice-windows
copy tool\randomizer.re build\practice-windows
copy tool\teleport.re build\practice-windows
copy tool\ui.re build\practice-windows
copy tool\multiplayer.re build\practice-windows
copy tool\tas.re build\practice-windows
copy tool\windshieldwipers.re build\practice-windows
copy tool\settings.re build\practice-windows
copy tool\misc.re build\practice-windows
copy tool\timer.re build\practice-windows
echo Converting lf to crlf
call :convert main.re
call :convert prelude.re
call :convert component.re
call :convert keys.re
call :convert newgame.re
call :convert practice.re
call :convert randomizer.re
call :convert teleport.re
call :convert ui.re
call :convert multiplayer.re
call :convert tas.re
call :convert windshieldwipers.re
call :convert settings.re
call :convert misc.re
call :convert timer.re

echo Don't forget to create a zip
exit /b 0

:convert
type "build\practice-windows\%~1" | find /v "" > "build\practice-windows\%~1.crlf"
move "build\practice-windows\%~1.crlf" "build\practice-windows\%~1"
exit /b 0
