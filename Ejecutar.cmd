@echo off
cd /d "%~dp0"
if not exist "bin\carga_modelos.exe" (
    echo Primero compila el proyecto con build.ps1. Consulta README.md.
    pause
    exit /b 1
)
start "" "%~dp0bin\carga_modelos.exe"
exit /b
