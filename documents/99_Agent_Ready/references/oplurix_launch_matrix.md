# OPLURIX Launch Matrix

Updated: 2026-05-05

This matrix answers a practical question for the agents:

- which of the 10 OPLURIX source products should stay in their current state
- which should move to waitlist
- which should become sellable next
- which should stay source-only until proof, packaging, or delivery improves

## Current Public Rule

Among the 10 products defined inside `documents/oplurix-site/`:

- `Expert-to-Influencer Content Engine` stays `live`
- `Training-to-Quiz Generator` stays `waitlist` and is the first `sellable_next`
  candidate
- `Biodiversity Pitch Deck Builder` stays `waitlist` and is the second
  `sellable_next` candidate
- the other 7 products remain `source_only` for now

## Activation Order

### 1. First sellable next

- `Training-to-Quiz Generator`

Why:

- already priced clearly
- clearly bounded buyer outcome
- best fit for PDF plus editable templates
- lower delivery complexity than the more technical products

### 2. Second sellable next

- `Biodiversity Pitch Deck Builder`

Why:

- clear donor-facing value
- strong buyer outcome
- deliverable can remain a simple PDF plus slide-template bundle

### 3. First waitlist candidate after that

- `Handwritten Log Digitizer`

Why:

- real archival and field-data pain point
- strong offline PDF-plus-workbook fit
- easier first-cycle delivery than the more technical products

## Product Matrix

| Product | Current state | Recommended public state | Launch decision | Main gate |
| --- | --- | --- | --- | --- |
| Expert-to-Influencer Content Engine | `live` | `live` | keep live | keep delivery clean and feedback-backed |
| Training-to-Quiz Generator | `coming_soon` | `waitlist` now, `sellable_next` | activate next after packaging | package templates and test buyer delivery |
| Gear & Equipment Concierge | `source_only` | keep `source_only` | defer | refresh changing specs and pricing logic first |
| Biodiversity Pitch Deck Builder | `coming_soon` | `waitlist` now, `sellable_next` after TTQ | activate second | package editable slide assets and test delivery |
| Handwritten Log Digitizer | `source_only` | move to `waitlist` later | package after next | add workbook, examples, and delivery bundle |
| Field Mission Planner | `source_only` | keep `source_only` for now | package later | extract editable checklists and planning sheets |
| Biodiversity Data Harmonizer | `source_only` | keep `source_only` for now | package later | add schemas, templates, and sample harmonization assets |
| Thermal Data Translator | `source_only` | keep `source_only` | defer until stronger proof | needs technical proof bundle and cleaner niche packaging |
| Grant Writing Co-Pilot | `source_only` | keep `source_only` | defer until stronger proof | high-stakes credibility and template proof needed |
| MRV Report Architect | `source_only` | keep `source_only` | treat as future flagship service | not a normal download-first product yet |

## Why Some Products Should Stay Source-Only

### Gear & Equipment Concierge

- useful idea, but live equipment recommendations age quickly
- needs an update process before it becomes a public product

### Thermal Data Translator

- strong technical concept
- should not be sold as a generic guide without a cleaner proof bundle and
  technical companion assets

### Grant Writing Co-Pilot

- high-stakes buyer expectations
- should move only after stronger proof and cleaner support boundaries

### MRV Report Architect

- not a lightweight digital product in practice
- better treated as a future flagship service or institutional offer

## Format Implications

The matrix also changes the format choice:

- `sellable_next` products should be optimized for `PDF plus templates`
- `source_only` products that depend on changing facts should not be rushed into
  static PDF sales without a maintenance plan
- `high-ticket` products should usually be `hybrid`
  - HTML to explain and position
  - PDF, templates, and structured documents to deliver

## Agent Rule

When discussing future OPLURIX products:

- mention `Training-to-Quiz Generator` and `Biodiversity Pitch Deck Builder`
  first
- do not push the other seven as near-term purchasable products
- use `NEEDS_HUMAN_VERIFICATION` if a buyer asks about timing, packaging, or
  checkout activation that has not been explicitly approved
