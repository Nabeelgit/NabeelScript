@echo off

rem Compile the project
rustc -o main.exe src\main.rs

if %errorlevel% neq 0 (
    echo Compilation failed
    exit /b %errorlevel%
)

rem Check if a .nabeel file is provided as an argument
if "%~1"=="" (
    echo Usage: %~nx0 ^<file.nabeel^>
    exit /b 1
)

rem Run the compiled executable with the provided .nabeel file
main.exe "%~1"