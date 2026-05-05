# OPLURIX Full Product Suite

Updated: 2026-05-05

This reference turns the 10 OPLURIX product source files in
`documents/oplurix-site/` into a conversation-safe agent briefing.

Use this when an agent needs to:

- explain the OPLURIX product ladder
- compare products for a buyer
- decide which product is live, coming soon, or source-only
- choose the most truthful delivery format for a client
- avoid selling assets that are not yet launch-ready

## Operating Rule

- `live` means the product may appear on the storefront with a real or manual
  checkout path.
- `coming_soon` means the product is visible and discussable, but checkout stays
  off until packaging and delivery are verified.
- `source_only` means the product exists as a strong product document, but
  should be treated as internal pipeline material rather than a public checkout
  offer.

## Current Product Map

### 1. Expert-to-Influencer Content Engine

- Status:
  - `live`
- Price:
  - `USD 39`
- Audience:
  - field biologists
  - researchers
  - conservation professionals who want credible LinkedIn visibility
- Core promise:
  - turn real field notes, research experience, and project observations into
    clear professional content without hype
- Source artifacts:
  - `documents/oplurix-site/product.html`
  - `documents/oplurix-site/ExpertToInfluencer.txt`
  - `documents/oplurix-site/OPLURIX_Product1_Expert_to_Influencer_Content_Engine.pdf`
- Best current client format:
  - `hybrid`
  - HTML for the sales page
  - PDF or ZIP bundle for delivery

### 2. Training-to-Quiz Generator

- Status:
  - `coming_soon`
- Price:
  - `USD 29`
- Audience:
  - NGO trainers
  - field team leaders
  - workshop facilitators
- Core promise:
  - turn SOPs and field training documents into validated quizzes that test real
    protocol understanding
- Source artifacts:
  - `documents/oplurix-site/OPLURIX_Product2_Training_to_Quiz_Generator.pdf`
- Best current client format:
  - `pdf_plus_templates`
  - PDF guide plus editable quiz templates

### 3. Gear & Equipment Concierge

- Status:
  - `source_only`
- Price:
  - `USD 39`
- Audience:
  - early-career field biologists
- Core promise:
  - help buyers choose the right equipment without wasting grant money or field
    season budget
- Source artifacts:
  - `documents/oplurix-site/product3.html`
- Best current client format:
  - `hybrid`
  - HTML is stronger long-term because specs and prices change
  - PDF is acceptable only as a timestamped snapshot

### 4. Biodiversity Pitch Deck Builder

- Status:
  - `coming_soon`
- Price:
  - `USD 69`
- Audience:
  - conservation NGOs
  - researchers seeking funding
  - protected area managers
- Core promise:
  - turn conservation data and project results into donor-ready or
    funder-ready pitch decks
- Source artifacts:
  - `documents/oplurix-site/OPLURIX_Product4_Biodiversity_Pitch_Deck_Builder.pdf`
- Best current client format:
  - `pdf_plus_editable_slides`
  - PDF guide plus editable slide template

### 5. Handwritten Log Digitizer

- Status:
  - `source_only`
- Price:
  - `USD 39`
- Audience:
  - field teams with paper-based records
  - researchers with archival notebooks
- Core promise:
  - provide a repeatable workflow from handwritten notebooks to clean,
    documented, analysis-ready datasets
- Source artifacts:
  - `documents/oplurix-site/product5.html`
- Best current client format:
  - `pdf_plus_workbook`
  - PDF guide plus spreadsheet and workflow templates

### 6. Field Mission Planner

- Status:
  - `source_only`
- Price:
  - `USD 49`
- Audience:
  - research teams
  - NGO field operations
- Core promise:
  - turn tacit field planning knowledge into a shared, reviewable mission plan
- Source artifacts:
  - `documents/oplurix-site/product6.html`
- Best current client format:
  - `pdf_plus_checklists`
  - PDF guide plus editable planning checklists

### 7. Biodiversity Data Harmonizer

- Status:
  - `source_only`
- Price:
  - `USD 99`
- Audience:
  - multi-site research projects
  - teams merging multi-season or multi-method biodiversity datasets
- Core promise:
  - give buyers a repeatable pipeline for harmonizing messy, mixed-source
    biodiversity data before analysis
- Source artifacts:
  - `documents/oplurix-site/product7.html`
- Best current client format:
  - `pdf_plus_templates`
  - PDF guide plus data schema and template files

### 8. Thermal Data Translator

- Status:
  - `source_only`
- Price:
  - `USD 149`
- Audience:
  - drone-equipped research teams
- Core promise:
  - turn raw thermal survey outputs into validated, GPS-tagged detection data
- Source artifacts:
  - `documents/oplurix-site/product8.html`
- Best current client format:
  - `pdf_plus_code`
  - PDF guide plus code or record schema files

### 9. Grant Writing Co-Pilot

- Status:
  - `source_only`
- Price:
  - `USD 199`
- Audience:
  - conservation NGOs
  - independent researchers
- Core promise:
  - turn grant writing into a repeatable system built around actual funder
    structures rather than generic advice
- Source artifacts:
  - `documents/oplurix-site/product9.html`
- Best current client format:
  - `pdf_plus_templates`
  - PDF guide plus application templates and prompt packs

### 10. MRV Report Architect

- Status:
  - `source_only`
- Price:
  - `USD 499-999`
- Audience:
  - institutions
  - REDD+ projects
  - conservation NGOs
- Core promise:
  - turn field data and the ERIS scoring framework into a structured,
    standards-aligned MRV package
- Source artifacts:
  - `documents/oplurix-site/product10.html`
- Best current client format:
  - `hybrid_high_ticket`
  - HTML or web page for the sales explanation
  - PDF and template package for delivery

## Agent Usage Rules

- If the buyer asks what can be purchased right now:
  - mention only `Expert-to-Influencer Content Engine` and the live EcoR tiers
  - you may mention `Training-to-Quiz Generator` and `Biodiversity Pitch Deck Builder`
    as waitlist products
- Do not present the other source-only products as currently purchasable unless
  the founder explicitly activates them.
- When a product is mainly a guide, framework, workflow, or template bundle:
  - PDF is the safest delivery default
- When a product depends on frequently changing specs, live examples, or
  evolving price comparisons:
  - HTML or hybrid delivery becomes stronger over time

## HTML vs PDF Decision Rule

Use `HTML` first when the client needs:

- a discoverable storefront
- mobile-first browsing
- search visibility
- embedded checkout
- forms, waitlists, or updates
- a product that changes often

Use `PDF` first when the client needs:

- a stable downloadable guide
- a printable or shareable document
- offline use in the field
- a bounded deliverable that feels complete
- easy packaging with templates inside a ZIP

Use `hybrid` when:

- HTML is best for selling
- PDF is best for delivery

For the current OPLURIX product suite, `hybrid` is the strongest default:

- HTML for storefront and pre-sale conversation
- PDF or ZIP bundles for most first-cycle delivery
