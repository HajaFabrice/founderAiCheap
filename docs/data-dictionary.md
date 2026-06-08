# Data Dictionary

This is the canonical naming reference for the OPLURIX/FounderAI repo-based platform.

Before adding any new form field, query parameter, metric, JSON key, product id, runtime key, table name, or event name, check this file first. If a needed name is missing, add it here before or during the implementation.

## Naming Rules

- Use `snake_case` for data keys, form fields, CSV columns, JSON keys, and future database fields.
- Use `kebab-case` for public file names, product slugs, URL paths, and campaign URL values.
- Use stable ids instead of display names when logic depends on a value.
- Do not create synonyms for an existing field. Extend the existing field or document a new one here.
- Keep public tracking fields human-readable enough to audit manually.
- Do not store secrets in public `docs/` assets.

## Future Database Tables

These are reserved table names if the project later moves from static files and CSVs into Supabase, SQLite, Postgres, or another database.

| table_name | Purpose |
| --- | --- |
| `leads` | People captured through Netlify, Kit, contact, waitlist, or checklist forms. |
| `products` | Canonical product catalog and launch state. |
| `orders` | Purchases from Payhip, PayPal, Shopify, Stripe, or manual checkout. |
| `deliveries` | Manual or automated product delivery records. |
| `campaign_events` | Clicks, submissions, CTA events, and source attribution. |
| `weekly_metrics` | Weekly scoreboard rows used for conversion review. |
| `support_contributions` | Direct research support through PayPal, Mvola, Sendwave, or manual channels. |
| `content_assets` | Public pages, PDFs, lead magnets, social visuals, videos, and product files. |
| `founderai_runs` | FounderAI runtime metadata from `runtime/runs/<run_id>/metadata.json`. |
| `approvals` | Protected action approvals and decisions. |

## Canonical Form Fields

These names are already used on the public site. Reuse them exactly.

| field_name | Type | Allowed or expected values | Notes |
| --- | --- | --- | --- |
| `form-name` | string | Netlify form name | Required by Netlify forms. Keep the hyphen because Netlify expects it. |
| `bot-field` | string | empty | Honeypot field. Never use for real data. |
| `name` | string | free text | Visitor name. Optional unless a specific form requires it. |
| `email` | string | valid email | Primary owned-audience identifier. |
| `message` | string | free text | Contact or product-interest message. |
| `source_page` | string | page or section id | Examples: `homepage-updates`, `ethics-checklist-top`, `ethics-checklist-fr-bottom`. |
| `language` | string | `en`, `fr`, `en-fr`, `mixed` | Site or submission language. |
| `campaign_name` | string | kebab-case campaign id | Stored version of `campaign` or `campaign_name` URL parameter. |
| `cta_surface` | string | kebab-case surface id | Examples: `organic-post`, `first-comment`, `profile-link`, `onsite-form`. |
| `interest_type` | string | `checklist`, `product`, `updates`, `contact`, `research`, `services`, `general` | Primary intent classification. |
| `resource_name` | string | resource title | Example: `3-Minute Ethics Checklist`. |
| `update_track` | string | track id | Use when a visitor asks for updates tied to research, products, or general OPLURIX work. |
| `product_interest` | string | product id or display name | Prefer product ids from the product catalog below. |

## Campaign Query Parameters

The site accepts short query parameters and stores canonical hidden fields.

| URL parameter | Stored field | Notes |
| --- | --- | --- |
| `campaign` | `campaign_name` | Preferred public short form. |
| `campaign_name` | `campaign_name` | Accepted fallback. |
| `surface` | `cta_surface` | Preferred public short form. |
| `cta_surface` | `cta_surface` | Accepted fallback. |
| `lang` | `language` | Preferred public short form. |
| `language` | `language` | Accepted fallback. |
| `interest` | `interest_type` | Preferred public short form. |
| `interest_type` | `interest_type` | Accepted fallback. |

## Attribution Fields

Use these on outbound checkout/support links and direct campaign links. Payhip and
Shopify can use these query parameters in their own analytics, while the repo
keeps the naming auditable.

| field_name | Type | Notes |
| --- | --- | --- |
| `utm_source` | string | Traffic origin such as `linkedin`, `facebook`, `whatsapp`, `email`, or `oplurix-site`. |
| `utm_medium` | string | Placement or channel type such as `first-comment`, `profile-link`, `site`, or `direct-link`. |
| `utm_campaign` | string | Campaign slug such as `linkedin-en-post-10` or `product1-direct-checkout`. |
| `utm_content` | string | Specific page/link surface such as `index__get-the-content-engine-39`. |
| `utm_term` | string | Optional intent or language marker such as `product`, `checklist`, `en`, or `fr`. |
| `conversion_platform` | string | `payhip`, `shopify`, `paypal`, or `manual`. |
| `conversion_event` | string | Event name from the reserved event table below. |

## Netlify And Kit Field Map

These fields align the current Netlify forms with the future Kit setup.

| repo_surface | language | current_form_name | future_kit_form_name | primary_tag | secondary_tag | resource_name | thank_you_variant |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ethics-checklist-top` | `en` | `ethics-checklist-interest` | `OPLURIX Checklist EN` | `oplurix-checklist-en` | `oplurix-buyer-prospect` | `3-Minute Ethics Checklist` | `checklist` |
| `ethics-checklist-bottom` | `en` | `ethics-checklist-interest` | `OPLURIX Checklist EN` | `oplurix-checklist-en` | `oplurix-buyer-prospect` | `3-Minute Ethics Checklist` | `checklist` |
| `ethics-checklist-fr-top` | `fr` | `ethics-checklist-interest` | `OPLURIX Checklist FR` | `oplurix-checklist-fr` | `oplurix-buyer-prospect` | `Checklist ethique de 3 minutes` | `checklist` |
| `ethics-checklist-fr-bottom` | `fr` | `ethics-checklist-interest` | `OPLURIX Checklist FR` | `oplurix-checklist-fr` | `oplurix-buyer-prospect` | `Checklist ethique de 3 minutes` | `checklist` |
| `homepage-updates` | `en-fr` | `oplurix-updates` | `OPLURIX Checklist + Updates` | `oplurix-checklist-homepage` | `oplurix-general-updates` | `Conservation Content Ethics Checklist` | `checklist` |
| `homepage-contact` | `en-fr` | `oplurix-product-interest` | `OPLURIX Product Interest` | `oplurix-live-product-interest` | empty | depends on `message` | `contact` |

## Product Catalog Fields

Use these keys for product pages, package manifests, checkout configuration, and future product tables.

| field_name | Type | Notes |
| --- | --- | --- |
| `product_id` | string | Canonical stable product id. Prefer the `oplurix_XX_slug` form in delivery manifests. |
| `id` | string | Short storefront id used in `docs/assets/storefront.js`. |
| `suite_number` | integer | Product number from 1 to 10. |
| `slug` | string | URL-safe page/package slug. |
| `public_name` | string | Human-facing product name. |
| `name_en` | string | English display name. |
| `name_fr` | string | French display name. |
| `status` | string | `live`, `coming_soon`, `source_only`, `internal_only`. |
| `storefront_visibility` | string | `live`, `waitlist_only`, `internal_only`. |
| `checkout_enabled` | boolean | True only after payment and delivery path are verified. |
| `price_usd` | number | Public USD price when a single confirmed price exists. |
| `display_price` | string | Public or draft display price. Use for ranges, held products, and source-only pages. |
| `payhip_url` | string | Payhip checkout URL when Payhip is active. |
| `paypal_url` | string | PayPal checkout or support URL when used. |
| `page_path` | string | Public product page path. |
| `delivery_package_path` | string | Sellable package folder path. |
| `french_package_path` | string | French package folder path when it exists. |
| `zip_path` | string | ZIP delivery preview path. |
| `recommended_client_format` | string | Examples: `hybrid`, `pdf_plus_templates`, `pdf_plus_editable_slides`, `pdf_plus_code`. |
| `format_reason` | string | Why that delivery format fits the product. |
| `core_promise` | string | One-sentence transformation promise. |
| `primary_audience` | array | Primary buyers or users. |
| `launch_decision` | string | Examples: `keep_live`, `sellable_next`, `waitlist_only`, `hold`. |
| `activation_priority` | integer | Lower number means higher activation priority. |
| `minimum_activation_gate` | string | What must be true before checkout goes live. |
| `main_blocker` | string | Current reason not to activate. |
| `agent_rule` | string | How FounderAI or assistants may describe the product. |

## Current Product Ids

| suite_number | short_id | product_id | public_name | status | display_price |
| --- | --- | --- | --- | --- | --- |
| 1 | `content-engine` | `oplurix_01_expert_to_influencer_content_engine` | Expert-to-Influencer Content Engine | `live` | 39 |
| 2 | `training-to-quiz-generator` | `oplurix_02_training_to_quiz_generator` | Training-to-Quiz Generator | `coming_soon` | 29 |
| 3 | `gear-equipment-concierge` | `oplurix_03_gear_and_equipment_concierge` | Gear & Equipment Concierge | `source_only` | 39 |
| 4 | `biodiversity-pitch-deck-builder` | `oplurix_04_biodiversity_pitch_deck_builder` | Biodiversity Pitch Deck Builder | `coming_soon` | 69 |
| 5 | `handwritten-log-digitizer` | `oplurix_05_handwritten_log_digitizer` | Handwritten Log Digitizer | `source_only` | 39 |
| 6 | `field-mission-planner` | `oplurix_06_field_mission_planner` | Field Mission Planner | `source_only` | 49 |
| 7 | `biodiversity-data-harmonizer` | `oplurix_07_biodiversity_data_harmonizer` | Biodiversity Data Harmonizer | `source_only` | 99 |
| 8 | `thermal-data-translator` | `oplurix_08_thermal_data_translator` | Thermal Data Translator | `source_only` | 149 |
| 9 | `grant-writing-co-pilot` | `oplurix_09_grant_writing_co_pilot` | Grant Writing Co-Pilot | `source_only` | 199 |
| 10 | `mrv-report-architect` | `oplurix_10_mrv_report_architect` | MRV Report Architect | `source_only` | 499-999 |

## Weekly Metrics

The current weekly scoreboard uses these exact CSV columns:

| metric_name | Type | Notes |
| --- | --- | --- |
| `week_start` | date | Start date for the weekly review period. |
| `homepage_visits` | integer | Homepage visits. |
| `homepage_primary_cta_clicks` | integer | Main homepage CTA clicks. |
| `product_page_visits` | integer | Product page visits, especially Product 1. |
| `product_checkout_clicks` | integer | Clicks toward checkout. |
| `payhip_checkout_clicks` | integer | Attributed clicks or visits recorded for Payhip checkout. |
| `shopify_store_visits` | integer | Shopify sessions or storefront visits when Shopify is in use. |
| `shopify_checkout_clicks` | integer | Shopify checkout or buy-button clicks when Shopify is in use. |
| `checklist_signups_en` | integer | English checklist submissions. |
| `checklist_signups_fr` | integer | French checklist submissions. |
| `updates_or_contact_signups` | integer | General updates or contact form submissions. |
| `total_email_subscribers` | integer | Total owned audience count. |
| `linkedin_posts_published` | integer | LinkedIn posts published during the week. |
| `linkedin_comments_or_dms` | integer | Relevant comments or direct messages. |
| `atbc_page_visits` | integer | ATBC research campaign page visits. |
| `paypal_support_clicks` | integer | PayPal support link clicks. |
| `payhip_sales` | integer | Payhip sales count. |
| `revenue_usd` | number | Revenue in USD. |
| `top_utm_campaign` | string | Highest-signal campaign in Payhip, Shopify, or manual review. |
| `top_objection` | string | Most important hesitation heard that week. |
| `notes` | string | Operator notes. |

## FounderAI Runtime Fields

Use these keys when reading or writing run metadata.

| field_name | Type | Notes |
| --- | --- | --- |
| `run_id` | string | Folder id under `runtime/runs/`. |
| `job_id` | string | Job or inbox request id. |
| `agent_id` | string or null | Agent id when run is agent-specific. |
| `role_id` | string or null | Team role lane when run is role-specific. |
| `task_type` | string | Router task type such as `draft`, `briefing`, `grant`, `qa_check`. |
| `trigger` | string | What started the run, such as `inbox_request` or scheduled job. |
| `provider` | string | `ollama`, `claude`, `anthropic`, or `openai`. |
| `model` | string | Provider model name. |
| `route_summary` | string | Human-readable router decision. |
| `started_at` | datetime | Run start time. |
| `finished_at` | datetime | Run finish time. |
| `exit_code` | integer | `0` means success. Non-zero means blocked or failed. |
| `worker_timeout_seconds` | integer | Timeout applied to provider call. |
| `prompt_chars` | integer | Prompt character count. |
| `prompt_words` | integer | Prompt word count. |
| `usage` | object or null | Provider usage if available. |
| `request_source` | string or null | Path to originating inbox request. |
| `team_output_file` | string or null | Team output artifact path if applicable. |

## Approval Fields

| field_name | Type | Allowed values or notes |
| --- | --- | --- |
| `approval_id` | string | Stable approval file id. |
| `approval_policy` | string | `never`, `before_run`, `after_run`. |
| `approval_tag` | string | `external-send`, `publish`, `financial`, `destructive-write`, `calendar-commitment`. |
| `pending_approval` | string or null | Approval id waiting for review. |
| `approval_status` | string | `pending`, `approved`, `rejected`, `expired`. |
| `approved_by` | string | Human reviewer if tracked. |
| `approved_at` | datetime | Approval timestamp if approved. |

## Reserved Event Names

Use these for lightweight site-side event naming and future analytics tools.

| event_name | Trigger |
| --- | --- |
| `homepage_primary_cta_click` | Visitor clicks the main homepage CTA. |
| `checklist_form_submit` | Visitor submits the checklist form. |
| `updates_form_submit` | Visitor joins updates. |
| `product_interest_submit` | Visitor sends product/contact interest. |
| `product_page_view` | Visitor views a product page. |
| `product_checkout_click` | Visitor clicks toward Payhip, PayPal, Shopify, or Stripe checkout. |
| `paypal_support_click` | Visitor clicks the ATBC direct support PayPal link. |
| `payhip_checkout_click` | Visitor clicks the active Payhip checkout. |
| `shopify_checkout_click` | Visitor clicks a Shopify checkout or Shopify storefront buying path. |
| `delivery_confirmed` | Product delivery is confirmed manually or automatically. |
