# OPLURIX Master Improvement Roadmap

Last updated: 2026-05-23

This document is the consolidated next-step plan for OPLURIX so we do not have
to keep rediscovering the same priorities pass by pass.

It is intentionally practical:

- what is already done
- what is still relevant
- what to do next
- in what order
- in which file or platform
- what "done" looks like

Implementation companion docs now live in:

- [operations/README.md](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/operations/README.md)
- [fulfillment-sop.md](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/fulfillment-sop.md)

---

## 1. Current Truth

### Public website

The repo-based public site is no longer a rough placeholder. It already has:

- a sharper homepage funnel in [index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- a public About lane in [about.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/about.html) and [about-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/about-fr.html)
- a Projects lane in [projects.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/projects.html)
- a research campaign lane in [atbc-2026-drone-surveys.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/atbc-2026-drone-surveys.html) and [atbc-2026-drone-surveys-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/atbc-2026-drone-surveys-fr.html)
- a checklist funnel in [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html) and [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist-fr.html)
- bilingual services pages
- a mobile CTA layer
- social preview assets and a stronger visual identity

### Checkout and sales

Your current reality is still:

- one live product
- live checkout on Payhip
- Shopify used as a temporary marketing/storefront layer
- repo site already strong enough to become the long-term owned front-end later

### Product suite

The product suite now has:

- English delivery packages in
  [packages](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages)
- French delivery packages for Products 1 to 4 in
  [packages-fr](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages-fr/README.md)

### Important constraint

Do not spend more time rebuilding the site structure from scratch right now.
That work is already far enough along.

The biggest remaining opportunity is no longer "make the site better in
general."

It is:

1. connect the funnel cleanly
2. publish with operational discipline
3. finish the delivery side of the real products
4. make traffic and follow-up measurable

---

## 2. What Is Already Done And Should Not Be Reopened Right Now

Unless something breaks or metrics prove otherwise, do **not** spend the next
cycle reworking these again:

- homepage hero/nav/CTA logic
- bilingual About pages
- ATBC campaign pages
- French public product pages
- social preview image system
- mobile CTA bar behavior
- French service-page naturalization

These are not perfect forever, but they are already past the "worth another
full redesign pass" threshold.

The next gains are operational, not architectural.

---

## 3. Master Priority Order

This is the order that makes the most sense now.

### Priority 1

Publish and operationalize what already exists.

### Priority 2

Connect the lead magnet and follow-up system properly.

### Priority 3

Align Shopify, Payhip, and the repo site so there is no contradiction between
them.

### Priority 4

Finish the delivery-side assets for the actual product business.

### Priority 5

Prepare the later Netlify migration without forcing it now.

---

## 4. Step-By-Step Execution Plan

## Phase A. Freeze The Truth Of The Offer

### Goal

Make sure every public surface says the same true thing.

### What to do

1. Treat this as the current canonical business truth:
   - one live product:
     `Expert-to-Influencer Content Engine`
   - checkout currently lives on Payhip
   - Shopify is a marketing/storefront layer for now
   - Products 2 to 4 are next-release items, not fully live open checkout
   - Products 5 to 10 are still earlier-stage

2. Check that these surfaces all reflect the same truth:
   - Shopify About section
   - Shopify homepage/banner text
   - Payhip Product 1 page
   - repo homepage
   - product page
   - Facebook page bio/about
   - LinkedIn profile links

3. If any of them still imply a larger live catalog than reality, correct them.

### Files/platforms

- [index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- [01-expert-to-influencer-content-engine.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/01-expert-to-influencer-content-engine.html)
- Shopify admin
- Payhip product listing

### Definition of done

No public surface implies:

- a fully live 10-product store
- native Shopify checkout as the current truth
- instant delivery of assets that are not actually connected yet

---

## Phase B. Deploy And Make The Repo Site Operational

### Goal

Turn the current repo site from "well built locally" into a reliably usable
public asset.

### What to do

1. Choose the current live host for the repo site.
   - If staying on GitHub Pages temporarily, keep it stable.
   - If using Netlify later, do not switch yet unless the email/funnel layer is
     ready too.

2. Deploy the current public site state.

3. Verify these live routes after deployment:
   - homepage
   - About EN/FR
   - Projects
   - Field Notes
   - ATBC EN/FR
   - Product 1 EN/FR
   - ethics checklist EN/FR
   - thank-you page

4. Refresh social preview caches for:
   - homepage
   - ATBC EN
   - ATBC FR

### Files/platforms

- [sitemap.xml](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/sitemap.xml)
- [robots.txt](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/robots.txt)
- current host

### Definition of done

- all major pages load publicly
- preview images update correctly when shared
- no dead internal routes remain in the public funnel

---

## Phase C. Complete The Lead Magnet Funnel Properly

### Goal

Stop treating lead capture as "good enough" and make it a real pipeline.

### Current state

The checklist pages and forms already exist, and they already capture campaign
metadata.

What is still missing is a proper email delivery and follow-up system.

### What to do

1. Create the actual email system.
   Recommended path:
   - create the Kit account
   - create the inline form
   - define the tag structure before connecting anything

2. Suggested tag structure:
   - `oplurix-checklist-en`
   - `oplurix-checklist-fr`
   - `oplurix-live-product-interest`
   - `oplurix-services-interest`
   - `oplurix-research-support-interest`

3. Upload the final checklist asset that will actually be delivered.

4. Replace temporary Netlify-only capture behavior with one of these:
   - direct Kit form action
   - or Netlify submission plus manual Kit sync

5. Connect the thank-you page to the real next step.
   - checklist signups -> checklist delivery + welcome sequence
   - product interest -> product follow-up
   - service inquiry -> direct reply expectation

6. Remove `noindex` on the checklist pages only when:
   - the promised asset exists
   - the promised delivery path exists
   - the welcome flow exists

### Files/platforms

- [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html)
- [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist-fr.html)
- [thank-you.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/thank-you.html)
- [storefront.js](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/assets/storefront.js)
- Kit / ConvertKit

### Definition of done

- a signup produces the promised asset
- the lead lands in the right segment/tag
- the visitor gets the right thank-you state
- the welcome sequence actually starts

---

## Phase D. Cleanly Align Shopify, Payhip, And The Repo Site

### Goal

Make the whole sales surface feel like one system even though checkout is split.

### What to do

1. Decide the role of each surface clearly.

   Repo site:
   - owned authority hub
   - research proof
   - deep storytelling
   - lead capture

   Shopify:
   - polished marketing storefront while subscription remains active
   - one live offer, plus clearly labeled upcoming products

   Payhip:
   - actual checkout for the live product

2. In Shopify:
   - use the new About copy
   - use the Facebook cover/brand visuals where relevant
   - keep the live product primary
   - label Products 2 to 4 carefully if shown
   - avoid pretending they are instantly deliverable if they are not

3. In the repo site:
   - keep the direct Payhip checkout link where buyer intent is strongest
   - keep softer visitors on the product page or checklist

4. In Payhip:
   - update Product 1 with the current guarantee language
   - make sure the description matches the repo product page tone

5. Create one "surface logic" note for yourself:
   - cold traffic -> checklist
   - warm traffic -> product page
   - hot traffic -> direct checkout

### Files/platforms

- Shopify admin
- Payhip admin
- [campaign-link-map.md](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/campaign-link-map.md)
- [01-expert-to-influencer-content-engine.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/01-expert-to-influencer-content-engine.html)

### Definition of done

- the visitor does not feel thrown between unrelated systems
- all copy agrees on what is live and what is next
- checkout links are used intentionally, not randomly

---

## Phase E. Launch Products 2 To 4 Cleanly

### Goal

Move from one live product to a small, believable release set.

### What to do

1. Finalize the public launch state for:
   - Product 2
   - Product 3
   - Product 4

2. For each product, prepare:
   - final pricing confirmation
   - final guarantee position if any
   - final delivery bundle
   - final public status wording

3. Choose the order of activation.
   Recommended:
   - Product 2 first
   - Product 3 second
   - Product 4 third

4. Why this order:
   - Product 2 is low-friction and practical
   - Product 3 has strong utility but benefits from clearer category framing
   - Product 4 is higher-value and needs stronger pitch credibility

5. Before any listing goes live, check:
   - English package exists and is clean enough
   - French package exists if you want bilingual delivery
   - product page does not oversell the current delivery format
   - support boundary is acceptable

### Files/platforms

- [packages](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages)
- [packages-fr](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages-fr/README.md)
- [products/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/index.html)
- Shopify or Payhip listings

### Definition of done

- Products 2 to 4 are not just "nice pages"
- they have real delivery assets
- they have real launch states
- they can be sold without confusion

---

## Phase F. Finish The Delivery-Side Product Operations

### Goal

Make the product business operational, not just public-facing.

### What to do

1. Create ZIP-ready deliverables for the French packages for Products 1 to 4.

2. Optionally create PDF editions for the main French guides if you want a more
   polished customer format.

3. Add one operator checklist per product:
   - what to send
   - what to verify
   - what language version to use
   - what support boundary to restate if needed

4. Decide delivery naming conventions now.

   Suggested structure:

   - `OPLURIX_Product1_Content_Engine_FR.zip`
   - `OPLURIX_Product2_Training_Quiz_FR.zip`
   - `OPLURIX_Product3_Equipment_Concierge_FR.zip`
   - `OPLURIX_Product4_Pitch_Deck_FR.zip`

5. Build a tiny fulfillment log template for yourself:
   - buyer name
   - date
   - product
   - language
   - delivery sent
   - follow-up due

### Files/platforms

- [packages-fr](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages-fr/README.md)
- optional new fulfillment log in `sales/` or `docs/operations/`

### Definition of done

- you can fulfill a French or English order quickly
- no manual delivery feels improvised
- no buyer gets the wrong language pack by mistake

---

## Phase G. Build The Welcome And Follow-Up Sequence

### Goal

Make every captured lead move somewhere useful.

### What to do

1. Implement the 5-email welcome flow from the optimization plan.

2. Split it into at least two tracks:
   - checklist/newsletter track
   - live product buyer-intent track

3. Keep research-support and service leads separate from product leads.

4. Add at least one French-friendly version or a bilingual adaptation for
   French leads.

5. Make the first sequence extremely simple:
   - Email 1: delivery + orientation
   - Email 2: credibility + founder story
   - Email 3: useful insight
   - Email 4: product relevance
   - Email 5: action invitation

### Files/platforms

- Kit / ConvertKit
- [OPLURIX_Optimization_Plan.docx](/C:/Users/Student/Desktop/perso/founderAiCheap/documents/oplurix-site/Hormozi/OPLURIX_Optimization_Plan.docx)

### Definition of done

- a new lead receives useful follow-up automatically
- the sequence does not mix unrelated intents
- you have at least one path from signup to purchase

---

## Phase H. Use The ATBC Campaign More Intentionally

### Goal

Make the research campaign strengthen the business instead of living beside it.

### What to do

1. Treat ATBC content as a credibility engine, not only a support request.

2. Use three CTA types consistently:
   - read the research story
   - support the travel/research
   - buy the product that supports the work

3. Create one operator schedule for campaign reuse:
   - LinkedIn long-form
   - LinkedIn shorter post
   - Facebook page version
   - WhatsApp share version
   - French version

4. Reuse the ATBC-specific share preview whenever the campaign page is shared.

5. Make sure the direct PayPal support link and product support path remain
   clearly separate.

### Files/platforms

- [atbc-2026-drone-surveys.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/atbc-2026-drone-surveys.html)
- [atbc-2026-drone-surveys-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/atbc-2026-drone-surveys-fr.html)
- [campaign-link-map.md](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/campaign-link-map.md)

### Definition of done

- the campaign increases trust and visibility
- it does not blur donations and product sales
- the support paths are easy to understand

---

## Phase I. Build A Simple Measurement Layer

### Goal

Know what is actually working without overbuilding analytics.

### What to do

1. Track at minimum:
   - homepage visits
   - clicks to Payhip
   - checklist signups
   - service inquiries
   - ATBC page visits
   - PayPal support clicks

2. Keep campaign naming consistent.
   Use the same style everywhere:
   - source
   - language
   - surface
   - offer intent

3. Create one simple reporting table that you update manually weekly if needed.

   Suggested columns:

   - week
   - homepage clicks to checkout
   - checklist signups EN
   - checklist signups FR
   - service inquiries
   - ATBC support clicks
   - product sales

4. Do not wait for perfect analytics to start learning.

### Files/platforms

- current host analytics
- Netlify analytics later if migrated
- Kit stats
- manual spreadsheet if simplest

### Definition of done

- you can answer "which link/path is working?" without guessing

---

## Phase J. Prepare The Later Netlify Migration Quietly

### Goal

Be ready to move cleanly later without rushing now.

### What to do

1. Keep the repo site host-agnostic as much as possible.

2. Before switching to Netlify, make sure:
   - checklist email system is working
   - forms are stable
   - domain decision is made
   - preview images and sitemap point to the right base URL

3. When you are ready to switch:
   - update absolute URLs
   - update sitemap
   - update robots
   - update campaign links
   - re-run social preview checks

4. Do not migrate only for style.
   Migrate when the funnel and domain strategy are ready together.

### Files/platforms

- [sitemap.xml](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/sitemap.xml)
- [robots.txt](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/robots.txt)
- [campaign-link-map.md](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/campaign-link-map.md)

### Definition of done

- migration changes hosting, not business logic
- no campaign links break during the move

---

## Phase K. Delay Wave 2 Product Translation Until The Source Stabilizes

### Goal

Avoid wasting time translating unstable internal products.

### What to do

1. Do not translate Products 5 to 10 into actual French delivery packs yet.

2. First stabilize each one in English:
   - final scope
   - final format
   - final support boundary
   - final public state

3. Only then translate the actual product deliverable, not just the sales page.

### Files/platforms

- [packages](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages)
- [packages-fr](/C:/Users/Student/Desktop/perso/founderAiCheap/sales/oplurix-product-suite/packages-fr/README.md)

### Definition of done

- you do not create French rework for products that are still moving too much

---

## 5. Immediate 14-Day Sprint

If you want the shortest serious execution path, do these next:

### Day 1 to 2

1. Deploy the repo site cleanly
2. Verify the main public pages
3. Refresh share previews

### Day 3 to 4

4. Set up Kit / ConvertKit
5. Connect the checklist funnel properly
6. confirm the welcome sequence structure

### Day 5 to 6

7. Align Shopify, Payhip, and the repo site messaging
8. update the Payhip Product 1 guarantee block if still pending

### Day 7 to 9

9. package the French delivery bundles for Products 1 to 4
10. decide Product 2 launch readiness

### Day 10 to 12

11. publish the first controlled traffic push:
   - one LinkedIn EN
   - one LinkedIn FR
   - one Facebook EN/FR path

### Day 13 to 14

12. review the first data
13. decide whether Product 2 activates next or whether the checklist funnel
    needs one more fix first

---

## 6. What Not To Do Next

Do not spend the next cycle on these unless something is broken:

- another homepage redesign
- another full visual identity pass
- translating Products 5 to 10 now
- moving to Shopify checkout now
- migrating to Netlify before the email funnel is ready
- overengineering analytics before basic tracking is being used

---

## 7. If You Only Do Three Things

If time, money, or energy gets tight, do only this:

1. deploy and use the current repo site publicly
2. connect the checklist funnel to real email delivery
3. operationalize Product 1 plus the French delivery packs for Products 1 to 4

That alone would move OPLURIX from "promising ecosystem" to "working public
marketing and fulfillment system."

---

## 8. Recommended Next File To Update After Reading This

If you want this roadmap to immediately become action, the next single file or
system to touch should be:

- the actual email capture/delivery setup behind
  [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html)

Because right now that is the highest-leverage unfinished part of the system.
