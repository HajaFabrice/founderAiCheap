param(
    [string]$ProcessName = "founderai-ollama-rust"
)

$processes = Get-Process -Name $ProcessName -ErrorAction SilentlyContinue

if (-not $processes) {
    Write-Host "No FounderAI processes matched."
    exit 0
}

foreach ($process in $processes) {
    try {
        Stop-Process -Id $process.Id -Force -ErrorAction Stop
        Write-Host "Stopped FounderAI process $($process.Id)"
    } catch {
        Write-Warning "Could not stop process $($process.Id): $($_.Exception.Message)"
    }
}
