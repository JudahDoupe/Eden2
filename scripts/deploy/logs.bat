@echo off
cd /d "%~dp0\..\..\"
echo Eden2 Container Logs:
echo ================================
docker-compose -f docker\docker-compose.yml logs
echo ================================
pause
