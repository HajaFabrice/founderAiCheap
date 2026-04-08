param(
    [string]$Model = "qwen2.5:7b-instruct"
)

$ollama = Get-Command ollama -ErrorAction SilentlyContinue
if (-not $ollama) {
    throw "Ollama is not installed or not on PATH."
}

try {
    $listed = & ollama list 2>$null
} catch {
    throw "Ollama is installed but not responding. Start the Ollama service first."
}

if ($listed -match [regex]::Escape($Model)) {
    Write-Host "Model already installed: $Model"
    exit 0
}

Write-Host "Pulling Ollama model: $Model"
& ollama pull $Model
