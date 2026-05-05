# Training-to-Quiz Generator

Generated: 2026-05-05

## Public Status

- Current state: $( @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.status )
- Recommended public state: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.recommended_public_state )
- Launch decision: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.launch_decision )
- Activation priority: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.activation_priority )
- Delivery state: $( Get-DeliveryState -Product @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]} )

## Price

- $29

## Core Promise

Turn SOPs and field training materials into validated quizzes that test protocol understanding.

## Primary Audience

- NGO trainers
- field team leaders
- workshop facilitators

## Recommended Hybrid Delivery Shape

- Code: $( @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.recommended_client_format )
- Reason: The client needs a stable guide plus editable quiz assets more than a dynamic app.

- Core guide as a stable downloadable document.
- Editable templates included in the delivery package.
- Works well for training, workflow, or analysis assets.
- Best fit for first-cycle manual fulfillment.

## Activation Gate

Package the templates, test payment confirmation, and verify the delivery path end to end.

## Main Blocker

Packaging and delivery are not yet verified.

## Source Artifacts

- documents/oplurix-site/OPLURIX_Product2_Training_to_Quiz_Generator.pdf

## Package Rule

This package is the deliverable-side preview for the matching sellable page:

- docs/products/02-training-to-quiz-generator.html
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
          </article> = New-Object System.Collections.Generic.List[string]
- Expert-to-Influencer Content Engine -> docs/products/01-expert-to-influencer-content-engine.html -> sales/oplurix-product-suite/packages/01-expert-to-influencer-content-engine -> sales/oplurix-product-suite/zips/01-expert-to-influencer-content-engine.zip = New-Object System.Collections.Generic.List[string]

Write-Output ("Suite product count: {0}" -f @(@{generated_at=2026-05-05; purpose=Structured reference for the 10 OPLURIX product source files in documents/oplurix-site.; operating_rule=Only live or founder-approved waitlist products may be presented as actively sellable.; status_legend=; default_format_rule=; products=System.Object[]}.products).Count)

foreach (@{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]} in @{generated_at=2026-05-05; purpose=Structured reference for the 10 OPLURIX product source files in documents/oplurix-site.; operating_rule=Only live or founder-approved waitlist products may be presented as actively sellable.; status_legend=; default_format_rule=; products=System.Object[]}.products) {
    oplurix_02_training_to_quiz_generator = [string]@{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.id
    if (-not System.Collections.Hashtable.ContainsKey(oplurix_02_training_to_quiz_generator)) {
        throw "Missing launch matrix entry for oplurix_02_training_to_quiz_generator"
    }

    @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.} = System.Collections.Hashtable[oplurix_02_training_to_quiz_generator]
    02-training-to-quiz-generator = Get-Slug -Product @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}
    System.Collections.Hashtable = Get-StatusMeta -Product @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]} -MatrixItem @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}
    $29 = Get-PriceLabel -Product @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}
    packaged_waitlist_candidate = Get-DeliveryState -Product @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}

    C:\Users\Student\Desktop\perso\founderAiCheap\docs\products\02-training-to-quiz-generator.html = Join-Path (Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\docs\products) (02-training-to-quiz-generator + ".html")
    Write-Output ("Generating page: {0}" -f C:\Users\Student\Desktop\perso\founderAiCheap\docs\products\02-training-to-quiz-generator.html)
    Write-Utf8File -Path C:\Users\Student\Desktop\perso\founderAiCheap\docs\products\02-training-to-quiz-generator.html -Content (Build-ProductPage -Product @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]} -MatrixItem @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.} -Slug 02-training-to-quiz-generator)

    C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator = Join-Path (Resolve-WorkspacePath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages) 02-training-to-quiz-generator
    Write-Output ("Generating package: {0}" -f C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator)
    C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator\02_Source_Assets = Join-Path C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator "02_Source_Assets"
    C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator\03_Notes = Join-Path C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator "03_Notes"

    if (Test-Path -LiteralPath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator) {
        Remove-Item -LiteralPath C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator -Recurse -Force
    }

    [System.IO.Directory]::CreateDirectory(C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator\02_Source_Assets) | Out-Null
    [System.IO.Directory]::CreateDirectory(C:\Users\Student\Desktop\perso\founderAiCheap\sales\oplurix-product-suite\packages\02-training-to-quiz-generator\03_Notes) | Out-Null

    # Training-to-Quiz Generator

This is the deliverable-side package preview for $( @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.public_name ).

- Public state: $( @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.status )
- Recommended public state: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.recommended_public_state )
- Launch decision: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.launch_decision )
- Delivery state: $( packaged_waitlist_candidate )

## Open These First

1. README.md
2.  1_PRODUCT_OVERVIEW.md
3.  3_Notes/SUPPORT_BOUNDARY.md
4.  2_Source_Assets/

## What This Package Is For

Turn SOPs and field training materials into validated quizzes that test protocol understanding.

## What This Package Does Not Replace

- scientific judgment
- project-specific supervision
- technical verification
- a founder approval decision about whether the product is ready for public checkout = @"
# Training-to-Quiz Generator

This is the deliverable-side package preview for $( @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.public_name ).

- Public state: $( @{id=oplurix_02_training_to_quiz_generator; suite_number=2; public_name=Training-to-Quiz Generator; status=coming_soon; price_usd=29; primary_audience=System.Object[]; core_promise=Turn SOPs and field training materials into validated quizzes that test protocol understanding.; source_formats=System.Object[]; source_artifacts=System.Object[]; storefront_visibility=waitlist_only; checkout_enabled=False; recommended_client_format=pdf_plus_templates; format_reason=The client needs a stable guide plus editable quiz assets more than a dynamic app.; agent_uses=System.Object[]}.status )
- Recommended public state: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.recommended_public_state )
- Launch decision: $( @{product_id=oplurix_02_training_to_quiz_generator; public_name=Training-to-Quiz Generator; current_state=coming_soon; recommended_public_state=waitlist_now; launch_decision=sellable_next; activation_priority=1; minimum_activation_gate=Package the templates, test payment confirmation, and verify the delivery path end to end.; main_blocker=Packaging and delivery are not yet verified.; recommended_client_format=pdf_plus_templates; agent_rule=Mention as the first next sellable product, but do not imply active checkout yet.}.launch_decision )
- Delivery state: $( packaged_waitlist_candidate )

## Open These First

1. README.md
2.  1_PRODUCT_OVERVIEW.md
3.  3_Notes/SUPPORT_BOUNDARY.md
4.  2_Source_Assets/

## What This Package Is For

Turn SOPs and field training materials into validated quizzes that test protocol understanding.

## What This Package Does Not Replace

- scientific judgment
- project-specific supervision
- technical verification
- a founder approval decision about whether the product is ready for public checkout