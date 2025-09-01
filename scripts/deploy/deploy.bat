@echo off
cd /d "%~dp0\..\..\"
echo ================================
echo   Eden2 One-Click Deployment
echo ================================

echo.
echo [1/3] Building web version...
REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Installing wasm-pack...
    cargo install wasm-pack
)

REM Build the WASM package
wasm-pack build --target web --out-dir web --release

if %ERRORLEVEL% neq 0 (
    echo ❌ Build failed!
    pause
    exit /b 1
)

REM Copy index.html to web directory
echo Copying index.html to web directory...
copy "src\web\index.html" "web\index.html" >nul
if %ERRORLEVEL% neq 0 (
    echo ❌ Failed to copy index.html!
    pause
    exit /b 1
)

echo [2/3] Stopping existing deployment...
docker-compose -f docker\docker-compose.yml down 2>nul

echo [3/3] Starting fresh deployment...
docker-compose -f docker\docker-compose.yml up -d --build

if %ERRORLEVEL% neq 0 (
    echo ❌ Docker deployment failed!
    pause
    exit /b 1
)

echo.
echo ================================
echo ✅ Eden2 deployed successfully!
echo ================================
echo.
echo 🎮 Game is ready: http://localhost
echo 📱 Network access: http://eedenthegame.com
echo.
echo Quick commands:
echo   Stop:    stop.bat
echo   Restart: scripts\deploy\restart.bat
echo   Logs:    scripts\deploy\logs.bat
echo   Redeploy: deploy.bat
echo.
echo ================================
