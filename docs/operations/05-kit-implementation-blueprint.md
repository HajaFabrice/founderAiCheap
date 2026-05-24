# Kit Implementation Blueprint

Purpose: connect OPLURIX lead capture to Kit without guessing on form names,
tags, or automation logic.

Use this only when you are ready to move from manual/no-Kit mode into real email
delivery and follow-up.

## 1. What Kit should do for OPLURIX

Kit is not the checkout tool.

For OPLURIX, Kit should do four things:

1. collect leads
2. label them correctly
3. send the promised resource automatically
4. start the right follow-up sequence

## 2. Forms to create in Kit

Create these forms first.

### Form A

- Internal name: `OPLURIX Checklist EN`
- Purpose: English checklist leads
- Primary route:
  [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html)

### Form B

- Internal name: `OPLURIX Checklist FR`
- Purpose: French checklist leads
- Primary route:
  [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist-fr.html)

### Form C

- Internal name: `OPLURIX Updates General`
- Purpose: homepage updates form
- Primary route:
  [index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)

### Form D

- Internal name: `OPLURIX Product Interest`
- Purpose: homepage product/contact interest
- Primary route:
  [index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)

## 3. Tags to create in Kit

Create these exact tags.

### Checklist tags

- `oplurix-checklist-en`
- `oplurix-checklist-fr`

### Intent tags

- `oplurix-live-product-interest`
- `oplurix-services-interest`
- `oplurix-research-support-interest`
- `oplurix-general-updates`

### Lifecycle tags

- `oplurix-checklist-delivered`
- `oplurix-welcome-sequence-active`
- `oplurix-buyer-prospect`

## 4. Custom fields to create in Kit

Create these text fields so campaign context is not lost.

- `campaign_name`
- `cta_surface`
- `source_page`
- `interest_type`
- `resource_name`
- `update_track`
- `site_language`

## 5. Form-to-tag map

### Checklist EN form

On submit:

- add tag `oplurix-checklist-en`
- add tag `oplurix-buyer-prospect`
- set `resource_name = 3-Minute Ethics Checklist`
- set `site_language = en`

### Checklist FR form

On submit:

- add tag `oplurix-checklist-fr`
- add tag `oplurix-buyer-prospect`
- set `resource_name = Checklist ethique de 3 minutes`
- set `site_language = fr`

### Updates form

On submit:

- add tag `oplurix-general-updates`
- set fields from form payload

### Product interest form

On submit:

- add tag `oplurix-live-product-interest`
- set fields from form payload

If the message is clearly about services or research:

- add the relevant secondary tag manually or later via automation rules

## 6. Minimum automations to build

If using Kit free/basic mode, keep the first setup simple.

### Automation 1. Checklist EN

Trigger:

- form submit `OPLURIX Checklist EN`

Actions:

1. add tag `oplurix-checklist-en`
2. add tag `oplurix-welcome-sequence-active`
3. deliver checklist email
4. start EN checklist welcome sequence

### Automation 2. Checklist FR

Trigger:

- form submit `OPLURIX Checklist FR`

Actions:

1. add tag `oplurix-checklist-fr`
2. add tag `oplurix-welcome-sequence-active`
3. deliver checklist email
4. start FR checklist welcome sequence or bilingual variant

### Automation 3. Product interest

Trigger:

- form submit `OPLURIX Product Interest`

Actions:

1. add tag `oplurix-live-product-interest`
2. send one short acknowledgement email if desired
3. optionally queue a manual reply task

## 7. Recommended email delivery logic

### Checklist EN

Email 1 should include:

- the printable checklist link:
  [ethics-checklist-en.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/checklists/ethics-checklist-en.html)
- one line on why it matters
- one line on the full paid system

### Checklist FR

Email 1 should include:

- the printable checklist link:
  [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/checklists/ethics-checklist-fr.html)
- one line on why it matters
- one line on the full paid system

## 8. Which repo forms map to which Kit forms

Use the field map file:

- [kit-field-map.csv](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/operations/kit-field-map.csv)

## 9. Migration strategy

Do not replace everything at once.

Use this order:

1. Checklist EN
2. Checklist FR
3. Homepage updates form
4. Homepage product/contact form

That way the highest-value lead path gets automated first.

## 10. Done means

- every form has a known Kit target
- every lead gets a meaningful tag
- EN and FR are separated cleanly
- checklist delivery no longer depends on memory
