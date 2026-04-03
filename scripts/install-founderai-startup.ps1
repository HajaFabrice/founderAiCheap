param(
    [string]$ShortcutName = "FounderAI Ollama.cmd",
    [string]$ConfigPath = "$(Join-Path $PSScriptRoot '..\\config\\founderai.json')",
    [string]$BinaryPath = ""
)

$startupFolder = [Environment]::GetFolderPath("Startup")
$startScript = (Resolve-Path (Join-Path $PSScriptRoot "start-founderai.ps1")).Path
$resolvedConfig = (Resolve-Path $ConfigPath).Path
$launcherPath = Join-Path $startupFolder $ShortcutName

$binaryArg = ""
if ($BinaryPath) {
    $resolvedBinary = (Resolve-Path $BinaryPath).Path
    $binaryArg = " -BinaryPath `"$resolvedBinary`""
}

$content = @"
@echo off
powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File "$startScript" -ConfigPath "$resolvedConfig"$binaryArg
"@

Set-Content -Path $launcherPath -Value $content -Encoding ASCII

Write-Host "FounderAI startup launcher installed."
Write-Host "Startup folder: $startupFolder"
Write-Host "Launcher: $launcherPath"
