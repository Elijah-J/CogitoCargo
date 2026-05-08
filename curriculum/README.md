# EduRatchet-2 Rust Moves Run

This run teaches Rust through a dependency graph of action-centered
lessons. Each cycle outputs one Rust move: a small action the learner can
recognize, type, run, and contrast with one nearby alternative.

## Audience

The learner is intelligent and computer-literate, but is learning
programming and Rust together. Assume terminal use and file navigation.
Do not assume programming vocabulary unless a prior lesson installed it.

## Priorities

1. Dependency correctness.
2. Grounding and verifiability.
3. Mental-model delta.
4. Runnable contrast.
5. Prose clarity.

The page should be clear, not professor-polished. It should read like a
useful lesson note, not a cumulative audit brief.

## Grounding Standard

Every substantive statement must map to one of:

- a listed Rust corpus source;
- a captured local probe;
- a prior accepted EduRatchet-2 lesson;
- or an explicit ordinary computer-use assumption.

Inline citations are optional. New lessons should keep only a short
evidence pointer in the learner-facing file; the full mapping belongs in
`evidence/NNN-short-slug.md`.

## Lesson Naming

Lessons live in `lessons/` and use:

`NNN-short-slug.md`

Observations live in `observations/` and use:

`NNN-short-slug.rs`

Evidence appendices live in `evidence/` and use:

`NNN-short-slug.md`

## Artifact Roles

- `graph.md` is the canonical dependency graph. It owns each node's move,
  main concept, dependencies, unlocks, evidence paths, and status.
- `lessons/` are learner-facing notes. They should not duplicate the graph
  fields beyond minimal frontmatter such as `id`, `status`, and `evidence`.
- `evidence/` is the audit appendix: source quotes, probe transcripts,
  toolchain details, and exact claim-to-evidence mapping.
- `observations/` holds minimal Rust probe files.

Accepted lessons before this split may still contain older inline
`## Evidence` sections. Do not rewrite them solely to match the new shape.

## Lesson Acceptance Standard

A lesson is acceptable only if:

- it teaches one Rust move;
- it installs one main concept;
- its dependencies are already installed or explicitly named as ordinary
  computer-use assumptions;
- it has a clear mental-model delta;
- it has a runnable example, contrast, or prediction when feasible;
- its substantive claims are source/probe/prior-lesson grounded;
- its `graph.md` node agrees with the lesson and evidence appendix;
- its learner-facing prose stays lesson-shaped rather than audit-heavy.

## Audit Trail Depth

- Current lesson: aim for 600-900 learner-facing words before the short
  evidence pointer; exceed 1200 only for a clear reason.
- Current evidence appendix: include the full grounding and observation
  detail needed for audit.
- Direct prerequisites: summarize the exact prerequisite claim in 1-3
  bullets in the appendix.
- Older supporting lessons: mention by lesson number/title only unless a
  specific claim is load-bearing and not restated in the direct
  prerequisite.
- Contrastive claims: when the move says "with X this works, without X it
  fails/differs," include a negative/contrast probe or state why one is not
  needed.
- Red-team may inspect as deeply as needed. The learner-facing lesson
  should not replay the whole chain.
- Do not maintain a prose `## Frontier` section in `graph.md`; choose from
  accepted node unlocks and the current graph instead.

## First Moves

Start from scratch. Natural early moves are toolset and program-shape
moves before abstract programming vocabulary:

- create a tiny Rust source file;
- compile it with `rustc`;
- run the produced executable;
- recognize `fn main()` as the start marker for these tiny programs;
- read a simple compiler diagnostic;
- give a value a name with `let`.

The orchestrator may choose a different first move if the dependency
argument is stronger.
