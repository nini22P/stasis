@echo off
echo [INFO] Compiling the project in release mode...

cargo build --release

if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    exit /b %errorlevel%
)

set "SOURCE_FILE=target\release\stasis.exe"
set "DEST_FILE=target\release\stasis.scr"

echo [INFO] Renaming %SOURCE_FILE% to stasis.scr...

rename "%SOURCE_FILE%" "stasis.scr"

echo [SUCCESS] Screen saver created at: %DEST_FILE%

pause