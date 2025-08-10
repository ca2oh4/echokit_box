@echo off
:: Enhanced script to create a junction link for .embuild directory

:: Get current working directory
set "current_dir=%cd%"

:: Get RUSTUP_HOME environment variable
set "rustup_home=%RUSTUP_HOME%"

:: If RUSTUP_HOME is not set, use USERPROFILE directory
if "%rustup_home%"=="" (
    set "rustup_home=%USERPROFILE%\.rustup"
    echo RUSTUP_HOME not set, using default: %rustup_home%
) else (
    echo Using RUSTUP_HOME: %rustup_home%
)

:: Define source path
set "source_path=%rustup_home%\toolchains\esp\.embuild"

:: Check if source directory exists
if not exist "%source_path%" (
    echo Error: Source directory does not exist: %source_path%
    echo Please ensure the ESP toolchain is installed properly.
    pause
    exit /b 1
)

:: Check if target junction already exists
if exist "%current_dir%\.embuild" (
    echo Warning: .embuild already exists in current directory
    echo Removing existing junction...
    rmdir "%current_dir%\.embuild"
)

:: Create the junction
echo Creating junction:
echo From: %source_path%
echo To:   %current_dir%\.embuild

mklink /j "%current_dir%\.embuild" "%source_path%"

:: Check if mklink was successful
