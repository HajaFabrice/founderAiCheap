param(
    [string]$Repo = ""
)

function Resolve-Repository {
    param([string]$Value)

    if ($Value) {
        return $Value
    }

    $remote = (git config --get remote.origin.url).Trim()
    if (-not $remote) {
        throw "Could not resolve the GitHub repository from remote.origin.url."
    }

    if ($remote -match 'github\.com[:/](.+?)(?:\.git)?$') {
        return $Matches[1]
    }

    throw "Unsupported GitHub remote URL: $remote"
}

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    throw "GitHub CLI is not installed. Install it first, then run this script."
}

$resolvedRepo = Resolve-Repository -Value $Repo
$null = gh auth status
if ($LASTEXITCODE -ne 0) {
    throw "GitHub CLI is installed but not authenticated."
}

$labels = @(
    @{ name = "good first issue"; color = "7057ff"; description = "Good entry point for a first contribution." },
    @{ name = "help wanted"; color = "008672"; description = "Maintainer would welcome outside help." },
    @{ name = "documentation"; color = "0e8a16"; description = "Docs, guides, onboarding, or public-facing explanations." },
    @{ name = "devops"; color = "5319e7"; description = "CI, deployment, infrastructure, or hosting work." },
    @{ name = "approvals"; color = "b60205"; description = "Touches approval gates or protected action review." },
    @{ name = "founder-voice"; color = "d93f0b"; description = "Touches founder identity, tone, or prompt fidelity." },
    @{ name = "provider"; color = "1d76db"; description = "Touches Ollama, OpenAI, or provider abstractions." },
    @{ name = "windows"; color = "0052cc"; description = "Windows-specific behavior or tooling." },
    @{ name = "linux"; color = "006b75"; description = "Linux-specific behavior or tooling." },
    @{ name = "smoke-test"; color = "fbca04"; description = "Smoke tests, validation flow, or artifact inspection." },
    @{ name = "needs-triage"; color = "c2e0c6"; description = "Needs maintainer review before it is categorized." }
)

foreach ($label in $labels) {
    $encoded = [uri]::EscapeDataString($label.name)
    gh api "repos/$resolvedRepo/labels/$encoded" --silent *> $null
    if ($LASTEXITCODE -eq 0) {
        gh api --method PATCH "repos/$resolvedRepo/labels/$encoded" -f new_name=$label.name -f color=$label.color -f description=$label.description *> $null
    } else {
        gh api --method POST "repos/$resolvedRepo/labels" -f name=$label.name -f color=$label.color -f description=$label.description *> $null
    }
}

$existingTitles = @{}
$existingIssues = gh issue list --repo $resolvedRepo --state all --limit 200 --json title | ConvertFrom-Json
foreach ($issue in $existingIssues) {
    $existingTitles[$issue.title] = $true
}

$issues = @(
    @{
        title = "Add Rust unit tests for config parsing and provider overrides"
        labels = @("good first issue", "help wanted", "provider")
        body = @"
## Goal

Add small, deterministic tests around config parsing and environment override precedence.

## Acceptance

- Tests cover `config/founderai.example.json`
- Tests cover environment overrides for provider, base URL, model, timeout, and API key env name
- `cargo test --release` stays green
"@
    },
    @{
        title = "Improve provider timeout logging and recovery hints"
        labels = @("help wanted", "provider", "smoke-test")
        body = @"
## Goal

Make slow or stalled provider generations easier to diagnose from the generated artifacts.

## Acceptance

- `stderr.txt` and `output.md` make timeout failures easier to understand
- recovery guidance distinguishes slow Ollama from missing model and bad API key cases
- the isolated smoke workflow remains unchanged
"@
    },
    @{
        title = "Add README screenshots or artifact examples"
        labels = @("good first issue", "documentation")
        body = @"
## Goal

Add a few concrete examples so contributors can see what a healthy run looks like.

## Acceptance

- README includes one or more artifact examples or screenshots
- examples do not leak secrets or private founder content
- examples match the current Rust runtime behavior
"@
    },
    @{
        title = "Verify Docker deployment on a Linux host"
        labels = @("help wanted", "devops", "linux")
        body = @"
## Goal

Validate the current Docker deployment story on a real Linux machine.

## Acceptance

- confirm the app builds and starts with the provided compose files
- document any gaps or fixes needed for Ollama and OpenAI modes
- record the exact commands and host assumptions
"@
    },
    @{
        title = "Add a branchless onboarding guide for first-time contributors"
        labels = @("good first issue", "documentation")
        body = @"
## Goal

Create a simple path for first-time contributors who want to make a small change without deep git knowledge.

## Acceptance

- guide is stored in `docs/`
- it covers a docs-only or script-only first contribution path
- it links back to the main contribution guide
"@
    },
    @{
        title = "Review founder-brain folder structure and index the references"
        labels = @("good first issue", "documentation", "founder-voice")
        body = @"
## Goal

Document the founder-brain reference structure so contributors understand the content without reshaping it.

## Acceptance

- add an index file that describes the main founder-brain reference groups
- do not rewrite or relabel the founder-brain ideology itself
- explain how the reference structure maps into prompt packets
"@
    }
)

foreach ($issue in $issues) {
    if (-not $existingTitles.ContainsKey($issue.title)) {
        $args = @("issue", "create", "--repo", $resolvedRepo, "--title", $issue.title, "--body", $issue.body)
        foreach ($label in $issue.labels) {
            $args += @("--label", $label)
        }
        gh @args *> $null
    }
}

Write-Host "GitHub labels and starter issues have been synced for $resolvedRepo."
Write-Host "Finish the admin-only steps in docs/github-admin-checklist.md."
