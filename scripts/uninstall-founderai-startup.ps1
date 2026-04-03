param(
    [string]$ShortcutName = "FounderAI Ollama.cmd"
)

$startupFolder = [Environment]::GetFolderPath("Startup")
$launcherPath = Join-Path $startupFolder $ShortcutName

if (Test-Path $launcherPath) {
    Remove-Item -LiteralPath $launcherPath -Force
    Write-Host "Removed FounderAI startup launcher: $launcherPath"
} else {
    Write-Host "No FounderAI startup launcher was installed."
}
