# ERIS Metadata And Governance

Updated: 2026-04-27

Purpose: preserve the audit and traceability discipline from the planning
appendices in an agent-usable plain-text form. This is aligned with the V4
master plan and supports grant, research, QA, and methodology drafting.

## Core Principle

Metadata are not auxiliary notes. They are part of the product. Every serious
ERIS-style output should be traceable, reproducible, and auditable.

## Dataset-Level Metadata

Each dataset should carry, at minimum:

- dataset ID
- data type
- data owner or provider
- collection method
- sensor or platform
- spatial coverage
- temporal coverage
- spatial resolution
- temporal resolution
- coordinate reference system
- file format
- quality flags
- access and use constraints
- pilot or version link

## Indicator-Level Metadata

Each indicator should carry:

- indicator ID
- standardized indicator name
- associated pillar
- input dataset IDs
- processing steps
- normalization method
- weighting class
- spatial unit
- temporal scope
- sensitivity notes
- update frequency

## Spatial Output Metadata

Maps and spatial products should carry:

- map ID
- represented indicators or pillars
- output spatial resolution
- classification or threshold scheme
- confidence layer description
- generation date
- linked version reference
- intended use

## Confidence And Uncertainty Metadata

Confidence is a first-class output, not a side comment. Preserve:

- data completeness score
- cross-source consistency signal
- bias sensitivity flag
- confidence class
- short note on what is driving confidence or uncertainty

## Versioning And Lineage

Version these layers:

- raw datasets
- processed datasets
- indicator calculations
- pillar scores
- composite score outputs

This is what allows reconstruction of a reported score later.

## Governance Principles

- Traceability: every output links back to inputs and parameters.
- Transparency: assumptions are documented, not implied.
- Minimum necessary disclosure: sensitive information stays protected while the
  workflow remains auditable.
- Reproducibility: an independent reviewer should be able to re-run the logic if
  granted access.
- Iterative improvement: metadata evolve with the method, not after it.

## Audit Walkthrough

When a reviewer asks how a score was produced, walk through it in this order:

1. identify the reported output, version ID, generation date, and confidence class
2. retrieve the pillar-level scores
3. inspect contributing indicators and their weights
4. trace indicators back to input dataset IDs and processing steps
5. confirm normalization and aggregation settings
6. review confidence and missing-data handling
7. if needed, reproduce the output from raw inputs and versioned parameters

## Safe Communication Rule

When describing ERIS methodology externally, describe it as transparent,
versioned, and confidence-tagged. Do not imply regulatory certification or
cross-site validation unless the prompt packet explicitly verifies that claim.
