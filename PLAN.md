# Plan

`cailoxo` MVP is complete. The project now generates native adaptive prompt scripts from TOML for `zsh`, Nushell, and PowerShell with no prompt framework required at runtime.

Archived MVP scope and completion notes live in `plans/MVP.md`.

## Current State

- Version: `0.1.2`.
- Shell targets: `zsh`, `nu`, `pwsh`.
- Runtime model: generated native shell scripts.
- Prompt shape: two-line left prompt with OS, adaptive path, Git, and prompt character.
- Config: TOML, schema version `1`.
- Release: tag-driven GitHub release and crates.io publish.

## Completed MVP

- Rust CLI: `cailoxo generate --shell <zsh|nu|pwsh>`.
- Native generators for `zsh`, `nu`, and `pwsh`.
- Adaptive path truncation with `edge_format` and `gitdir_format`.
- Local Git branch/status/ahead/behind/stash data.
- Optional async `git fetch` outside prompt rendering.
- Built-in OS, Git branch, Git upstream, and Git status icon dictionaries.
- OMP-style decoration tags in templates and path format strings.
- Transient prompts for all target shells.
- zsh/pwsh OSC8 path and Git links; Nu documented as unsupported due Reedline prompt handling.
- GitHub Actions release pipeline for Linux/macOS/Windows assets and crates.io.

## Post-MVP Priorities

1. Stabilize config schema.
2. Add focused tests for generated shell behavior.
3. Improve docs and examples.
4. Add import/conversion helpers for existing prompt themes.
5. Expand prompt features only when native shell behavior is clear.

## Candidate Work

- Add config schema reference generated from Rust structs.
- Add shell snapshot tests for generated scripts.
- Add tests for Git remote URL normalization.
- Add icon override docs and examples.
- Add more bundled themes.
- Add right prompt support.
- Add optional console title support.
- Add config validation diagnostics with line/path context.
- Add `cailoxo doctor` for shell/profile/debug info.
- Add installation docs for package managers when distribution exists.

## Deferred

- Full oh-my-posh or Starship compatibility.
- Theme marketplace.
- Arbitrary custom executable spans.
- Shared cross-pane remote fetch cache/lock.
- Nu OSC8 prompt hyperlinks unless Reedline supports them reliably.

## Validation

Before commits and releases:

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked -- -D warnings
cargo package --locked --allow-dirty
cargo run -- generate --shell zsh
cargo run -- generate --shell nu
cargo run -- generate --shell pwsh
zsh -n output/prompt.zsh
nu --no-config-file --commands 'source output/prompt.nu; print (do $env.PROMPT_COMMAND); print (do $env.PROMPT_INDICATOR)'
pwsh -NoLogo -NoProfile -Command '. ./output/prompt.ps1; prompt | Out-String'
```
