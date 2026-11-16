# DeepSeek Harness integration

## Latest checkpoint

Product runtime source is `ab6b7e434`; the subsequent checkpoint changes only
this handoff.
The real user and Khor-parent-to-DSH paths have passed against trusted runtime
source `09ce10a98e77a745a1f5b078e95b7805745c6c1a`. That acceptance exposed a
read-only recovery-offer omission, fixed in ab6b7e434. Goal remains active until
that exact fix passes native regression and a no-model-call live recheck.

- Native workflow `43960937477`, source ab6b7e434, is the only current run.
  It verifies both doctor and read-only recovery regressions red then green,
  builds the exact CLI/bridge, and runs discovery, sync/GUI agents, far-open,
  ACP lifecycle and real Khor runtime delegation tests. Poll the same run;
  do not restart it on a waiting timeout.
- `43953514368`, source 09ce10a9: frontend and exact binary build passed;
  discovery/override, green doctor, 11 sync-agent and six GUI-core tests passed.
  The final job failed on one test-only formatting difference and an old
  far-open test using a terminal connector against a GUI host. Both are fixed
  in `425d79501`. ACP/delegation regressions were skipped in that run.
- `43959393857`, source 425d79501, was cancelled before native execution once
  the real acceptance found the recovery-offer omission, avoiding an obsolete
  native build. Early `43947733635` passed agent/Windows checks but lacked
  runner rustfmt/frontend inputs; those workflow inputs were fixed.
- Focused GUI Quality `43955048330`, source `9d086a5e`: **success**. All 20
  complete-App scenarios passed: Chromium/WebKit × four widths (1280, 412,
  390, 320) × light/dark, plus four setup flows. Focused creation/shared-mark/
  configuration, type/unit/mobile and hot-update checks passed too. Normal
  production GUI files have not changed since source 09ce10a9.
- `lab-final/developer-lab` was verified against 9d086a5e and its Git tree
  `699dacda1c5788b0ef12dbeefa07773303e03d67`. The 320px WebKit dark and 1280px
  Chromium light screenshots and the 390px WebKit setup flow were visually
  inspected. The official mark and
  readable Pro model label are present. The successful bundle is served at
  `http://127.0.0.1:23971/app/?scene=dsh` by owned exec session 70277.
- Previous full GUI run `43949486204` passed frontend/GUI gates and 79/83 Lab
  cases. Four DSH assertions observed browser network traffic instead of the
  existing in-page fixture boundary; the successful focused run replaces them
  and verifies actual theme state. Old private Lab `43947576487` is superseded;
  its owned manager PID 67503 exited. Existing user Lab 23961 was never changed.

Evidence is under `/Volumes/disk0/Khor/evidence/deepseek-harness-20261117`.
The trusted `native-09` artifact has matching source SHA/tree and verified
SHA-256 hashes for both binaries. It contains the controlled doctor red proof.
`icon-provenance.json` proves the vendored SVG geometry matches official commit
`39163dd30f703a79079e2d2446545cff3c453738`.

## Real acceptance

`runtime-acceptance-baseline09.json` records these successful checks:

- Default discovery without registration, execution-home capability setup,
  opaque model/reasoning selection, and actual file upload to that home.
- A real DSH file-tool read returned the unpredictable token. Both the tool
  output and agent message appear in canonical Khor history (21 entries).
  Reattaching retained that history.
- A second real DSH prompt was interrupted and returned `Cancelled`.
- Native `close` control succeeded; explicit Khor recovery resumed the same
  native session and preserved the canonical history. This works on source 09,
  but its read-only offer incorrectly projects support `unknown` because that
  lookup omitted shipped defaults. ab6b7e434 adds the same default/override
  precedence to the pure snapshot read; the new regression checks parity with
  actual recovery resolution, including an explicit override.
- The real embedded Khor agent runtime called `agents`, `delegate`, and
  `task_wait` twice. A real DSH child completed a separate file-read task; the
  parent received the actual random token, and the child receipt binds to the
  parent run. One parent permission was approved. The parent planner was a
  deterministic localhost model with a synthetic credential, not a paid model;
  the Khor runtime, MCP transport, permission path and DSH child were actual.

Four bounded real DSH prompt rounds were used in total: the initial raw probe,
user attachment-read, cancellation probe, and parent delegate. A prompt can
involve multiple provider requests. No cost amount is inferred. Intermediate
acceptance-driver fixes did not repeat completed paid user phases. One direct
embedded-resource request was rejected by DSH before a model call; it is
retained as negative evidence, while the real GUI uses uploaded `@path` text.
The driver also learned that row Close deliberately forgets the row and that
abrupt host termination cannot manufacture recovery authority. Final acceptance
uses the actual capability-gated native close control and normal explicit
recovery; it does not edit session metadata.

The Python drivers use an isolated Khor home/workspace and loopback bridge,
retain original DSH credentials in their existing store, and close only their
own sessions/processes. Baseline cleanup left zero owned live session metadata;
the bridge exited by the driver's intentional SIGTERM. No production Khor
store or existing user session was changed. `runtime-acceptance.py
--continue-session` reuses recorded user history and skips completed paid phases;
when the new trusted artifact is available, require recovery support `resume`
and no `known_baseline_issue` in the resulting final report.

Structured self-review used review-changes and both indexes. The worktree graph
is partial/stale, so zero graph flows is not a no-impact claim. Direct review
and actual acceptance found the doctor and read-only recovery omissions; both
have behavioral regressions. The latter is included by the permanent full-node
`dsh_` test selection. No independent reviewer was delegated, in accordance
with the serial-execution constraint.

## Execution state

- Owner: root, serial execution; no delegated agents.
- Worktree: `/Volumes/disk0/Khor/worktrees/current/khor-deepseek-harness-20261117`.
- Branch: `feat/deepseek-harness-20261117`; base: `81cc76c48b48b2900f37c70e0307a47df29dcaa1`.
- User authorized complete DSH integration for user sessions and Khor agent, an independent worktree, the official DeepSeek Harness session icon, and reasonable real calls to installed DSH.
- Installed DSH: `/opt/homebrew/bin/dsh`, version `0.2.1-rc.5`.
- No merge, release, installed Khor replacement, or existing session restart is part of this task.
- Local source-only workflow remains in effect; the specifically authorized installed DSH runtime may run locally. Builds and substantial frontend/Rust verification use the established remote workflow.

## Required outcome and evidence

1. Users can select DeepSeek Harness directly, create a GUI conversation on its execution machine, use advertised model/configuration, tools, permission decisions and cancellation, and continue recorded sessions.
2. Khor agent can discover, launch and drive DSH through its normal delegation tools, with observable results and the same session identity.
3. Official Harness artwork is used by the shared session type judgment in creation, lists, tabs and mobile navigation; it must not fall back to a generic agent or the DeepSeek model-provider mark.
4. DSH setup and errors respect execution-machine ownership and do not replicate credentials. Unsupported upstream capabilities are represented honestly.
5. Appropriate existing checks, actual rendered UI inspection, isolated backend flows, and bounded real installed DSH calls prove the integration. Source version and limits accompany every result.

## Plan

1. Inspect actual installed ACP capabilities and profile/credential behavior without exposing secrets; inspect registry, user-session and delegation routes.
2. Add the supported DSH launch identity and official brand mark; implement any missing configuration or routing required by both entry points.
3. Verify the complete user and agent flows remotely and through bounded local DSH execution; fix failures and record evidence.
4. Audit the final diff and each required outcome, commit the independent branch and provide reviewable results.

## Implementation and completed local baseline

- Shared default `dsh --profile acp`, explicit registration override, and no
  synced default/credential writes.
- Direct GUI choice, readable product/model names and official monochrome
  Harness whale in shared session marks.
- Execution-computer credential guidance and no unsupported extra-directory
  input for official DSH. Existing ACP configuration, tool, permission,
  cancellation, attachment and history/recovery paths are reused.
- Complete App DSH Lab scene and permanent discovery/doctor/UI checks.
- `control-probe.json`: real installed ACP initialize/new/close, clean exit.
- `real-probe.json`: one real `deepseek-flash` prompt with reasoning off read
  a random token through the native file tool, returned the exact token, and
  passed close/list/resume/close with a clean exit. No existing session touched.
- Installed DSH advertises images false and listing/resume/close/HTTP MCP true.
  It does not replay provider transcripts; Khor keeps its own event history.
- 23 Python Lab configuration/lifecycle tests passed locally. No local Node,
  Rust or Xcode build was run.

Still required: finish native run 43960937477, acquire its trusted artifact,
recheck the fixed read-only offer and native continuation without new model
prompts, record exact-source results, and complete the branch audit.
