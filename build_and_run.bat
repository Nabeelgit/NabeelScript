@echo off

rem Compile the project
rustc -o main.exe src\main.rs

if %errorlevel% neq 0 (
    echo Compilation failed
    exit /b %errorlevel%
)

rem Run the compiled executable
main.exe
