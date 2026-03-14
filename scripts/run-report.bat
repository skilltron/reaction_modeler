@echo off
REM Standalone run script for the genetic report package (Windows 11 / Windows 10).
REM Expects vcf-to-variants.exe and genetic-report-html.exe in the same directory.
REM Usage: run-report.bat path-to.vcf
REM        run-report.bat path-to-variants.json
REM Report is written to: %USERPROFILE%\Downloads\genetic_report.html

set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:\=/%"
set "DOWNLOADS=%USERPROFILE%\Downloads"
set "OUT=%DOWNLOADS%\genetic_report.html"

if "%~1"=="" (
  echo Usage: run-report.bat ^<path-to.vcf or path-to-variants.json^>
  echo   Report will be written to: %OUT%
  exit /b 1
)

set "INPUT=%~1"
set "VARIANTS_JSON=%SCRIPT_DIR%variants_temp_%RANDOM%.json"
set "VCF_TO_VARIANTS=%SCRIPT_DIR%vcf-to-variants.exe"
set "REPORT_HTML=%SCRIPT_DIR%genetic-report-html.exe"

if not exist "%VCF_TO_VARIANTS%" (
  echo Binary not found: %VCF_TO_VARIANTS%
  exit /b 1
)
if not exist "%REPORT_HTML%" (
  echo Binary not found: %REPORT_HTML%
  exit /b 1
)

echo %INPUT% | findstr /i "\.json$" >nul
if %ERRORLEVEL% equ 0 (
  echo Building report from variants JSON...
  "%REPORT_HTML%" "%INPUT%" "%OUT%"
) else (
  echo Converting VCF to variants...
  REM For .gz files: Windows may not have gzip; try passing path to exe (Rust handles .gz for non-BGZF)
  echo %INPUT% | findstr /i "\.gz$" >nul
  if %ERRORLEVEL% equ 0 (
    "%VCF_TO_VARIANTS%" "%INPUT%" > "%VARIANTS_JSON%" 2>nul
  ) else (
    "%VCF_TO_VARIANTS%" "%INPUT%" > "%VARIANTS_JSON%" 2>nul
  )
  echo Building report...
  "%REPORT_HTML%" "%VARIANTS_JSON%" "%OUT%"
  if exist "%VARIANTS_JSON%" del "%VARIANTS_JSON%"
)

echo Done. Report written to: %OUT%
