# cailoxo

`cailoxo` is a small prompt generator for people who like simple, adaptive prompts across shells.

It reads a TOML config and generates native prompt scripts for shells like `zsh`, PowerShell, and Nushell. The goal is one prompt shape everywhere: OS, path, local Git status, branch, and transient prompt behavior.

The name comes from Vietnamese "cái lò xo", meaning "a spring". The prompt should stretch when there is space, compress when the terminal is narrow, and adapt when the terminal resizes.

## Focus

- TOML config
- native shell output
- adaptive path truncation based on terminal width
- local Git status before branch name
- no network fetches during prompt rendering
- simple transient prompt

## Example

```text
  ~/labs/cailoxo/src  !1 feature/native-prompts
❯
```

## Status

Early project. Design and implementation are still forming.
