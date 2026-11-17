# Next round: session priority independent of grouping

Status: deferred by user on 2026-11-18. Product implementation has NOT started.
Baseline: main 745f334db237cd02854a0415aab7b11cd5f45b8a.
Latest instruction: pause this feature and save it for the next round; investigate ACP import main CI failure first.

## Confirmed behavior

- One priority order for every session list and every grouping, including inside the pinned section: Blocked > Errored > Done > Busy > Idle > Failed. Unknown/stale state must not be promoted as a live actionable state without evidence.
- Within equal priority, use most recent meaningful activity, then stable identity for deterministic ties.
- Pinned section remains first. Within it use exactly the same priority comparator.
- Ordinary groups compare their highest visible session priority, then most recent meaningful activity, then stable group identity. One actionable session suffices; do not weight by group size.
- Derive group ordering after filtering; rows already placed in pinned section do not also affect their former ordinary group.
- Meaningful activity: user submission, task/turn start, real status transition (approval/error/completion). Stream chunks, terminal redraw, heartbeat, polls and duplicate state delivery must not bump ordering.
- Recompute on priority decrease as well as increase; recently resumed Busy must not outrank another Blocked.
- Defer visible reordering during scroll, held pointer/touch, open row menus or keyboard row interaction; preserve scroll anchor, selected identity and collapsed-group state.
- Grouping chooses membership only: none / machine / category / directory / state. Priority comparator is shared backend policy, not copied per view.
- Parent/child rows remain a tree. Surface a child requiring attention through a derived parent priority/reason without falsifying the parent's own execution state.

## Validation when resumed

Backend: all grouping modes, pinned/nonpinned, tie determinism, priority up/down, filter effects, unknown/stale state and meaningful-event timestamps.
GUI/Lab: root, workspace and machine session hosts; group promotion/demotion; pinned internal order; children; menu/touch/keyboard/scroll stability. Verify actual source artifact remotely; local source-only restriction remains.
Do not start implementation or publish as part of the CI investigation.
