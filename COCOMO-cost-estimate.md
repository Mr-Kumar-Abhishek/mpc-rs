# COCOMO cost estimate for mpc-rs

This document provides a rough effort and schedule estimate for the current state of the mpc-rs project using the Basic COCOMO model for all three project types.

## Assumptions

- Estimated size: about 0.78 KLOC
- Source basis: the repository currently contains roughly 775 lines of code across the library and example files
- Model used: Basic COCOMO
- Unit: person-months (PM)
- Estimated price assumption: $5,000 per person-month for a small Rust software project

## COCOMO formulas

- Effort = $a \times (KLOC)^b$ person-months
- Development time = $c \times (Effort)^d$ months
- Average team size = $Effort / Time$

## Coefficients

| Model | $a$ | $b$ | $c$ | $d$ |
|---|---:|---:|---:|---:|
| Organic | 2.4 | 1.05 | 2.5 | 0.38 |
| Semi-detached | 3.0 | 1.12 | 2.5 | 0.35 |
| Embedded | 3.6 | 1.20 | 2.5 | 0.32 |

## Estimated results

| Model | Effort (PM) | Duration (months) | Average team size | Estimated price |
|---|---:|---:|---:|---:|
| Organic | 1.84 | 3.14 | 0.58 | $9,200 |
| Semi-detached | 2.27 | 3.33 | 0.68 | $11,350 |
| Embedded | 2.68 | 3.43 | 0.78 | $13,400 |

## Notes

- These values are indicative and intended for planning rather than precise budgeting.
- Because mpc-rs is a small Rust library, the project would likely be delivered by a very small team or even a single contributor.
- The estimate can be refined if the actual total delivered size is updated.
