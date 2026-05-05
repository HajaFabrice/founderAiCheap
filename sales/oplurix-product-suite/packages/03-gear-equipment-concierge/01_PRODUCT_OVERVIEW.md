# Gear & Equipment Concierge

Generated: 2026-05-05

## Public Status

- Current state: $( @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.status )
- Recommended public state: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.recommended_public_state )
- Launch decision: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.launch_decision )
- Activation priority: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.activation_priority )
- Delivery state: $( Get-DeliveryState -Product @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]} )

## Price

- $39

## Core Promise

Help buyers choose the right field equipment without wasting budget or buying the wrong tool.

## Primary Audience

- early-career field biologists

## Recommended Hybrid Delivery Shape

- Code: $( @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.recommended_client_format )
- Reason: Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.

- Dedicated sellable HTML page for browsing and pre-sale context.
- Core guide delivered as a bounded file bundle.
- Manual delivery with a support boundary and manifest.
- Best fit when the product needs both discovery and a clean packaged handoff.

## Activation Gate

Refresh equipment specs, pricing logic, and update workflow before public launch.

## Main Blocker

Fast-changing product facts make a static first launch risky.

## Source Artifacts

- documents/oplurix-site/product3.html

## Package Rule

This package is the deliverable-side preview for the matching sellable page:

- docs/products/03-gear-equipment-concierge.html
"@
}

 = Resolve-WorkspacePath documents\99_Agent_Ready\databases\oplurix_product_suite.json
 = Resolve-WorkspacePath documents\99_Agent_Ready\databases\oplurix_launch_matrix.json
 = Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\docs\products
 = Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages
 = Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\zips
C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\README.md = Resolve-WorkspacePath sales\oplurix-product-suite\README.md

@{generated_at=2026-05-05; purpose=Structured reference for the 10 OPLURIX product source files in documents/oplurix-site.; operating_rule=Only live or founder-approved waitlist products may be presented as actively sellable.; status_legend=; default_format_rule=; products=System.Object[]} = Get-Content -LiteralPath  -Raw | ConvertFrom-Json
@{generated_at=2026-05-05; purpose=Structured launch-priority and activation-gate matrix for the 10 OPLURIX source products.; global_rule=Only founder-approved products may move from source-only to waitlist or from waitlist to active checkout.; launch_sequence=System.Object[]; products=System.Object[]} = Get-Content -LiteralPath  -Raw | ConvertFrom-Json

System.Collections.Hashtable = @{}
foreach (@{product_id=oplurix_10_mrv_report_architect; public_name=MRV Report Architect; current_state=source_only; recommended_public_state=source_only; launch_decision=future_flagship_service; activation_priority=9; minimum_activation_gate=Reframe as a flagship service or institutional offer, not a normal lightweight download.; main_blocker=Scope and buyer type are too large for a simple digital-product launch.; recommended_client_format=hybrid_high_ticket; agent_rule=Treat as future flagship commercial infrastructure, not a current downloadable product.} in @{generated_at=2026-05-05; purpose=Structured launch-priority and activation-gate matrix for the 10 OPLURIX source products.; global_rule=Only founder-approved products may move from source-only to waitlist or from waitlist to active checkout.; launch_sequence=System.Object[]; products=System.Object[]}.products) {
    System.Collections.Hashtable[[string]@{product_id=oplurix_10_mrv_report_architect; public_name=MRV Report Architect; current_state=source_only; recommended_public_state=source_only; launch_decision=future_flagship_service; activation_priority=9; minimum_activation_gate=Reframe as a flagship service or institutional offer, not a normal lightweight download.; main_blocker=Scope and buyer type are too large for a simple digital-product launch.; recommended_client_format=hybrid_high_ticket; agent_rule=Treat as future flagship commercial infrastructure, not a current downloadable product.}.product_id] = @{product_id=oplurix_10_mrv_report_architect; public_name=MRV Report Architect; current_state=source_only; recommended_public_state=source_only; launch_decision=future_flagship_service; activation_priority=9; minimum_activation_gate=Reframe as a flagship service or institutional offer, not a normal lightweight download.; main_blocker=Scope and buyer type are too large for a simple digital-product launch.; recommended_client_format=hybrid_high_ticket; agent_rule=Treat as future flagship commercial infrastructure, not a current downloadable product.}
}

[System.IO.Directory]::CreateDirectory() | Out-Null
[System.IO.Directory]::CreateDirectory() | Out-Null
[System.IO.Directory]::CreateDirectory() | Out-Null

          <article class="catalog-card">
            <span class="product-status">Live now</span>
            <h3>Expert-to-Influencer Content Engine</h3>
            <span class="product-price">$39</span>
            <p>Turn real field notes, research, and conservation work into credible, specific professional content.</p>
                        <ul class="catalog-list">
              <li>Launch decision: keep_live</li>
              <li>Format: hybrid</li>
              <li>Package state: ready_for_manual_fulfillment</li>
            </ul>
            <div class="product-actions">
              <a class="button button-primary" href="01-expert-to-influencer-content-engine.html">Open sellable page</a>
            </div>
          </article>           <article class="catalog-card">
            <span class="product-status">Waitlist / coming soon</span>
            <h3>Training-to-Quiz Generator</h3>
            <span class="product-price">$29</span>
            <p>Turn SOPs and field training materials into validated quizzes that test protocol understanding.</p>
                        <ul class="catalog-list">
              <li>Launch decision: sellable_next</li>
              <li>Format: pdf_plus_templates</li>
              <li>Package state: packaged_waitlist_candidate</li>
            </ul>
            <div class="product-actions">
              <a class="button button-primary" href="02-training-to-quiz-generator.html">Open sellable page</a>
            </div>
          </article> = New-Object System.Collections.Generic.List[string]
- Expert-to-Influencer Content Engine -> docs/products/01-expert-to-influencer-content-engine.html -> sales/oplurix-product-suite/packages/01-expert-to-influencer-content-engine -> sales/oplurix-product-suite/zips/01-expert-to-influencer-content-engine.zip - Training-to-Quiz Generator -> docs/products/02-training-to-quiz-generator.html -> sales/oplurix-product-suite/packages/02-training-to-quiz-generator -> sales/oplurix-product-suite/zips/02-training-to-quiz-generator.zip = New-Object System.Collections.Generic.List[string]

Write-Output ("Suite product count: {0}" -f @(@{generated_at=2026-05-05; purpose=Structured reference for the 10 OPLURIX product source files in documents/oplurix-site.; operating_rule=Only live or founder-approved waitlist products may be presented as actively sellable.; status_legend=; default_format_rule=; products=System.Object[]}.products).Count)

foreach (@{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]} in @{generated_at=2026-05-05; purpose=Structured reference for the 10 OPLURIX product source files in documents/oplurix-site.; operating_rule=Only live or founder-approved waitlist products may be presented as actively sellable.; status_legend=; default_format_rule=; products=System.Object[]}.products) {
    oplurix_03_gear_and_equipment_concierge = [string]@{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.id
    if (-not System.Collections.Hashtable.ContainsKey(oplurix_03_gear_and_equipment_concierge)) {
        throw "Missing launch matrix entry for oplurix_03_gear_and_equipment_concierge"
    }

    @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.} = System.Collections.Hashtable[oplurix_03_gear_and_equipment_concierge]
    03-gear-equipment-concierge = Get-Slug -Product @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}
    System.Collections.Hashtable = Get-StatusMeta -Product @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]} -MatrixItem @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}
    $39 = Get-PriceLabel -Product @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}
    internal_preview_package = Get-DeliveryState -Product @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}

    C:\Users\Student\Desktop\perso\founderAiCheap\docs\products\03-gear-equipment-concierge.html = Join-Path (Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\docs\products) (03-gear-equipment-concierge + ".html")
    Write-Output ("Generating page: {0}" -f C:\Users\Student\Desktop\perso\founderAiCheap\docs\products\03-gear-equipment-concierge.html)
    Write-Utf8File -Path C:\Users\Student\Desktop\perso\founderAiCheap\docs\products\03-gear-equipment-concierge.html -Content (Build-ProductPage -Product @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]} -MatrixItem @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.} -Slug 03-gear-equipment-concierge)

    C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge = Join-Path (Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages) 03-gear-equipment-concierge
    Write-Output ("Generating package: {0}" -f C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge)
    C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge\02_Source_Assets = Join-Path C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge "02_Source_Assets"
    C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge\03_Notes = Join-Path C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge "03_Notes"

    if (Test-Path -LiteralPath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge) {
        Remove-Item -LiteralPath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge -Recurse -Force
    }

    [System.IO.Directory]::CreateDirectory(C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge\02_Source_Assets) | Out-Null
    [System.IO.Directory]::CreateDirectory(C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\03-gear-equipment-concierge\03_Notes) | Out-Null

    # Gear & Equipment Concierge

This is the deliverable-side package preview for $( @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.public_name ).

- Public state: $( @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.status )
- Recommended public state: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.recommended_public_state )
- Launch decision: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.launch_decision )
- Delivery state: $( internal_preview_package )

## Open These First

1. README.md
2.  1_PRODUCT_OVERVIEW.md
3.  3_Notes/SUPPORT_BOUNDARY.md
4.  2_Source_Assets/

## What This Package Is For

Help buyers choose the right field equipment without wasting budget or buying the wrong tool.

## What This Package Does Not Replace

- scientific judgment
- project-specific supervision
- technical verification
- a founder approval decision about whether the product is ready for public checkout = @"
# Gear & Equipment Concierge

This is the deliverable-side package preview for $( @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.public_name ).

- Public state: $( @{id=oplurix_03_gear_and_equipment_concierge; suite_number=3; public_name=Gear & Equipment Concierge; status=source_only; price_usd=39; primary_audience=System.Object[]; core_promise=Help buyers choose the right field equipment without wasting budget or buying the wrong tool.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=internal_only; checkout_enabled=False; recommended_client_format=hybrid; format_reason=Equipment specs and prices change often, so HTML ages better; PDF is acceptable only as a timestamped guide.; agent_uses=System.Object[]}.status )
- Recommended public state: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.recommended_public_state )
- Launch decision: $( @{product_id=oplurix_03_gear_and_equipment_concierge; public_name=Gear & Equipment Concierge; current_state=source_only; recommended_public_state=source_only; launch_decision=defer; activation_priority=6; minimum_activation_gate=Refresh equipment specs, pricing logic, and update workflow before public launch.; main_blocker=Fast-changing product facts make a static first launch risky.; recommended_client_format=hybrid; agent_rule=Keep as internal pipeline only for now.}.launch_decision )
- Delivery state: $( internal_preview_package )

## Open These First

1. README.md
2.  1_PRODUCT_OVERVIEW.md
3.  3_Notes/SUPPORT_BOUNDARY.md
4.  2_Source_Assets/

## What This Package Is For

Help buyers choose the right field equipment without wasting budget or buying the wrong tool.

## What This Package Does Not Replace

- scientific judgment
- project-specific supervision
- technical verification
- a founder approval decision about whether the product is ready for public checkout