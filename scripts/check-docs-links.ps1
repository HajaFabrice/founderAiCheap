param(
    [string]$DocsPath = "$(Join-Path $PSScriptRoot '..\docs')",
    [int]$MaxIssues = 200,
    [switch]$CheckAnchors
)

$ErrorActionPreference = "Stop"

$docsRoot = (Resolve-Path -LiteralPath $DocsPath).Path
$issues = New-Object System.Collections.Generic.List[string]

function Resolve-LocalTarget {
    param(
        [string]$Url,
        [string]$SourceDirectory
    )

    $clean = ($Url -split "[?#]")[0]
    if ([string]::IsNullOrWhiteSpace($clean) -or $clean.StartsWith("#")) {
        return $null
    }

    $decoded = [System.Uri]::UnescapeDataString($clean)
    $pathText = $decoded -replace "/", [IO.Path]::DirectorySeparatorChar

    if ($decoded.StartsWith("/")) {
        $target = Join-Path $docsRoot ($pathText.TrimStart([IO.Path]::DirectorySeparatorChar))
    } else {
        $target = Join-Path $SourceDirectory $pathText
    }

    if (Test-Path -LiteralPath $target -PathType Container) {
        return Join-Path $target "index.html"
    }

    return $target
}

function Test-Anchor {
    param(
        [string]$TargetPath,
        [string]$Url
    )

    if (-not $CheckAnchors) {
        return $true
    }

    $fragment = ($Url -split "#", 2)
    if ($fragment.Count -lt 2 -or [string]::IsNullOrWhiteSpace($fragment[1])) {
        return $true
    }

    $anchor = ($fragment[1] -split "[?]", 2)[0]
    $anchor = [System.Uri]::UnescapeDataString($anchor)
    $html = Get-Content -LiteralPath $TargetPath -Raw
    $escaped = [regex]::Escape($anchor)

    return $html -match "(?i)\bid\s*=\s*[""']$escaped[""']" -or
        $html -match "(?i)\bname\s*=\s*[""']$escaped[""']"
}

Get-ChildItem -LiteralPath $docsRoot -Recurse -File -Include *.html | ForEach-Object {
    $sourceFile = $_.FullName
    $sourceDir = $_.DirectoryName
    $text = Get-Content -LiteralPath $sourceFile -Raw
    $links = [regex]::Matches($text, "(?i)(?:href|src)\s*=\s*[""']([^""']+)[""']")

    foreach ($link in $links) {
        $url = $link.Groups[1].Value.Trim()

        if ([string]::IsNullOrWhiteSpace($url)) {
            continue
        }

        if ($url -match "^(https?:|mailto:|tel:|javascript:|data:|#)") {
            continue
        }

        $target = Resolve-LocalTarget -Url $url -SourceDirectory $sourceDir
        if (-not $target) {
            continue
        }

        if (-not (Test-Path -LiteralPath $target -PathType Leaf)) {
            $relativeSource = Resolve-Path -LiteralPath $sourceFile -Relative
            $issues.Add("$relativeSource -> $url")
            continue
        }

        if (-not (Test-Anchor -TargetPath $target -Url $url)) {
            $relativeSource = Resolve-Path -LiteralPath $sourceFile -Relative
            $issues.Add("$relativeSource -> missing anchor in $url")
        }
    }
}

if ($issues.Count -eq 0) {
    Write-Host "Docs link check passed."
    exit 0
}

Write-Host "Docs link check failed."
$issues | Select-Object -First $MaxIssues | ForEach-Object {
    Write-Host "- $_"
}

if ($issues.Count -gt $MaxIssues) {
    Write-Host "... and $($issues.Count - $MaxIssues) more issue(s)."
}

exit 1
