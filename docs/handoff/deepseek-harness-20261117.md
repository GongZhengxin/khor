# DeepSeek Harness integration

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

## Current progress

Implemented first candidate `28320506`, verification-runner fix `86220f3e`:

- Shared built-in `dsh --profile acp` launch default, explicit registration override and no synced default/credential writes.
- Direct GUI conversation choice, product name and official monochrome Harness whale shared by session marks.
- Advertised model labels for opaque configuration values; execution-machine credential guidance; no unsupported additional-directory input for official DSH.
- Complete App DSH Lab scene and entry/icon/model checks. Additional test/documentation work is currently uncommitted; inspect actual diff.

Local real DSH acceptance at `/Volumes/disk0/Khor/evidence/deepseek-harness-20261117/`:

- `control-probe.json`: actual ACP initialize/new/close, version 0.2.1-rc.5, successful clean exit.
- `real-probe.json`: one paid prompt on `deepseek-flash`, reasoning off, native file-tool read of a random token from the isolated workspace, exact expected reply, successful close/list/resume/close and clean exit. No existing user session was opened or modified. No API key was logged or copied.
- Images are actually advertised false; list/resume/close and HTTP MCP are advertised.
- 23 Python Lab lifecycle/configuration tests passed locally. No Node/Rust/Xcode build ran locally.

Verification handles (poll these exact runs; a timeout is not permission to restart):

- Public Quality `43947733635`, source `86220f3e`: agent and Windows process checks succeeded; trusted macOS DSH job was queued at last inspection. Initial earlier dispatch was rejected for runner-context placement and was corrected; it was not a product failure.
- Private Developer Lab `43947576487`, request `0c379a1787884c6289abbe64e99c30ab`, snapshot `1de332a1f499403f35a178d34b7e083fe5eab8dc`, source tree `068b3c3ed731940b1e9e9e50fa7a101268262803`: queued at last inspection. Display title has not yet expanded its request id; manager has not discovered the run. Preserve it and reconcile the same run before any resubmission.
- Owned Lab manager PID 67503 on port 23971; cache `/Volumes/disk0/Khor/caches/deepseek-harness-lab`, manager log under `5e0ad2f18336773e-23971/manager.log`. No bundle available yet; do not present it as ready.

Still required: exact-source Rust/frontend verification and rendered DSH screens; acquire trusted CLI/bridge artifacts; run isolated real Khor user-session and delegation flows against installed DSH, including cancellation and recovery; audit and finish the branch. The native workflow is being extended to include bridge and normal frontend artifacts for that acceptance. Do not mark the goal complete from the raw ACP probe alone.
