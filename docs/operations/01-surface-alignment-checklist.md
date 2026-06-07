# Surface Alignment Checklist

Purpose: make sure every public-facing surface tells the same true story.

## Current canonical truth

- One live product:
  `Expert-to-Influencer Content Engine`
- Live checkout currently runs through Payhip.
- Shopify is a temporary storefront/marketing layer.
- The repo site is the authority/proof/funnel layer.
- Products 2 and 4 are the next release candidates.
- Product 3 is visible but held/source-only until equipment specs, pricing, and update rules are refreshed.
- Products 5 to 10 are not to be presented as actively sellable now.

## Step-by-step audit

### 1. Shopify

Check:

- homepage hero
- About section
- navigation
- live product card
- upcoming product cards

Confirm:

- Product 1 is clearly primary
- Products 2 and 4 are labeled as upcoming, next release, or similar
- Product 3 is labeled as held, source-only, or needing refresh if shown
- no copy implies a fully live 10-product store
- no copy implies Shopify is the actual current checkout if Payhip is still the truth

### 2. Payhip

Check:

- Product 1 title
- Product 1 description
- Product 1 guarantee wording
- delivery expectation wording

Confirm:

- description tone matches the repo product page
- guarantee language is current
- no contradiction with the repo site

### 3. Repo site

Check:

- [index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- [01-expert-to-influencer-content-engine.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/01-expert-to-influencer-content-engine.html)
- [products/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/index.html)
- [about.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/about.html)
- [projects.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/projects.html)

Confirm:

- Product 1 is the only live offer
- products 2 and 4 are framed as next-release products
- product 3 is framed as held/source-only until refreshed
- products 5 to 10 stay exploratory or internal-state where appropriate

### 4. Social surfaces

Check:

- Facebook page bio/about
- LinkedIn profile links
- pinned post text if any
- Shopify profile links

Confirm:

- cold traffic goes to checklist or homepage
- warm traffic goes to product page
- hot traffic goes to direct checkout

## Routing rules

- Cold traffic -> checklist landing page
- Warm traffic -> repo product page
- Hot traffic -> direct Payhip checkout
- Research-support traffic -> ATBC page
- Service traffic -> services page

## Done means

- all surfaces agree
- no public page oversells the catalog
- no visitor is sent to the wrong destination for their temperature
