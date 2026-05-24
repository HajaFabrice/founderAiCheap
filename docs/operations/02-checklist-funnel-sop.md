# Checklist Funnel SOP

Purpose: make the ethics checklist funnel usable now, and easy to upgrade later.

## What exists already

- English landing page:
  [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html)
- French landing page:
  [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist-fr.html)
- Thank-you page:
  [thank-you.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/thank-you.html)
- Printable EN asset:
  [ethics-checklist-en.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/checklists/ethics-checklist-en.html)
- Printable FR asset:
  [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/checklists/ethics-checklist-fr.html)

## Current operating modes

### Mode A. Without Kit

Use this when:

- traffic is still low
- you want validation first
- you are comfortable following up manually

How it works:

1. Visitor fills form.
2. Form records:
   - campaign_name
   - cta_surface
   - language
   - interest_type
   - source_page
3. Thank-you page shows the printable checklist immediately.
4. You review submissions manually.
5. If useful, you later copy leads into your email system or spreadsheet.

### Mode B. With Kit

Use this when:

- you want automatic delivery
- you want segmented follow-up
- you want a welcome sequence

How it should work:

1. Visitor fills form.
2. Form hits Kit.
3. Kit tags the lead.
4. Kit sends the checklist email automatically.
5. Kit starts the right welcome sequence.

## Recommended Kit tags

- `oplurix-checklist-en`
- `oplurix-checklist-fr`
- `oplurix-live-product-interest`
- `oplurix-services-interest`
- `oplurix-research-support-interest`

## File-to-field map

### Hidden fields already captured

- `campaign_name`
- `cta_surface`
- `language`
- `interest_type`
- `source_page`
- `resource_name`
- `update_track`

### Current behavior file

- [storefront.js](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/assets/storefront.js)

## Manual daily operating procedure

If you are still in no-Kit mode:

1. Open the latest form submissions.
2. Filter checklist leads only.
3. Review:
   - email
   - language
   - campaign_name
   - cta_surface
4. Add them to your lead sheet if they matter.
5. If you are doing manual follow-up that week, send the right EN or FR message.

## Weekly review procedure

Once per week:

1. Count checklist EN signups
2. Count checklist FR signups
3. Note top campaign sources
4. Note top surfaces
5. Decide:
   - keep current CTA mix
   - push more traffic
   - or connect Kit now

## Switch criteria for moving to Kit

Switch when at least one is true:

- manual follow-up is becoming unreliable
- you have repeated checklist traffic from multiple sources
- you want a real welcome sequence
- you want EN/FR separation without spreadsheet cleanup

## Done means

- checklist landing pages capture leads cleanly
- thank-you page gives a real next step
- printable asset exists
- you know exactly when you are operating manually versus with Kit
