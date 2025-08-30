@echo off
cd /d "%~dp0\..\..\"
echo Stopping Eden2...
docker-compose -f docker\docker-compose.yml down
echo Eden2 stopped successfully!
pause
