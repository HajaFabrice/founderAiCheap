# 30-Day OPLURIX Conversion Sprint

Purpose: implement the strategy memo as weekly operating behavior, not another planning document.

## North Star

Turn warm attention into owned audience and Product 1 sales.

Primary offer:

- Expert-to-Influencer Content Engine

Primary free entry:

- Conservation Content Ethics Checklist

Primary weekly metric:

- Email subscribers from conservation-relevant traffic

Secondary weekly metrics:

- Product page visits
- Product checkout clicks
- Payhip sales
- LinkedIn comments or DMs
- Top buyer objection

## Week 1: Clarify and Connect

Goal: make the public site pass the 8-second test.

Actions:

- Confirm homepage headline says who OPLURIX helps and what transformation it provides.
- Confirm Product 1 is the only live checkout path.
- Confirm the checklist path records a lead and opens the printable checklist on the thank-you step.
- Confirm proof appears close to the Product 1 CTA.
- Confirm the header has no more than five main choices.

Success metric:

- A new visitor can identify the main product, free checklist, and research proof without needing explanation.

Avoid:

- Adding new product pages before Product 1 gets traffic and buyer feedback.

## Week 2: Publish and Capture

Goal: send real traffic into the cleaned funnel.

Actions:

- Publish at least three LinkedIn posts.
- Use one first-comment link per post:
  - Cold traffic: checklist page.
  - Warm traffic: Product 1 page.
  - Research-support traffic: ATBC page.
- Reply to every relevant comment or DM within 24 hours.
- Record weekly metrics in `measurement-scoreboard-template.csv`.

Success metric:

- At least 10 checklist/update/contact signups or direct buyer conversations.

Avoid:

- Posting without a next step.

## Week 3: Strengthen Proof

Goal: lower checkout hesitation.

Actions:

- Add one buyer objection to the scoreboard.
- Turn one objection into a product-page FAQ or LinkedIn post.
- Ask early readers or buyers for one sentence of feedback.
- Publish one post explaining why OPLURIX is not generic AI content advice.

Success metric:

- At least one proof asset captured: testimonial, reply, objection, use case, or before/after example.

Avoid:

- Hiding credibility on the About page only.

## Week 4: Review and Narrow

Goal: improve the highest-signal path.

Actions:

- Compare homepage visits, Product 1 visits, checklist signups, checkout clicks, and Payhip sales.
- Identify the strongest traffic source.
- Identify the strongest CTA.
- Decide whether to keep pushing Product 1 or publish the next product.

Decision rule:

- If Product 1 is getting clicks but no sales, improve the product page and objection handling.
- If the checklist is getting signups but no product interest, improve the welcome/follow-up message.
- If LinkedIn posts get comments but no clicks, make the first-comment CTA more concrete.
- If there is no traffic, publish more before redesigning.

Success metric:

- One clear buyer path identified for the next 30 days.

Avoid:

- Launching Products 2 and 4, or reviving Product 3, as a way to avoid reading the signal from Product 1.

## Weekly Review Template

Use this every Sunday.

```text
Week of:

Traffic:
- Homepage visits:
- Product 1 page visits:
- ATBC page visits:

Capture:
- Checklist EN signups:
- Checklist FR signups:
- Updates/contact signups:
- Total email subscribers:

Sales:
- Product checkout clicks:
- Payhip checkout clicks / visits:
- Shopify store visits:
- Shopify checkout clicks:
- Payhip sales:
- Revenue:
- Top UTM campaign:

Distribution:
- LinkedIn posts published:
- Comments or DMs:
- Best-performing post:

Learning:
- Top objection:
- Best CTA:
- One change for next week:
```

## Current Public Routing Rules

- Cold traffic -> checklist page.
- Warm traffic -> Product 1 page.
- Hot traffic -> direct Payhip checkout.
- Research-support traffic -> ATBC page.
- Service traffic -> services page.

## Payhip / Shopify Analytics Review

Use Payhip and Shopify as the checkout-side source of truth. The repo site now
adds UTM parameters to Payhip and Shopify checkout links, so the weekly review
should compare:

- site-side route used: homepage, product page, checklist, ATBC page, or direct campaign link
- `utm_source`: where the buyer came from
- `utm_medium`: placement such as `first-comment`, `profile-link`, or `site`
- `utm_campaign`: campaign/post id
- `utm_content`: exact link surface
- Payhip visits, conversions, and sales
- Shopify sessions and checkout/buy-button activity if Shopify is active that week

Do not over-interpret one click. Use this to find repeated patterns.

## Stop Rule

Do not create a new product or new sales page until one of these is true:

- Product 1 has at least 10 sales.
- The same buyer objection appears three times.
- A specific audience asks for the same next product at least three times.
