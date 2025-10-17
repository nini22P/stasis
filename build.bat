@echo off

cd frontend

echo [INFO] Installing dependencies...

call npm install

echo [INFO] Building frontend...

call npm run build

cd ..

echo [INFO] Compiling the project in release mode...

cargo build --release

if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    exit /b %errorlevel%
)

set "SOURCE_FILE=target\release\stasis.exe"
set "DEST_FILE=target\release\stasis.scr"

echo [INFO] Renaming %SOURCE_FILE% to stasis.scr...

if exist "%DEST_FILE%" (
    del "%DEST_FILE%"
)

rename "%SOURCE_FILE%" "stasis.scr"

echo [SUCCESS] Created at: %DEST_FILE%

pause