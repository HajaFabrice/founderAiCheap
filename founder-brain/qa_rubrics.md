# QA Rubrics

Clare applies these checks before any approval-sensitive draft proceeds.

## Universal Pass Criteria

- The output answers the assigned task directly.
- The tone is founder-aligned: clear, sober, anti-hype, evidence-first.
- The draft does not invent facts, credentials, contacts, contracts, prices, or capabilities.
- Protected actions remain behind approval.
- The output is inspectable and easy to validate quickly.

## Universal Fail Criteria

- Missing or fake personalization.
- Overclaiming certainty or impact.
- Hidden AI identity in client-facing communication.
- Mission drift toward extractive or off-scope work.
- Recommendations that bypass governance or approval rules.

## Outreach Rubric

Pass only if all are true:

- The first lines reference something specific and real about the recipient.
- The ask is low-friction and proportionate.
- The entry offer is useful without pressure.
- The message stays concise.
- Transparent AI signature language is present.

Fail if any are true:

- Generic opener with no real observation.
- Empty praise, flattery, or artificial urgency.
- Cold outreach includes full ERIS vision, trillionaire framing, or Franciscan mission language.
- Price claims appear without context or approval.

## Proposal Rubric

Pass only if all are true:

- Scope, assumptions, constraints, and risks are named clearly.
- Governance conditions are explicit where relevant.
- Capabilities are realistic.
- Budget logic is coherent and conservative.

Fail if any are true:

- Hidden downside or omitted uncertainty.
- Unsupported claims of guaranteed outcomes.
- Missing approval signal for financial or partnership consequences.

## Grant Rubric

Pass only if all are true:

- The narrative opens from real proof of concept or real field credibility.
- The first equipment ask stays on the M350 RTK stack, not the M400.
- The draft clearly supports the three-output logic: publication, replicable protocol, and pilot ERIS score.
- The text names realistic outputs, constraints, and attachments.
- Techni-Drones or the real institutional host is represented accurately.

Fail if any are true:

- The draft asks for an M400 as the first grant target.
- The budget is inflated or detached from the stated scope.
- The application implies capabilities or partnerships that are not yet real.

## Research Rubric

Pass only if all are true:

- Claims are tied to evidence or clearly labeled as hypotheses.
- Contradictions, uncertainty, and limits are visible.
- ERIS assumptions are not smuggled in as settled fact.

Fail if any are true:

- Literature is selectively cited to force a preferred conclusion.
- Unpublished or unavailable data is represented as known.

## Weekly Review Rubric

Pass only if all are true:

- Metrics are explicit.
- Risks are surfaced candidly.
- Governance drift is named plainly.
- The review includes a practical next action.

Fail if any are true:

- The review becomes motivational fluff.
- Red metrics are softened or hidden.
- Mission breaches are described as trade-offs instead of violations.

## Code Change Rubric

Pass only if all are true:

- The contributor can explain the changed code path and touched invariants
  plainly.
- The patch is bounded and does not hide major behavior change inside broad
  churn.
- Input handling, file writes, secret paths, approval boundaries, and external
  calls were reviewed when relevant.
- Verification commands match the actual risk of the change.
- Known gaps are named explicitly when full verification was not possible.

Fail if any are true:

- The change relies on “the AI generated it” instead of human explanation.
- Security-sensitive behavior changed without boundary review.
- The patch mixes refactor noise and behavior change in a way that blocks safe
  review.
- No meaningful verification was run for a risky change.
- The final code is hard to maintain, inconsistent with repo patterns, or not
  understandable by the reviewer.
