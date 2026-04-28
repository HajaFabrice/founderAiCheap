# Independent Business Boundary

Updated: 2026-04-27

Purpose: keep FounderAI useful for business development without letting it
cross employer boundaries or confuse Techni-Drones promotion with the
founder's separate freelance pipeline.

## Canonical Boundary

- Techni-Drones Madagascar is the founder's employer and credibility platform,
  not the founder's owned client base.
- FounderAI may support Techni-Drones work internally:
  - internal notes
  - briefing memos
  - meeting preparation
  - strategy drafts
  - review-ready public-promotion drafts that do not create an unapproved
    client-facing CTA
- FounderAI may not draft outbound client communication on behalf of
  Techni-Drones clients or prospects.
- FounderAI may support independent freelance work only for clearly non-Techni
  leads.

## Lead Classes

### Employer Lead

Definition:
- Techni-Drones client
- Techni-Drones prospect
- Techni-Drones referral owned by the employer
- public promotion written in the employee role

Allowed FounderAI support:
- internal talking points
- internal strategy note
- founder prep memo
- public-promotion draft without a personal-service CTA

Not allowed:
- send-ready client outreach
- send-ready proposal or reply on behalf of Techni-Drones
- moving the lead into the independent pipeline

### Independent Lead

Definition:
- clearly outside Techni-Drones ownership
- sourced independently
- no employer client-data dependency
- no conflict with current employer scope

Allowed FounderAI support:
- outreach drafts
- follow-up drafts
- scope notes
- sample-review replies
- proposal and delivery templates

### Ambiguous Lead

Definition:
- shared network contact
- warm introduction from work
- unclear ownership
- unclear permission to pursue independently

Default action:
- stop
- return `NEEDS_HUMAN_CLARIFICATION`
- prepare only an internal decision memo if useful

## Separation Rules

- Do not mix Techni-Drones promotion with a personal-service CTA in one
  message.
- Do not use Techni-Drones client data as an independent prospecting database.
- Do not imply that FounderAI speaks for Techni-Drones externally.
- Do not pitch ERIS inside employer promotion unless explicitly authorized and
  clearly separated.
- If written referral or overflow permission exists later, store that document
  as a source before treating those leads as independent.

## Operational Defaults For Agents

- Anthony and Bonaventure:
  - external drafting only for independent leads
  - internal memos only for employer or ambiguous leads
- Zacchaeus:
  - if ownership is unclear, draft a holding note or internal summary instead
    of a send-ready reply
- Perpetua:
  - nurture only independent leads
  - pause immediately for employer or ambiguous ownership
- Hildegard and Francis:
  - may review the independent pipeline and the employer-boundary discipline as
    internal operations work

## Source Basis

- User instruction dated 2026-04-27 establishing the dual-track model
- `documents/Freelance/30-Day NGO Outreach & Contract Plan - Grok`
- `documents/Freelance/Client Outreach tracker 1.xlsx`
- `documents/Freelance/client outreach tracker 2.xlsx`
