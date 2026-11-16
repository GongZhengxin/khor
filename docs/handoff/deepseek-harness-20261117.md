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

Independent worktree created. Design-system entry and icon/behavior rules read. Repository already supports ACP v1 resume, generic registered agents and shared session marks. No product changes yet; installed DSH protocol and delegation discovery are being investigated.
