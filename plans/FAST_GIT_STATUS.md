# Fast Git Status For Prompts

This document records two possible plans for making Git status fast in generated prompts.

## Plan A: Zsh Integration With romkatv/gitstatus

Use `romkatv/gitstatus` in the generated zsh prompt, similar to Powerlevel10k. The goal is to replace synchronous `git status`, `git rev-list`, `git stash list`, and `git config` calls in `__cailoxo_git_info` with one fast daemon query.

### Prerequisites

- Interactive zsh runtime.
- `git` available in `PATH`.
- zsh modules available: `zsh/system`, `zsh/datetime`, and `zsh/files`.
- `add-zsh-hook` available through `autoload -Uz add-zsh-hook`.
- `gitstatus.plugin.zsh`, `install`, `install.info`, and `build.info` available from a `gitstatus` checkout or package.
- `gitstatusd` binary available through `GITSTATUS_DAEMON`, local `usrbin`, or downloadable cache.

### Packaging Choice

Do not vendor `gitstatus` into `cailoxo` at first.

`gitstatus` is GPLv3 while `cailoxo` is MIT. Vendoring plugin code or binaries may affect licensing and distribution. Initial integration should treat `gitstatus` as an optional external dependency.

Use one of these runtime inputs:

- `CAILOXO_GITSTATUS_DIR=/path/to/gitstatus`
- `GITSTATUS_DAEMON=/path/to/gitstatusd`
- Existing `GITSTATUS_CACHE_DIR` for auto-downloaded binaries

### Config Shape

Add zsh-only backend configuration under the Git span settings.

```toml
[line.span.settings]
backend = "shell" # shell | gitstatus
gitstatus_dir = "/path/to/gitstatus"
gitstatus_timeout_ms = 10
gitstatus_max_index_size = -1
gitstatus_max_staged = 1
gitstatus_max_unstaged = 1
gitstatus_max_untracked = 1
gitstatus_max_conflicted = 1
```

Defaults:

- `backend = "shell"`
- `gitstatus_timeout_ms = 10`
- count limits match current prompt behavior, which only needs presence/count snippets

### Generated Zsh Flow

Source plugin with a suffix to avoid collisions.

```zsh
source "$CAILOXO_GITSTATUS_DIR/gitstatus.plugin.zsh" _cailoxo_
```

Start one daemon per shell session.

```zsh
gitstatus_start_cailoxo_ \
  -s $__CAILOXO_GITSTATUS_MAX_STAGED \
  -u $__CAILOXO_GITSTATUS_MAX_UNSTAGED \
  -d $__CAILOXO_GITSTATUS_MAX_UNTRACKED \
  -c $__CAILOXO_GITSTATUS_MAX_CONFLICTED \
  -m $__CAILOXO_GITSTATUS_MAX_INDEX_SIZE \
  -t $__CAILOXO_GITSTATUS_INIT_TIMEOUT_S \
  CAILOXO
```

Query from `__cailoxo_git_info`.

```zsh
gitstatus_query_cailoxo_ -d "$PWD" -t $__CAILOXO_GITSTATUS_QUERY_TIMEOUT_S CAILOXO
```

Map `VCS_STATUS_*` into existing cailoxo variables.

| cailoxo variable | gitstatus variable |
| --- | --- |
| `branch` | `VCS_STATUS_LOCAL_BRANCH`, fallback to short `VCS_STATUS_COMMIT` |
| `upstream_url` | `VCS_STATUS_REMOTE_URL` |
| `upstream` | derived from `VCS_STATUS_REMOTE_URL` |
| `upstream_icon` | derived from `VCS_STATUS_REMOTE_URL` |
| `ahead` | `VCS_STATUS_COMMITS_AHEAD` |
| `behind` | `VCS_STATUS_COMMITS_BEHIND` |
| `stashed` | `VCS_STATUS_STASHES` |
| `action` | `VCS_STATUS_ACTION` |
| `conflicted` | `VCS_STATUS_NUM_CONFLICTED` |
| `staged` | `VCS_STATUS_NUM_STAGED` |
| `modified` | `VCS_STATUS_NUM_UNSTAGED` |
| `untracked` | `VCS_STATUS_NUM_UNTRACKED` |
| `deleted` | `VCS_STATUS_NUM_UNSTAGED_DELETED + VCS_STATUS_NUM_STAGED_DELETED` |
| `renamed` | unsupported, set to `0` |

`git_dirty=1` when any dirty count is positive.

### Fallback Behavior

Fall back to current shell implementation if any of these happen:

- Plugin file is missing.
- zsh modules fail to load.
- daemon fails to start.
- query times out or returns error.
- backend config is not `gitstatus`.

Keep existing async `git fetch` logic separate. `gitstatusd` reads local repository state; it does not replace fetch scheduling.

### Main Tradeoffs

- Fastest path for zsh with proven implementation.
- Minimal cailoxo code compared to building a daemon.
- zsh-only at first.
- External GPL dependency requires careful packaging and docs.

## Plan B: Native Cross-Platform cailoxo Git Status Daemon

Build a small `cailoxo` companion app optimized for prompt Git queries on Linux, macOS, and Windows.

The goal is the same as `gitstatusd`: prompt rendering should perform cheap IPC, not run full Git commands or scan a repository on every prompt.

### Product Shape

Add hidden or documented subcommands to the `cailoxo` binary.

```sh
cailoxo git-daemon
cailoxo git-query --path "$PWD"
cailoxo git-stop
```

Prompt scripts normally talk directly to the daemon. `git-query` is mainly for debugging and fallback.

### Runtime Architecture

- One daemon per user/session or per shell profile.
- IPC transport:
  - Linux/macOS: Unix domain socket under `${XDG_RUNTIME_DIR:-/tmp}`.
  - Windows: named pipe under `\\.\pipe\cailoxo-gitstatus-$USER`.
- Request protocol: versioned field protocol, not JSON, to keep shell parsing simple.
- Response protocol: stable ordered fields matching prompt needs.
- Query timeout in prompt hot path, default 5 to 20 ms.
- If fresh status is not ready, daemon returns last known status plus `stale=1`, or prompt falls back to no Git segment.

### Repository Cache

Daemon keeps a cache per Git worktree.

Cached data:

- repo root and gitdir path
- HEAD commit
- branch name
- upstream ref, remote name, remote URL
- ahead/behind counts
- index metadata
- dirty counts
- stash count
- current action: merge, rebase, cherry-pick, revert, bisect
- last successful response

Invalidation inputs:

- `.git/HEAD`
- `.git/index`
- `.git/packed-refs`
- `.git/refs/**`
- `.git/config`
- `.git/logs/refs/stash`
- action files such as `MERGE_HEAD`, `CHERRY_PICK_HEAD`, `REVERT_HEAD`, `BISECT_LOG`, `rebase-merge`, `rebase-apply`, `sequencer`
- working tree directory mtimes for untracked and unstaged detection

Use file watches when available through Rust `notify`, with mtime polling fallback.

### Git Engine Options

Evaluate two implementation choices.

Option 1: `gix` / gitoxide.

- Mostly Rust implementation.
- Good fit for cross-platform static-ish distribution.
- Avoids native libgit2 packaging complexity.
- Need verify status performance and feature completeness for ignored files, sparse checkout, submodules, worktrees, and platform quirks.

Option 2: `git2` / libgit2.

- Mature status API.
- Similar conceptual model to existing gitstatus.
- Native dependency increases build and release complexity on Windows and macOS.
- Static linking and cross-compilation need extra work.

Preferred initial spike: `gix`, because cailoxo is Rust and release packaging stays simpler.

### Status Algorithm

Hot path request:

1. Resolve path to cached repo, walking parents only when cache miss.
2. If cache is fresh, return cached response immediately.
3. If cache is stale, schedule refresh on worker thread and return last known response if available.
4. If no cached response exists, compute with strict timeout or return `loading`.

Refresh path:

1. Read HEAD and branch.
2. Resolve upstream and remote URL.
3. Read index.
4. Compare index entries with filesystem metadata for unstaged changes.
5. Walk worktree with ignore rules for untracked files, respecting count caps.
6. Compute ahead/behind from commit graph only when upstream changed or HEAD changed.
7. Count stash entries from stash reflog.
8. Detect action files.
9. Store response atomically.

Count caps should stop work early:

- `max_staged`
- `max_unstaged`
- `max_untracked`
- `max_conflicted`
- `max_index_size_dirty`

Prompt often only needs count presence, not exhaustive counts. Early exit is key.

### Shell Integration

Zsh:

- Start daemon in `precmd` init if socket missing.
- Query through zsh module support for sockets if practical, otherwise use a tiny `cailoxo git-query` fallback with aggressive timeout.
- Use async callback later if fresh result arrives, similar to Powerlevel10k.

Nushell:

- Use `job spawn` for daemon startup.
- Query through `cailoxo git-query --format fields` until native socket support is worth adding.
- Keep timeout low and return cached/stale result.

PowerShell:

- Start daemon as background process.
- Query named pipe through a small .NET snippet generated into the prompt, or through `cailoxo git-query` first.
- Use named pipe for real fast path once protocol stabilizes.

### Response Fields

Use ordered fields similar to `gitstatusd`, trimmed to cailoxo needs.

```text
version
result
stale
workdir
commit
branch
remote_name
remote_url
action
index_size
staged
modified
conflicted
untracked
behind
stashed
unstaged_deleted
staged_deleted
```

Use ASCII unit separator and record separator, or newline-safe length-prefixed fields. Avoid JSON in prompt hot path.

### Cross-Platform Concerns

- Case-insensitive filesystems on Windows and default macOS volumes.
- Unicode normalization differences on macOS.
- Symlinks, junctions, and long paths on Windows.
- Worktrees and bare repositories.
- Submodules.
- Sparse checkout and skip-worktree bits.
- Assume-unchanged bits.
- Git config options that hide untracked or dirty state.
- Filesystem watchers can drop events, so mtime validation remains required.

### Milestones

1. Prototype daemon with fake/static response and cross-platform IPC.
2. Implement repo discovery and HEAD/branch/remote data.
3. Add index dirty counts with count caps.
4. Add untracked scan with ignore rules and count caps.
5. Add ahead/behind, stash, and action detection.
6. Integrate zsh backend behind `backend = "cailoxo-daemon"`.
7. Add PowerShell named pipe query.
8. Add Nushell query path.
9. Benchmark against shell implementation and `gitstatusd` on small, medium, and large repos.

### Main Tradeoffs

- Best long-term cross-platform story.
- Keeps licensing and distribution under cailoxo control.
- Larger implementation and testing burden.
- Hardest part is correctness across Git edge cases, not daemon IPC.

## Recommendation

Implement Plan A first for zsh as optional external integration. It gives immediate performance gains and validates the generated prompt API.

Use lessons from Plan A to shape Plan B. Build Plan B only after cailoxo has stable Git status fields, config names, timeout behavior, and fallback semantics.
