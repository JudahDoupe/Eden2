@echo off
cd /d "%~dp0\..\..\"
echo Restarting Eden2...
docker-compose -f docker\docker-compose.yml restart
echo Eden2 restarted successfully!
echo 🎮 Game ready: http://localhost:8080
pause
