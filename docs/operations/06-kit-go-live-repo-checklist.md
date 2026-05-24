# Kit Go-Live Repo Checklist

Purpose: the exact repo-side changes to make on the day Kit becomes active.

## Before touching the repo

Confirm all three are true:

1. the Kit forms exist
2. the Kit tags/custom fields exist
3. the first checklist delivery email is written

If not, stop here.

## Repo changes to make

### 1. Checklist pages

Files:

- [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html)
- [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist-fr.html)

Change:

- replace “records your interest cleanly” wording with “sent by email” wording
- if you use Kit embed, swap form block for Kit embed
- if you use a manual-style HTML action that posts to Kit, verify field names

### 2. Thank-you page

File:

- [thank-you.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/thank-you.html)

Change:

- checklist state should say the printable asset has also been emailed
- keep the printable checklist open/download path as a secondary fallback

### 3. Remove noindex only if the funnel is truly live

Files:

- [ethics-checklist.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist.html)
- [ethics-checklist-fr.html](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/ethics-checklist-fr.html)

Change only if:

- forms work
- email delivery works
- welcome sequence works

### 4. Campaign links

File:

- [campaign-link-map.md](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/campaign-link-map.md)

Check:

- campaign links still go to the checklist landing pages
- printable checklist links are not used for cold traffic unless intentional

### 5. Thank-you logic

File:

- [storefront.js](/C:/Users/Student/Desktop/perso/founderAiCheap/docs/assets/storefront.js)

Check:

- checklist thank-you state still points to the printable asset
- wording no longer implies “email not connected yet”

## Post-change verification

1. Submit EN checklist form
2. Confirm Kit tag applied
3. Confirm email received
4. Confirm thank-you page still makes sense
5. Repeat for FR checklist form

## Done means

- forms and repo copy tell the same truth
- cold traffic still works
- email delivery is real
- no page still talks like Kit is “coming later”
