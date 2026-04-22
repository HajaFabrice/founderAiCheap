# Workflows

## Universal Run Workflow

1. Load founder identity, founder brain, governance, and the relevant agent prompt.
2. Classify the work as briefing, outreach, proposal, grant, research, scheduler, or review.
3. Route it to one owner with bounded scope.
4. Draft first, then pause behind approval if external consequences exist.
5. Preserve inspectable artifacts in run folders and team or grant outputs.

## Outreach Workflow

1. Target one real organization or person.
2. Reference a concrete project, paper, report, or field need.
3. Lead with one useful low-friction offer, usually a free demo flight or bounded sample.
4. Keep the CTA small and the tone sober.
5. Do not price in cold outreach.

## Grant Workflow

1. Pio raises the deadline or the founder drops a grant task into `inbox/`.
2. Bartholomew drafts one bounded grant asset: narrative, budget note, attachment checklist, or short cover email.
3. Clare checks for governance, realism, and the M350-not-M400 rule.
4. Hildegard surfaces the item in the briefing and queue.
5. Founder approves or rejects. Submission remains manual unless a future approved send path exists.

## Deadline Workflow

1. Pio tracks dated and rolling deadlines through `config/pio_deadlines.json`.
2. On alert windows, Pio creates inspectable inbox requests rather than silent reminders.
3. Deadline requests route toward the assigned agent or founder review.
4. Processed alerts remain recorded in state so the same alert is not duplicated.

## Team Orchestration Workflow

1. Start from the six roles, not abstract teams.
2. Queue one bounded daily packet per role.
3. Keep outreach and production separate unless a packet is trivially safe and small.
4. Let overlay agents support the workflow without replacing the six-lane structure.

## Cloud Migration Workflow

1. Keep the local daemon healthy.
2. Add Gmail polling before replacing inbox assumptions.
3. Dockerize the daemon.
4. Add cloud persistence only after the runtime remains auditable and approval-safe.
5. Keep any UI or webhook layer thin and reversible.
