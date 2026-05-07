# OPLURIX Pro Launch Workspace

Updated: 2026-05-07

This folder is the operational workspace for `EcoR Toolkit Pro Bundle`.

## Current Truth

- Product: `EcoR Toolkit Pro Bundle`
- Price: `USD 297`
- Checkout: PayPal on the main storefront
- Delivery: manual email fulfillment
- Delivery window target: within `12 hours` after payment confirmation
- Support model:
  - 1 month email Q&A
  - private WhatsApp community access
  - founder-managed onboarding and tracking

## Folder Guide

- `release_support/`
  - buyer-facing Pro-specific files added to the release package
- `releases/`
  - generated release folder for `EcoR_Toolkit_Pro_v1`
- `google-drive-mirror/`
  - local mirror of the Google Drive delivery structure
- `pro_delivery_log.csv`
  - delivery, payment, and access tracking
- `pro_support_tracker.csv`
  - support-window and WhatsApp onboarding tracking
- `whatsapp_setup_checklist.md`
  - exact setup and moderation checklist for the WhatsApp group
- `pro_launch_messages.md`
  - human launch, delivery, and support messages

## Pro Delivery Workflow

1. Verify the PayPal payment.
2. Send the Pro delivery email with:
   - download link
   - support-window start and end dates
   - WhatsApp access steps
3. Record the sale in `pro_delivery_log.csv`.
4. Add the buyer to `pro_support_tracker.csv`.
5. Send the WhatsApp invite manually.
6. Confirm the buyer joined or note if they prefer email-only support.
7. Track support usage until the window closes.

## Operating Boundary

- Keep Pro live only while you are actually maintaining:
  - the delivery log
  - the support tracker
  - the WhatsApp onboarding path
- If support operations slip, pause Pro promotion before they drift into false promises.
