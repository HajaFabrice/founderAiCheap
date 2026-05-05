param(
    [string]$SuiteJson = "documents\99_Agent_Ready\databases\oplurix_product_suite.json",
    [string]$MatrixJson = "documents\99_Agent_Ready\databases\oplurix_launch_matrix.json",
    [string]$DocsRoot = "docs\products",
    [string]$PackagesRoot = "sales\oplurix-product-suite\packages",
    [string]$ZipsRoot = "sales\oplurix-product-suite\zips",
    [string]$ReadmePath = "sales\oplurix-product-suite\README.md"
)

$ErrorActionPreference = "Stop"
$script:WorkspaceRoot = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($script:WorkspaceRoot)) {
    $script:WorkspaceRoot = (Get-Location).Path
}

function Resolve-WorkspacePath {
    param([string]$RelativePath)

    if ([string]::IsNullOrWhiteSpace($RelativePath)) {
        throw "Path input cannot be empty."
    }

    if ([System.IO.Path]::IsPathRooted($RelativePath)) {
        return [System.IO.Path]::GetFullPath($RelativePath)
    }

    return [System.IO.Path]::GetFullPath((Join-Path -Path $script:WorkspaceRoot -ChildPath $RelativePath))
}

function Ensure-ParentDirectory {
    param([string]$FilePath)

    $parent = [System.IO.Path]::GetDirectoryName($FilePath)
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        [System.IO.Directory]::CreateDirectory($parent) | Out-Null
    }
}

function Write-Utf8File {
    param(
        [string]$Path,
        [string]$Content
    )

    Ensure-ParentDirectory -FilePath $Path
    [System.IO.File]::WriteAllText($Path, $Content, [System.Text.Encoding]::UTF8)
}

function Escape-Html {
    param([AllowNull()][string]$Text)

    if ($null -eq $Text) {
        return [System.Net.WebUtility]::HtmlEncode("")
    }

    return [System.Net.WebUtility]::HtmlEncode($Text)
}

function Get-OptionalText {
    param(
        [AllowNull()]$Value,
        [string]$Fallback = ""
    )

    if ($null -eq $Value) {
        return $Fallback
    }

    $text = [string]$Value
    if ([string]::IsNullOrWhiteSpace($text)) {
        return $Fallback
    }

    return $text.Trim()
}

function Get-Slug {
    param($Product)

    $raw = [string]$Product.public_name
    $normalized = [regex]::Replace($raw.ToLowerInvariant(), "[^a-z0-9]+", "-").Trim("-")
    return ("{0:D2}-{1}" -f [int]$Product.suite_number, $normalized)
}

function Get-PriceLabel {
    param($Product)

    if ($Product.PSObject.Properties.Name -contains "price_usd" -and $null -ne $Product.price_usd) {
        return ('$' + [string]$Product.price_usd)
    }

    if ($Product.PSObject.Properties.Name -contains "price_range_usd" -and $null -ne $Product.price_range_usd) {
        $value = [string]$Product.price_range_usd
        $parts = $value.Split('-', 2)
        if ($parts.Count -eq 2) {
            return ('$' + $parts[0] + '-$' + $parts[1])
        }
        return ('$' + $value)
    }

    return "NEEDS_HUMAN_VERIFICATION"
}

function Get-StatusMeta {
    param($Product, $MatrixItem)

    $status = [string]$Product.status
    $decision = Get-OptionalText -Value $MatrixItem.launch_decision -Fallback "review_before_launch"

    if ($status -eq "live") {
        return @{
            Label = "Live now"
            Css = "is-live"
            Note = "This product can be sold now with the current lightweight checkout and manual delivery flow."
            CTA = "Buy from the main storefront"
            Href = "../index.html#products"
        }
    }

    if ($status -eq "coming_soon") {
        return @{
            Label = "Waitlist / coming soon"
            Css = "is-waitlist"
            Note = "This product is ready enough to show, but checkout stays off until packaging and delivery are verified."
            CTA = "Join the waitlist"
            Href = "../index.html#contact"
        }
    }

    $cta = "Request early product review"
    if ($decision -eq "future_flagship_service") {
        $cta = "Request a founder briefing"
    }

    return @{
        Label = "Internal launch draft"
        Css = "is-internal"
        Note = "This page is a founder-ready sellable draft. Keep it source-only unless the founder explicitly activates it."
        CTA = $cta
        Href = "../index.html#contact"
    }
}

function Get-DeliveryState {
    param($Product)

    switch ([string]$Product.status) {
        "live" { return "ready_for_manual_fulfillment" }
        "coming_soon" { return "packaged_waitlist_candidate" }
        default { return "internal_preview_package" }
    }
}

function Get-FormatBullets {
    param([string]$FormatCode)

    switch ($FormatCode) {
        "hybrid" {
            return @(
                "Dedicated sellable HTML page for browsing and pre-sale context.",
                "Core guide delivered as a bounded file bundle.",
                "Manual delivery with a support boundary and manifest.",
                "Best fit when the product needs both discovery and a clean packaged handoff."
            )
        }
        "pdf_plus_templates" {
            return @(
                "Core guide as a stable downloadable document.",
                "Editable templates included in the delivery package.",
                "Works well for training, workflow, or analysis assets.",
                "Best fit for first-cycle manual fulfillment."
            )
        }
        "pdf_plus_editable_slides" {
            return @(
                "Core guide in document form.",
                "Editable slide blueprint or outline in the package.",
                "Strong for donor-facing or funder-facing communication products.",
                "Easy to review, send, and update manually."
            )
        }
        "pdf_plus_workbook" {
            return @(
                "Core guide plus workbook files.",
                "Useful for process-heavy products with step-by-step execution.",
                "Printable and offline-friendly.",
                "Good for archival or field data workflows."
            )
        }
        "pdf_plus_checklists" {
            return @(
                "Core guide plus editable planning checklists.",
                "Strong offline and field-operations fit.",
                "Easy to package in a bounded bundle.",
                "Simple to support manually."
            )
        }
        "pdf_plus_code" {
            return @(
                "Core guide plus code, schemas, or technical companion files.",
                "Works for more technical buyer workflows.",
                "Supports offline review and reproducible handoff.",
                "Should be paired with clear support boundaries."
            )
        }
        "hybrid_high_ticket" {
            return @(
                "Sell with a strong HTML explanation page.",
                "Deliver with documents, templates, and structured evidence files.",
                "Best for high-ticket or institution-facing offers.",
                "Not a lightweight instant-download product."
            )
        }
        default {
            return @(
                "Hybrid sales and delivery surface.",
                "Bounded manual package.",
                "Clear support boundary.",
                "Founder-controlled updates."
            )
        }
    }
}

function New-HtmlList {
    param(
        [string[]]$Items,
        [string]$ClassName = "meta-list"
    )

    $safeItems = @($Items | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
    if ($safeItems.Count -eq 0) {
        return "<ul class=""$ClassName""><li>NEEDS_HUMAN_VERIFICATION</li></ul>"
    }

    $lines = $safeItems | ForEach-Object { "              <li>$(Escape-Html $_)</li>" }
    return @"
            <ul class="$ClassName">
$($lines -join "`n")
            </ul>
"@
}

function Build-ProductPage {
    param($Product, $MatrixItem, [string]$Slug)

    $statusMeta = Get-StatusMeta -Product $Product -MatrixItem $MatrixItem
    $priceLabel = Get-PriceLabel -Product $Product
    $deliveryState = Get-DeliveryState -Product $Product
    $audienceList = New-HtmlList -Items @($Product.primary_audience) -ClassName "meta-list"
    $formatList = New-HtmlList -Items (Get-FormatBullets -FormatCode ([string]$Product.recommended_client_format)) -ClassName "deliverable-list"
    $sourceList = New-HtmlList -Items @($Product.source_artifacts | ForEach-Object {
        ([System.IO.Path]::GetFileName([string]$_) + " - " + [string]$_)
    }) -ClassName "source-list"
    $mainBlocker = Get-OptionalText -Value $MatrixItem.main_blocker -Fallback "none"
    $activationGate = Get-OptionalText -Value $MatrixItem.minimum_activation_gate -Fallback "Founder review required."
    $launchDecision = Get-OptionalText -Value $MatrixItem.launch_decision -Fallback "review_before_launch"

    return @"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>OPLURIX | $(Escape-Html $Product.public_name)</title>
  <meta name="description" content="$(Escape-Html $Product.core_promise)">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@300;400;500;600&family=Playfair+Display:wght@600;700&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="../assets/oplurix-store.css">
</head>
<body>
  <header class="site-header">
    <div class="shell topbar">
      <a class="brand" href="../index.html">
        <span class="brand-mark">OPLURIX</span>
        <span class="brand-sub">Conservation tools funding ERIS</span>
      </a>
      <nav class="nav">
        <a href="../index.html">Main storefront</a>
        <a href="index.html">Product suite</a>
        <a href="../project-docs.html">Project docs</a>
      </nav>
      <div class="header-actions">
        <a class="mini-cta" href="../index.html#contact">Ask a question</a>
      </div>
    </div>
  </header>

  <main>
    <section class="hero">
      <div class="shell hero-grid">
        <div class="page-hero-copy">
          <a class="page-backlink" href="index.html">Back to the OPLURIX product suite</a>
          <span class="status-chip $($statusMeta.Css)">$($statusMeta.Label)</span>
          <h1>$(Escape-Html $Product.public_name)</h1>
          <p class="hero-text">$(Escape-Html $Product.core_promise)</p>
          <p class="inline-note">$($statusMeta.Note)</p>
          <div class="hero-actions">
            <a class="button button-primary" href="$($statusMeta.Href)">$($statusMeta.CTA)</a>
            <a class="button button-secondary" href="../index.html#contact">Ask before buying</a>
          </div>
        </div>

        <aside class="hero-panel">
          <div class="panel-card">
            <div class="panel-label">Price</div>
            <div class="hero-price">$priceLabel</div>
            <p>Launch decision: $(Escape-Html $launchDecision)</p>
            <p>Delivery state: $(Escape-Html $deliveryState)</p>
          </div>
          <div class="panel-card panel-card-soft">
            <div class="panel-label">Recommended format</div>
            <h2>$(Escape-Html $Product.recommended_client_format)</h2>
            <p>$(Escape-Html $Product.format_reason)</p>
          </div>
        </aside>
      </div>
    </section>

    <section class="section">
      <div class="shell">
        <div class="section-label">Fit</div>
        <h2>Who this product is designed for</h2>
        <div class="meta-grid">
          <article class="meta-card">
            <h3>Primary audience</h3>
$audienceList
          </article>
          <article class="meta-card">
            <h3>Activation gate</h3>
            <p>$(Escape-Html $activationGate)</p>
          </article>
        </div>
      </div>
    </section>

    <section class="section section-plain">
      <div class="shell">
        <div class="section-label">Hybrid recommendation</div>
        <h2>How the sellable and deliverable versions work together</h2>
        <div class="deliverable-grid">
          <article class="deliverable-card">
            <h3>Sellable version</h3>
            <p>This HTML page explains the product clearly, stays easy to update, and gives the buyer a safe next step without pretending the package is bigger than it is.</p>
          </article>
          <article class="deliverable-card">
            <h3>Deliverable version</h3>
            <p>The paired package in <code>sales/oplurix-product-suite/packages/$Slug</code> is the bounded handoff that should be zipped and fulfilled once the founder activates the product.</p>
          </article>
          <article class="deliverable-card">
            <h3>Why hybrid</h3>
$formatList
          </article>
        </div>
      </div>
    </section>

    <section class="section section-plain">
      <div class="shell">
        <div class="section-label">Source basis</div>
        <h2>What this version was built from</h2>
        <div class="source-grid">
          <article class="source-card">
            <h3>Source artifacts</h3>
$sourceList
          </article>
          <article class="source-card">
            <h3>Current launch truth</h3>
            <p>Current state: $(Escape-Html $Product.status)</p>
            <p>Recommended public state: $(Escape-Html $MatrixItem.recommended_public_state)</p>
            <p>Main blocker: $(Escape-Html $mainBlocker)</p>
          </article>
        </div>
      </div>
    </section>

    <section class="section">
      <div class="shell">
        <div class="section-label">Next step</div>
        <h2>Founder-safe activation rule</h2>
        <div class="next-step-grid">
          <article class="callout-card">
            <h3>Before public checkout</h3>
            <p>$(Escape-Html $activationGate)</p>
          </article>
          <article class="callout-card">
            <h3>Current buyer-safe CTA</h3>
            <p>Use the main storefront or contact flow first. The package should stay manual and auditable until the activation gate is cleared.</p>
          </article>
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="shell footer-grid">
      <div>
        <div class="brand-mark">OPLURIX</div>
        <p>Hybrid product page generated from the OPLURIX suite references on 2026-05-05.</p>
      </div>
      <div class="footer-links">
        <a href="../index.html">Main storefront</a>
        <a href="index.html">Full suite</a>
      </div>
      <div class="footer-links">
        <a href="mailto:hajafabriceeris@gmail.com">hajafabriceeris@gmail.com</a>
        <a href="tel:+261349414112">+261349414112</a>
      </div>
    </div>
  </footer>
</body>
</html>
"@
}

function Build-ProductOverview {
    param($Product, $MatrixItem, [string]$Slug)

    $priceLabel = Get-PriceLabel -Product $Product
    $formatBullets = @((Get-FormatBullets -FormatCode ([string]$Product.recommended_client_format)) | ForEach-Object { "- $_" }) -join "`n"
    $audienceBullets = @($Product.primary_audience | ForEach-Object { "- $_" }) -join "`n"
    $artifactBullets = @($Product.source_artifacts | ForEach-Object { "- $_" }) -join "`n"
    $mainBlocker = Get-OptionalText -Value $MatrixItem.main_blocker -Fallback "none"

    return @"
# $( $Product.public_name )

Generated: 2026-05-05

## Public Status

- Current state: `$( $Product.status )`
- Recommended public state: `$( $MatrixItem.recommended_public_state )`
- Launch decision: `$( $MatrixItem.launch_decision )`
- Activation priority: `$( $MatrixItem.activation_priority )`
- Delivery state: `$( Get-DeliveryState -Product $Product )`

## Price

- $priceLabel

## Core Promise

$( $Product.core_promise )

## Primary Audience

$audienceBullets

## Recommended Hybrid Delivery Shape

- Code: `$( $Product.recommended_client_format )`
- Reason: $( $Product.format_reason )

$formatBullets

## Activation Gate

$( $MatrixItem.minimum_activation_gate )

## Main Blocker

$mainBlocker

## Source Artifacts

$artifactBullets

## Package Rule

This package is the deliverable-side preview for the matching sellable page:

- `docs/products/$Slug.html`
"@
}

$suitePath = Resolve-WorkspacePath $SuiteJson
$matrixPath = Resolve-WorkspacePath $MatrixJson
$script:ResolvedDocsRoot = Resolve-WorkspacePath $DocsRoot
$script:ResolvedPackagesRoot = Resolve-WorkspacePath $PackagesRoot
$script:ResolvedZipsRoot = Resolve-WorkspacePath $ZipsRoot
$script:ResolvedReadmePath = Resolve-WorkspacePath $ReadmePath

$suite = Get-Content -LiteralPath $suitePath -Raw | ConvertFrom-Json
$matrix = Get-Content -LiteralPath $matrixPath -Raw | ConvertFrom-Json

$matrixLookup = @{}
foreach ($item in $matrix.products) {
    $matrixLookup[[string]$item.product_id] = $item
}

[System.IO.Directory]::CreateDirectory($script:ResolvedDocsRoot) | Out-Null
[System.IO.Directory]::CreateDirectory($script:ResolvedPackagesRoot) | Out-Null
[System.IO.Directory]::CreateDirectory($script:ResolvedZipsRoot) | Out-Null

$catalogCards = New-Object System.Collections.Generic.List[string]
$packageSummary = New-Object System.Collections.Generic.List[string]

foreach ($product in $suite.products) {
    $productId = [string]$product.id
    if (-not $matrixLookup.ContainsKey($productId)) {
        throw "Missing launch matrix entry for $productId"
    }

    $matrixItem = $matrixLookup[$productId]
    $slug = Get-Slug -Product $product
    $statusMeta = Get-StatusMeta -Product $product -MatrixItem $matrixItem
    $priceLabel = Get-PriceLabel -Product $product
    $deliveryState = Get-DeliveryState -Product $product

    $pagePath = Join-Path (Resolve-WorkspacePath $DocsRoot) ($slug + ".html")
    Write-Utf8File -Path $pagePath -Content (Build-ProductPage -Product $product -MatrixItem $matrixItem -Slug $slug)

    $packageDir = Join-Path (Resolve-WorkspacePath $PackagesRoot) $slug
    $sourceAssetsDir = Join-Path $packageDir "02_Source_Assets"
    $notesDir = Join-Path $packageDir "03_Notes"

    if (Test-Path -LiteralPath $packageDir) {
        Remove-Item -LiteralPath $packageDir -Recurse -Force
    }

    [System.IO.Directory]::CreateDirectory($sourceAssetsDir) | Out-Null
    [System.IO.Directory]::CreateDirectory($notesDir) | Out-Null

    $startHere = @"
# $( $product.public_name )

This is the deliverable-side package preview for `$( $product.public_name )`.

- Public state: `$( $product.status )`
- Recommended public state: `$( $matrixItem.recommended_public_state )`
- Launch decision: `$( $matrixItem.launch_decision )`
- Delivery state: `$( $deliveryState )`

## Open These First

1. `README.md`
2. `01_PRODUCT_OVERVIEW.md`
3. `03_Notes/SUPPORT_BOUNDARY.md`
4. `02_Source_Assets/`

## What This Package Is For

$( $product.core_promise )

## What This Package Does Not Replace

- scientific judgment
- project-specific supervision
- technical verification
- a founder approval decision about whether the product is ready for public checkout
"@

    $packageReadme = @"
# $( $product.public_name ) Delivery Package

- Package slug: `$slug`
- Sellable page: `docs/products/$slug.html`
- Delivery format recommendation: `$( $product.recommended_client_format )`
- Activation gate: $( $matrixItem.minimum_activation_gate )

Use this folder as the founder-controlled delivery bundle that pairs with the
matching sellable HTML page.
"@

    $supportBoundary = @"
# Support Boundary

This package is designed to help the buyer move faster through a clearly bounded
workflow.

It does not provide:

- custom consulting beyond the packaged scope
- project-specific technical signoff
- automatic scientific correctness
- any promise that exceeds the files included here

If the founder activates this product for sale, delivery should remain bounded,
auditable, and manually supportable.
"@

    $deliveryNotes = @"
# Delivery Notes

- Recommended client format: `$( $product.recommended_client_format )`
- Format reason: $( $product.format_reason )
- Public state: `$( $product.status )`
- Recommended public state: `$( $matrixItem.recommended_public_state )`
- Main blocker: $( Get-OptionalText -Value $matrixItem.main_blocker -Fallback 'none' )

If this product moves to checkout later, keep the sellable surface in HTML and
the delivered bundle in a bounded package.
"@

    $overview = Build-ProductOverview -Product $product -MatrixItem $matrixItem -Slug $slug

    Write-Utf8File -Path (Join-Path $packageDir "00_START_HERE.md") -Content $startHere
    Write-Utf8File -Path (Join-Path $packageDir "README.md") -Content $packageReadme
    Write-Utf8File -Path (Join-Path $packageDir "01_PRODUCT_OVERVIEW.md") -Content $overview
    Write-Utf8File -Path (Join-Path $notesDir "SUPPORT_BOUNDARY.md") -Content $supportBoundary
    Write-Utf8File -Path (Join-Path $notesDir "DELIVERY_NOTES.md") -Content $deliveryNotes

    $manifest = [ordered]@{
        generated_at = "2026-05-05"
        product_id = $productId
        public_name = [string]$product.public_name
        suite_number = [int]$product.suite_number
        current_state = [string]$product.status
        recommended_public_state = [string]$matrixItem.recommended_public_state
        launch_decision = [string]$matrixItem.launch_decision
        activation_priority = [int]$matrixItem.activation_priority
        delivery_state = $deliveryState
        recommended_client_format = [string]$product.recommended_client_format
        price = $priceLabel
        activation_gate = [string]$matrixItem.minimum_activation_gate
        source_artifacts = @($product.source_artifacts)
        sellable_page = ("docs/products/{0}.html" -f $slug)
    }
    Write-Utf8File -Path (Join-Path $packageDir "DELIVERY_MANIFEST.json") -Content ($manifest | ConvertTo-Json -Depth 6)

    foreach ($artifact in $product.source_artifacts) {
        $artifactPath = Resolve-WorkspacePath ([string]$artifact)
        if (Test-Path -LiteralPath $artifactPath) {
            $artifactTarget = Join-Path $sourceAssetsDir ([System.IO.Path]::GetFileName($artifactPath))
            Copy-Item -LiteralPath $artifactPath -Destination $artifactTarget -Force
        }
    }

    $zipPath = Join-Path (Resolve-WorkspacePath $ZipsRoot) ($slug + ".zip")
    if (Test-Path -LiteralPath $zipPath) {
        Remove-Item -LiteralPath $zipPath -Force
    }
    Compress-Archive -Path (Join-Path $packageDir "*") -DestinationPath $zipPath -CompressionLevel Optimal -Force

    $catalogCards.Add(@"
          <article class="catalog-card">
            <span class="product-status">$([System.Net.WebUtility]::HtmlEncode($statusMeta.Label))</span>
            <h3>$(Escape-Html $product.public_name)</h3>
            <span class="product-price">$priceLabel</span>
            <p>$(Escape-Html $product.core_promise)</p>
            $(New-HtmlList -Items @(
                "Launch decision: $([string]$matrixItem.launch_decision)",
                "Format: $([string]$product.recommended_client_format)",
                "Package state: $deliveryState"
            ) -ClassName "catalog-list")
            <div class="product-actions">
              <a class="button button-primary" href="$slug.html">Open sellable page</a>
            </div>
          </article>
"@) | Out-Null

    $packageSummary.Add("- $($product.public_name) -> docs/products/$slug.html -> sales/oplurix-product-suite/packages/$slug -> sales/oplurix-product-suite/zips/$slug.zip") | Out-Null
}

$catalogHtml = @"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>OPLURIX | Hybrid product suite</title>
  <meta name="description" content="Sellable and deliverable hybrid versions of the 10 OPLURIX product concepts.">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@300;400;500;600&family=Playfair+Display:wght@600;700&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="../assets/oplurix-store.css">
</head>
<body>
  <header class="site-header">
    <div class="shell topbar">
      <a class="brand" href="../index.html">
        <span class="brand-mark">OPLURIX</span>
        <span class="brand-sub">Conservation tools funding ERIS</span>
      </a>
      <nav class="nav">
        <a href="../index.html">Main storefront</a>
        <a href="../project-docs.html">Project docs</a>
      </nav>
      <div class="header-actions">
        <a class="mini-cta" href="../index.html#contact">Contact</a>
      </div>
    </div>
  </header>

  <main>
    <section class="hero">
      <div class="shell hero-grid">
        <div class="page-hero-copy">
          <a class="page-backlink" href="../index.html">Back to the main storefront</a>
          <span class="status-chip is-waitlist">Hybrid suite pages</span>
          <h1>Sellable pages plus deliverable packages for all 10 OPLURIX product concepts.</h1>
          <p class="hero-text">This catalog is the founder-ready bridge between product source documents and real client-facing delivery packages.</p>
          <div class="hero-actions">
            <a class="button button-primary" href="../index.html#products">Open main storefront</a>
            <a class="button button-secondary" href="../index.html#contact">Ask before launch</a>
          </div>
        </div>

        <aside class="hero-panel">
          <div class="panel-card">
            <div class="panel-label">Live now</div>
            <p>Expert-to-Influencer Content Engine</p>
          </div>
          <div class="panel-card panel-card-soft">
            <div class="panel-label">Sellable next</div>
            <p>Training-to-Quiz Generator, then Biodiversity Pitch Deck Builder.</p>
          </div>
        </aside>
      </div>
    </section>

    <section class="section section-plain">
      <div class="shell">
        <div class="section-label">Catalog</div>
        <h2>Each product now has a sellable page and a matching delivery package.</h2>
        <div class="catalog-grid">
$($catalogCards -join "`n")
        </div>
      </div>
    </section>
  </main>

  <footer class="site-footer">
    <div class="shell footer-grid">
      <div>
        <div class="brand-mark">OPLURIX</div>
        <p>Generated from the OPLURIX suite and launch matrix references on 2026-05-05.</p>
      </div>
      <div class="footer-links">
        <a href="../index.html">Main storefront</a>
      </div>
      <div class="footer-links">
        <a href="mailto:hajafabriceeris@gmail.com">hajafabriceeris@gmail.com</a>
        <a href="tel:+261349414112">+261349414112</a>
      </div>
    </div>
  </footer>
</body>
</html>
"@

$catalogRoot = Resolve-WorkspacePath $DocsRoot
Write-Utf8File -Path (Join-Path $catalogRoot "index.html") -Content $catalogHtml

$suiteReadme = @"
# OPLURIX Hybrid Product Suite

Generated: 2026-05-05

This folder contains the deliverable-side package previews for the 10 OPLURIX
product concepts.

Each product now has:

- a sellable HTML page in `docs/products/`
- a delivery package folder in `sales/oplurix-product-suite/packages/`
- a ZIP preview in `sales/oplurix-product-suite/zips/`

## Generated Package Map

$($packageSummary -join "`n")

## Rule

These packages do not automatically change public launch status.

- `live` products can use the package immediately
- `coming_soon` products still need founder approval before checkout activation
- `source_only` products remain internal unless the founder promotes them to a
  waitlist or live state
"@

$resolvedReadmePath = Resolve-WorkspacePath $ReadmePath
Write-Utf8File -Path $resolvedReadmePath -Content $suiteReadme

Write-Output "Generated OPLURIX hybrid suite pages in $DocsRoot"
Write-Output "Generated OPLURIX delivery packages in $PackagesRoot"
Write-Output "Generated OPLURIX package ZIPs in $ZipsRoot"
