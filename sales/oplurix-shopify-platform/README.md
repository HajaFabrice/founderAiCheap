# OPLURIX Shopify Marketing Platform Pack

Created: 2026-05-19

This folder is a founder-safe Shopify setup pack for OPLURIX.

It is designed to help you:

- set up Shopify first as a marketing platform for the one already-live OPLURIX product
- use Shopify as a branded front door while the active checkout remains on Payhip
- keep additional OPLURIX products unpublished until they are actually live
- optionally import commercially verified OPLURIX products in `draft` status later
- upload the existing buyer-facing PDFs and delivery ZIPs already present in the repo
- keep launch-state truth intact while you review checkout, delivery, and support rules

## Folder Map

- `01_Guides/`
  - step-by-step setup and activation guidance
- `02_Shopify_CSV/`
  - draft import CSVs and upload manifests
- `03_Upload_Files/`
  - buyer-facing PDFs and delivery ZIPs copied from current OPLURIX assets
- `04_Notes/`
  - truth rules, missing assets, and launch cautions

## Important Rule

The CSV import is intentionally conservative:

- products default to `draft`
- `Published` defaults to `FALSE`
- activation still requires human review

That protects you from accidentally publishing products whose delivery, support,
or fulfillment boundaries are not fully operational yet.

## Source Truth Used

- `documents/oplurix-site/OPLURIX_Product_Suite_Final.md`
- `sales/oplurix-product-suite/README.md`
- `documents/oplurix-site/RDigitalProduct/EcoR_Toolkit_v3/Payhip_Product_Packages/`

## Best Use Order

1. Read `01_Guides/00_START_HERE.md`
2. Follow `01_Guides/01_SHOPIFY_SETUP_CHECKLIST.md`
3. Adapt `01_Guides/04_SHOPIFY_LANDING_PAGE_SOURCE.html` into a Shopify page or homepage section for the live Payhip product
4. Review `01_Guides/03_MARKETING_PLATFORM_PLAYBOOK.md`
5. Keep `02_Shopify_CSV/shopify_product_import_draft.csv` for a later expansion phase
6. Review `04_Notes/MISSING_ASSETS_AND_TRUTH_RULES.md` before activating any additional product
