# Template Variables

`cailoxo` templates use named variables provided by each span type. Variables are part of the public config surface and should be documented before use in examples.

## Template Syntax

MVP templates support variable substitution:

```text
{{ variable }}
```

MVP templates also support simple truthy conditionals:

```text
{{ if variable }}...{{ else }}...{{ end }}
```

Templates and template-like format settings support OMP-style decoration tags. Path `edge_format` and `gitdir_format` are `%s` format strings that wrap a generated path part.

```text
<b>{{ branch }}</b>
<b><i>%s</i></b>
```

Supported tags:

| Tag | Style |
| --- | --- |
| `<b>...</b>` | bold |
| `<i>...</i>` | italic |
| `<u>...</u>` | underline |
| `<o>...</o>` | overline |
| `<s>...</s>` | strikethrough |
| `<d>...</d>` | dim |
| `<f>...</f>` | blink |
| `<r>...</r>` | reverse |

## Common Variables

Available to every span:

| Variable | Meaning |
| --- | --- |
| `shell` | Current shell name, such as `zsh`, `pwsh`, or `nu` |
| `status_code` | Exit code from previous command |
| `success` | True when `status_code` is `0` |
| `error` | True when `status_code` is not `0` |

## OS Span

Available when `type = "os"`:

| Variable | Meaning |
| --- | --- |
| `icon` | Icon selected from built-in OS icon dictionary or user override |
| `name` | Normalized OS name, such as `ubuntu`, `macos`, or `windows` |
| `family` | OS family, such as `linux`, `macos`, or `windows` |
| `distro` | Linux distro name when available |

## Path Span

Available when `type = "path"`:

| Variable | Meaning |
| --- | --- |
| `path` | Rendered path after mapping and adaptive truncation |
| `cwd` | Full current working directory |
| `folder` | Current directory basename |
| `home` | True when current directory is home |
| `home_icon` | Configured home icon |
| `folder_icon` | Configured folder icon |
| `repo_root` | Git repository root basename when inside repo |
| `in_repo` | True when current directory is inside Git repo |

Example:

```toml
template = " {{ if home }}{{ home_icon }} {{ else }}{{ folder_icon }} {{ end }}{{ path }} "
```

## Git Span

Available when `type = "git"`:

| Variable | Meaning |
| --- | --- |
| `branch` | Current branch name |
| `branch_icon` | Icon selected from built-in Git branch icon dictionary or user override |
| `head` | Branch name or detached HEAD short SHA |
| `detached` | True when HEAD is detached |
| `status` | Rendered local Git status string |
| `upstream` | Git upstream provider name, such as `github` or `gitlab`, when known |
| `upstream_icon` | Icon selected from built-in Git upstream icon dictionary or user override |
| `upstream_url` | Configured upstream URL when available locally |

Remote variables must come from existing local Git config only. Prompt rendering must not fetch network state. Built-in Git branch, upstream, and status icons come from `defaults/icons.toml`.

## Git Status Templates

Git status templates receive a smaller variable set:

| Variable | Meaning |
| --- | --- |
| `count` | Count for current status kind |
| `status_icon` | Icon for current status kind from selected `git_status.<icon_set>` |

Status kinds:

| Status | Meaning |
| --- | --- |
| `ahead` | Local commits ahead of upstream, using existing local refs only |
| `behind` | Local commits behind upstream, using existing local refs only |
| `conflicted` | Files with merge conflicts |
| `untracked` | Untracked files |
| `modified` | Modified files in worktree |
| `staged` | Files staged in index |
| `renamed` | Renamed files |
| `deleted` | Deleted files |
| `stashed` | Stash entries |

Example:

```toml
[line.span.settings]
icon_set = "minimal"
separator = " |"

[line.span.settings.status]
modified = "{{ status_icon }}{{ count }}"
```


## Text Span

Available when `type = "text"`:

| Variable | Meaning |
| --- | --- |
| `status_code` | Exit code from previous command |
| `success` | True when `status_code` is `0` |
| `error` | True when `status_code` is not `0` |
