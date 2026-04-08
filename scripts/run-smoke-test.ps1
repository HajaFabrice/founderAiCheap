param(
    [string]$ConfigPath = "$(Join-Path $PSScriptRoot '..\\config\\founderai.smoke.json')",
    [string]$RoleId = "B-Production",
    [string]$Title = "No-budget smoke test",
    [string]$Body = "Write a concise founder-quality brief for how contributors should help harden FounderAI-Ollama-Rust this week. Keep it practical, anti-hype, and survival-first."
)

$resolvedConfig = (Resolve-Path $ConfigPath).Path
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$binaryPath = Join-Path $repoRoot "target\\release\\founderai-ollama-rust.exe"

if (-not (Test-Path $binaryPath)) {
    throw "FounderAI binary not found. Build it first with: cargo build --release"
}

$smokeDirs = @(
    (Join-Path $repoRoot "inbox-smoke"),
    (Join-Path $repoRoot "outbox-smoke"),
    (Join-Path $repoRoot "runtime-smoke"),
    (Join-Path $repoRoot "runtime-smoke\\runs"),
    (Join-Path $repoRoot "runtime-smoke\\archived-inbox")
)

foreach ($dir in $smokeDirs) {
    New-Item -ItemType Directory -Force -Path $dir | Out-Null
}

Set-Location $repoRoot

$pendingInboxItems = Get-ChildItem (Join-Path $repoRoot "inbox-smoke") -File -ErrorAction SilentlyContinue
if ($pendingInboxItems) {
    $archiveDir = Join-Path $repoRoot ("runtime-smoke\\archived-inbox\\" + [DateTime]::UtcNow.ToString("yyyyMMddTHHmmssZ"))
    New-Item -ItemType Directory -Force -Path $archiveDir | Out-Null
    foreach ($item in $pendingInboxItems) {
        Move-Item -LiteralPath $item.FullName -Destination (Join-Path $archiveDir $item.Name)
    }
}

$startedAtUtc = [DateTime]::UtcNow
$effectiveTitle = "$Title " + $startedAtUtc.ToString("yyyyMMddTHHmmssZ")

Write-Host "Creating smoke request..."
& $binaryPath request --config $resolvedConfig --title $effectiveTitle --body $Body --role-id $RoleId
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Host "Running isolated smoke tick..."
& $binaryPath tick --config $resolvedConfig
$tickExit = $LASTEXITCODE

$runsRoot = Join-Path $repoRoot "runtime-smoke\\runs"
$latestRun = Get-ChildItem $runsRoot -Directory -ErrorAction SilentlyContinue |
    Where-Object { $_.LastWriteTimeUtc -ge $startedAtUtc.AddSeconds(-1) } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1

if (-not $latestRun) {
    $latestRun = Get-ChildItem $runsRoot -Directory -ErrorAction SilentlyContinue |
        Sort-Object LastWriteTime -Descending |
        Select-Object -First 1
}

if ($latestRun) {
    Write-Host ""
    Write-Host "Latest run folder:"
    Write-Host $latestRun.FullName
    Write-Host ""
    Write-Host "Artifacts:"
    Write-Host (Join-Path $latestRun.FullName "prompt.md")
    Write-Host (Join-Path $latestRun.FullName "output.md")
    Write-Host (Join-Path $latestRun.FullName "stdout.txt")
    Write-Host (Join-Path $latestRun.FullName "stderr.txt")
} else {
    Write-Warning "No smoke run folder was found under runtime-smoke\\runs."
}

exit $tickExit
