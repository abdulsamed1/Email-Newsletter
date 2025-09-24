# Documentation Guidelines

## Principles
- **Consistency**: every Phase and ADR follows the same template.
- **Traceability**: decisions are linked from README → docs/adr.
- **Separation of concerns**: README for overview, ADR for design decisions, phases for learning journey.

## Workflow
1. After each phase:
   - Write a `docs/phases/phase-N.md` note.
2. When facing a major design choice:
   - Add a new ADR in `docs/adr/`.
3. For releases/milestones:
   - Update `CHANGELOG.md`.

## File Naming
- `phase-XX-title.md` for phases.
- `000X-title.md` for ADRs.
