param(
    [string]$Source = "documents\RDigitalProduct\Gumroad_Tier_Packages\Tier_2_Complete_Toolkit",
    [string]$ReleaseSupport = "sales\oplurix-first-sale\release_support",
    [string]$ReleaseDir = "sales\oplurix-first-sale\releases\EcoR_Toolkit_Complete_v1",
    [string]$ZipPath = "sales\oplurix-first-sale\google-drive-mirror\OPLURIX Deliveries\EcoR Complete\EcoR_Toolkit_Complete_v1.zip"
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $Source)) {
    throw "Source package not found: $Source"
}

if (-not (Test-Path -LiteralPath $ReleaseSupport)) {
    throw "Release support files not found: $ReleaseSupport"
}

if (Test-Path -LiteralPath $ReleaseDir) {
    Remove-Item -LiteralPath $ReleaseDir -Recurse -Force
}

New-Item -ItemType Directory -Path $ReleaseDir -Force | Out-Null
Copy-Item -Path (Join-Path $Source "*") -Destination $ReleaseDir -Recurse -Force
Copy-Item -Path (Join-Path $ReleaseSupport "*") -Destination $ReleaseDir -Recurse -Force

$zipFolder = Split-Path -Parent $ZipPath
New-Item -ItemType Directory -Path $zipFolder -Force | Out-Null
if (Test-Path -LiteralPath $ZipPath) {
    Remove-Item -LiteralPath $ZipPath -Force
}

Compress-Archive -Path (Join-Path $ReleaseDir "*") -DestinationPath $ZipPath -CompressionLevel Optimal -Force
Write-Output "Packaged EcoR Toolkit Complete to $ZipPath"
