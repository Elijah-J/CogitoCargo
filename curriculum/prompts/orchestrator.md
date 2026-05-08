# Orchestrator Nudge

Use this when starting or resuming an EduRatchet-2 cycle:

```text
You are the EduRatchet-2 orchestrator in `/Users/eli/.eduratchet-worktrees/eduratchet-2`.

Read `CLAUDE.md`, `experimental/eduratchet2/runs/rust-moves/README.md`,
and `experimental/eduratchet2/runs/rust-moves/graph.md`.

Start the next cycle. Choose one Rust move whose prerequisites are
already installed in the graph, or whose prerequisites are ordinary
computer-use assumptions appropriate for an intelligent beginner learning
programming and Rust together.

The output should be a bite-sized action-centered lesson, not a proof
packet and not a textbook chapter. The graph is the spine and the
canonical metadata source: the lesson should install one main concept and
update the graph.

Dispatch `eduratchet2-worker` to write one lesson under:

`experimental/eduratchet2/runs/rust-moves/lessons/`

one evidence appendix under:

`experimental/eduratchet2/runs/rust-moves/evidence/`

and any minimal probe under:

`experimental/eduratchet2/runs/rust-moves/observations/`

The lesson prose does not need inline citations for every sentence, but
every substantive statement must map in the evidence appendix to a
listed Rust source, captured probe, prior accepted lesson, or explicit
assumption.

After the worker returns, dispatch `eduratchet2-redteam`. Red-team must
audit dependency fit, grounding/verifiability, graph consistency, and
learner fit. If verdict is pass, mark the lesson and graph node as
`accepted`, then commit the lesson, evidence appendix, probe, and graph
update atomically.
If verdict is demand, send back only the concrete demands. If verdict is
fail, stop and summarize the blocker.

Do not add or maintain a prose `## Frontier` section in `graph.md`.

Do not use network fetching. Do not mutate the host worktree. Do not
inspect other active lesson workflows unless explicitly redirected.
Proceed without asking for confirmation unless blocked.
```
