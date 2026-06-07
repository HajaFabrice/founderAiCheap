# Errors Log

Use this as an append-only incident and debugging log for the OPLURIX/FounderAI platform.

Do not delete old entries. Add new entries at the top of the log so recent issues are easy to inspect.

## How To Use

Create a new entry when:

- a form submission fails
- a checkout link breaks
- Payhip, PayPal, Shopify, Netlify, Kit, or hosting behaves unexpectedly
- a product delivery fails or is delayed
- a FounderAI run exits with failure
- a provider is unreachable or out of credits
- a public page has a serious broken link, layout issue, or wrong launch-state claim
- a campaign link loses tracking parameters
- an approval-sensitive action is accidentally exposed or unclear

## Severity

| Severity | Meaning |
| --- | --- |
| `S0` | Revenue, trust, data, or security emergency. Fix immediately. |
| `S1` | Major user-facing breakage or blocked operational path. Fix before new work. |
| `S2` | Important bug with workaround. Schedule soon. |
| `S3` | Minor issue, polish, typo, or improvement. Batch later. |

## Status

Use one of:

- `open`
- `investigating`
- `blocked`
- `fixed_pending_verification`
- `verified`
- `wont_fix`

## Entry Template

Copy this block for each new issue.

```md
## YYYY-MM-DD HH:MM - Short issue title

- Severity:
- Status:
- Surface:
- Environment:
- Detected by:
- Owner:

### Symptom

What happened?

### Expected Behavior

What should have happened?

### Actual Behavior

What happened instead?

### Steps To Reproduce

1. 
2. 
3. 

### Suspected Cause

Current best explanation.

### Files, Links, Or Run Artifacts

- 

### Fix

What changed?

### Verification

How was it tested?

### Follow-Up

What should be improved to prevent this class of issue?
```

## Log

No entries yet.

