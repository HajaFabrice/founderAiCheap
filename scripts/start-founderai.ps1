param(
    [string]$ConfigPath = "$(Join-Path $PSScriptRoot '..\\config\\founderai.json')",
    [string]$BinaryPath = "",
    [switch]$Foreground,
    [switch]$Once
)

$resolvedConfig = (Resolve-Path $ConfigPath).Path
$resolvedScript = (Resolve-Path $MyInvocation.MyCommand.Path).Path
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

if (-not $BinaryPath) {
    $releaseBinary = Join-Path $repoRoot "target\\release\\founderai-ollama-rust.exe"
    $debugBinary = Join-Path $repoRoot "target\\debug\\founderai-ollama-rust.exe"

    if (Test-Path $releaseBinary) {
        $BinaryPath = $releaseBinary
    } elseif (Test-Path $debugBinary) {
        $BinaryPath = $debugBinary
    } else {
        throw "FounderAI binary not found. Build it first with: cargo build --release"
    }
}

$resolvedBinary = (Resolve-Path $BinaryPath).Path

if (-not $Foreground) {
    $arguments = @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-WindowStyle", "Hidden",
        "-File", $resolvedScript,
        "-ConfigPath", $resolvedConfig,
        "-BinaryPath", $resolvedBinary,
        "-Foreground"
    )
    if ($Once) {
        $arguments += "-Once"
    }
    Start-Process -FilePath "powershell.exe" -ArgumentList $arguments -WorkingDirectory $repoRoot -WindowStyle Hidden | Out-Null
    Write-Host "FounderAI background launcher started."
    exit 0
}

$binaryArgs = @()
if ($Once) {
    $binaryArgs += "tick"
} else {
    $binaryArgs += "daemon"
}
$binaryArgs += @("--config", $resolvedConfig)

Set-Location $repoRoot
& $resolvedBinary @binaryArgs
