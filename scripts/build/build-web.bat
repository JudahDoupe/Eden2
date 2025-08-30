@echo off
cd /d "%~dp0\..\..\"
echo ================================
echo   Eden2 Development Build
echo ================================
echo Building web version for testing (no deployment)...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Installing wasm-pack...
    cargo install wasm-pack
)

REM Build the WASM package
echo Building WASM package...
wasm-pack build --target web --out-dir web --release

REM Check if build was successful
if %ERRORLEVEL% neq 0 (
    echo ❌ Build failed!
    pause
    exit /b 1
)

echo ✅ Build complete!
echo.
echo Files generated in web\ directory
echo For quick local testing: 
echo   cd web ^&^& python -m http.server 8080
echo   OR just run deploy.bat for full Docker deployment
echo.
pause
