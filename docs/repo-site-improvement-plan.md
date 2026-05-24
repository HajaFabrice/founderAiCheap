# Repo-Based Site Improvement Plan

Based on the checklist in [Website-Checklist-Client-Attraction-2026.pdf](/C:/Users/Student/Desktop/perso/founderAiCheap/documents/Website-Checklist-Client-Attraction-2026.pdf)

## Goal

Turn the public OPLURIX site from a strong mission-and-proof hub into a clearer conversion system that:

- sells the one live product more reliably
- captures more qualified leads for biodiversity services and collaborations
- gives research and mission supporters a cleaner path to act
- keeps the wider OPLURIX and ERIS story visible without overwhelming first-time visitors

## What the site already does well

- It already has real proof assets instead of empty branding.
- It honestly reflects the current state: one live product, several real project lanes.
- It already supports multiple trust paths: product, services, research, field notes, and founder proof.
- It has working capture infrastructure through Netlify forms and a thank-you page.
- It has bilingual public surfaces, which is an advantage for Madagascar-grounded visibility.

## The main diagnosis

The PDF checklist was written for coach-style sites, but the core lesson transfers well: the current site has enough substance, yet it still asks new visitors to process too many ideas before taking one clear next step.

The biggest risk is not lack of content. It is conversion friction.

On the homepage especially, the site is trying to be:

- a one-product storefront
- a research visibility platform
- a mission explainer
- a services brochure
- a project map
- a content hub

All of those are valuable. The problem is that they currently compete too early.

## Audit Against the 5 Gaps

| Gap from checklist | Current state on repo site | Priority | What to change |
| --- | --- | --- | --- |
| Vague positioning | Partial | High | The homepage is honest and thoughtful, but the hero is still broad. It says what OPLURIX is, but not quickly enough who the primary visitor is and what immediate result they can get. |
| Weak messaging | Partial | High | The site carries mission language well, but it still uses more ecosystem language than client pain language in key conversion areas. |
| Confusing navigation | Weak | High | The homepage nav and page structure expose too many routes too early. New visitors have to choose between offer, projects, proof, field notes, contact, and docs before they understand the main path. |
| Missing credibility | Moderate | Medium | Proof exists, but more of it needs to sit next to decisions. Quantified outcomes, clearer before/after stories, and proof closer to CTAs would help a lot. |
| No clear CTA | Weak | High | There are many valid actions, but not one dominant action per page. Several buttons are useful, yet they compete rather than guide. |

## Highest-Impact Strategic Shift

We should define page roles much more sharply.

### 1. Homepage role

The homepage should do one main job:

**Convert the right visitor into one of two actions**

- buy the live product
- join the list or request contact if they are not ready yet

Everything else should support that job, not compete with it.

### 2. Supporting-page roles

- `projects.html`: bigger-picture context for curious visitors, collaborators, and mission-aligned followers
- `services-en.html` and `services-fr.html`: service conversion pages for biodiversity data support
- `atbc-2026-drone-surveys.html`: research credibility and support page
- `field-notes.html`: authority-building content hub
- `project-docs.html`: deep technical appendix, not a primary first-click destination

## Improvement Plan by Gap

### Gap 1. Clarify positioning in the first 8 seconds

#### Problem

The current homepage hero in [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html) is thoughtful but broad:

- "One live product. Several real projects. One place to understand the work."

That is honest, but it does not immediately answer:

- who this is for
- what urgent problem it solves
- what should happen next

#### Plan

Rewrite the hero to lead with the primary audience and the primary transformation.

Suggested direction:

- For researchers, conservation professionals, and biodiversity teams who want clearer public communication and more credible visibility.
- Start with the live product now, then explore the wider work only after the main offer is understood.

#### Repo changes

- Rewrite the homepage hero in [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- Add a short audience strip under the hero
- Keep the mission language, but move more of it lower on the page

#### Success metric

A stranger should be able to answer within seconds:

- "Who is this for?"
- "What can I get here right now?"
- "What should I click first?"

### Gap 2. Shift copy from ecosystem-first to visitor-first

#### Problem

The site has strong mission language, but the checklist is right that some key copy still sounds more like a project map than a conversion path.

For example:

- OPLURIX
- ERIS
- ecological intelligence
- mission visibility

These are useful trust builders, but they should follow the visitor's problem, not lead it.

#### Plan

Rework copy blocks so the sequence becomes:

1. name the visitor
2. name the friction they feel
3. show the result they want
4. explain how this page helps
5. then introduce the broader project story

#### Repo changes

- Rewrite hero and product-intro paragraphs in [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- Strengthen "before/after" language on [docs/products/01-expert-to-influencer-content-engine.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/01-expert-to-influencer-content-engine.html)
- Make [docs/services-en.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/services-en.html) and [docs/services-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/services-fr.html) more problem-first

#### Example copy direction

Current style:

- "field-grounded products, ecological intelligence projects"

Stronger visitor-first style:

- "Turn field notes, project experience, and biodiversity work into clear public communication people trust."

### Gap 3. Reduce navigation friction

#### Problem

The homepage currently asks first-time visitors to choose among too many destinations. The checklist's warning applies directly here: too many options create hesitation.

#### Plan

Reduce top-level navigation to the minimum needed for first-time conversion.

Suggested homepage nav:

- Home
- Product
- Proof
- Research
- Contact

Move these out of primary nav:

- `project-docs.html`
- possibly `field-notes.html`
- possibly `projects.html` as a secondary footer or in-page route instead of main-header prominence

#### Repo changes

- Simplify the top nav in [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- Keep deep links in the footer
- Add stronger in-page section links so the homepage itself handles more of the first journey

#### Additional structural recommendation

Use traffic-specific paths:

- Facebook and LinkedIn traffic -> homepage or ATBC page
- service inquiries -> services page
- warm collaborators -> projects page
- technical readers -> project docs

That way, the homepage does not need to do all jobs equally.

### Gap 4. Put proof closer to decisions

#### Problem

The site is not proof-poor. It is proof-scattered.

You already have:

- founder credentials
- publication references
- case-study content
- ATBC conference acceptance
- services rationale

But much of that proof lives one click away from the decision point.

#### Plan

Move the strongest proof elements closer to the first CTA and the product block.

#### Repo changes

- Add a compact "Why trust this?" band directly below the homepage hero in [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- Add one short before/after or outcome-style story near the product CTA
- Add one tighter proof module near the service CTA on [docs/services-en.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/services-en.html)
- Expand [docs/case-study.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/case-study.html) into a more explicit outcome narrative

#### Best proof formats to add next

- one quantified before/after example
- one "what changed" story from a dataset cleanup or communication workflow
- one "accepted at ATBC 2026" authority ribbon on relevant pages
- one founder-proof strip reused across product and services pages

### Gap 5. Make the CTA system clearer

#### Problem

The site has valid actions, but too many of them are equally loud:

- buy the live product
- explore projects
- join updates
- read the campaign
- support the trip
- contact for services

That is good ecosystem coverage, but weak CTA hierarchy.

#### Plan

Assign one primary CTA and one secondary CTA to each major page.

Suggested CTA hierarchy:

- Homepage primary: Buy the live product
- Homepage secondary: Join updates
- Services page primary: Request a sample review
- Services page secondary: See proof
- ATBC page primary: Support the research
- ATBC page secondary: Buy the live resource
- Field notes primary: Join updates
- Field notes secondary: Read proof page

#### Repo changes

- Reduce hero CTA count on [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- Make button language more specific
- Add repeated homepage CTA at:
  - hero
  - after proof
  - near the bottom
- Add friction-removal copy under the main CTA:
  - secure checkout
  - price
  - what happens after payment
  - expected response time for non-buyers

## Quick Wins We Can Implement First

### Quick win 1. Rewrite the homepage hero

Replace the broad current hero with a sharper audience-and-result message.

### Quick win 2. Cut the homepage nav

Remove at least one or two top-level destinations from the main header.

### Quick win 3. Add a proof strip under the hero

Use:

- Master's degree
- Flowers et al. (2023)
- ATBC 2026 acceptance
- Madagascar field experience

### Quick win 4. Add one lead magnet or free-entry asset

The checklist is right that not-ready visitors should not disappear.

Possible OPLURIX lead magnets:

- "5 mistakes conservation professionals make when turning field work into public content"
- "Biodiversity dataset cleanup readiness checklist"
- "ATBC 2026 drone survey one-page summary"

This should connect to the existing Netlify form flow.

### Quick win 5. Make CTA language more concrete

Examples:

- "Buy the Content Engine"
- "Request a sample review"
- "Support the ATBC 2026 trip"
- "Get future launch notes"

These outperform generic verbs like "Explore" when used at decision points.

## Proposed Execution Roadmap

### Phase 1: Conversion cleanup

Priority: Highest

- Rewrite homepage hero and supporting copy
- Simplify nav
- Reduce CTA clutter
- Add proof strip near hero
- Repeat the main CTA in three places

Files:

- [docs/index.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/index.html)
- [docs/assets/oplurix-store.css](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/assets/oplurix-store.css)

### Phase 2: Proof and offer strengthening

Priority: High

- Improve the live-product page with stronger before/after language
- Upgrade the case study into a more outcome-led proof page
- Tighten services pages around specific pains and outcomes

Files:

- [docs/products/01-expert-to-influencer-content-engine.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/products/01-expert-to-influencer-content-engine.html)
- [docs/case-study.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/case-study.html)
- [docs/services-en.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/services-en.html)
- [docs/services-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/services-fr.html)

### Phase 3: Lead capture and authority engine

Priority: Medium

- Create a lead magnet page and thank-you flow
- Turn field notes into a recurring content lane
- Add one reusable authority block across homepage, services, and campaign pages

Files:

- [docs/field-notes.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/field-notes.html)
- [docs/thank-you.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/thank-you.html)
- new lead-magnet page in `docs/`

### Phase 4: Traffic-specific landing pages

Priority: Medium

- Create one landing page for product traffic
- Create one landing page for services traffic
- Create one landing page for research-support traffic

This will let you post or advertise more precisely without forcing the homepage to do every job at once.

## Recommended North-Star Metrics

Even before full analytics maturity, we should track:

- click-through rate on the homepage primary CTA
- update-form submissions
- service-form submissions
- traffic to the ATBC campaign page
- clicks to Payhip checkout

## Recommended First Build Sprint

If we implement this in the repo, the best first sprint is:

1. rewrite the homepage hero and CTA hierarchy
2. simplify the header navigation
3. add a founder-proof strip under the hero
4. tighten the live-offer section around one pain-to-outcome arc
5. create one lightweight lead magnet for update capture

## Bottom line

The site does not need more ambition. It needs more sequencing.

You already have the substance that many sites lack:

- real work
- real proof
- real mission
- real public pages

The improvement opportunity is to make the first visit feel simpler, clearer, and more decisive.

That is how the repo-based site can become not just credible, but converting.
