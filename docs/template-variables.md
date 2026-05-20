# Template Variables

`cailoxo` templates use a small variable and conditional syntax. Variables are replaced by generated shell scripts at prompt render time.

## Syntax

Variable substitution:

```text
{{ variable }}
```

Simple truthy conditional:

```text
{{ if variable }}then{{ else }}else{{ end }}
```

Supported conditionals in MVP:

- `{{ if status }}` in Git templates.
- `{{ if home }}` in path templates.

Other condition names are not evaluated yet.

## Decoration Tags

Templates and template-like path format settings support OMP-style decoration tags.

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

Path `edge_format` and `gitdir_format` use `%s` format strings and also support decoration tags.

## OS Span

Available when `type = "os"`:

| Variable | Meaning |
| --- | --- |
| `icon` | OS/platform icon selected from `defaults/icons.toml` |

Example:

```toml
template = " {{ icon }} "
```

## Path Span

Available when `type = "path"`:

| Variable | Meaning |
| --- | --- |
| `path` | Rendered path after adaptive truncation and formatting |
| `home` | True when current directory is home |
| `home_icon` | Configured home icon |
| `folder_icon` | Configured folder icon |

Example:

```toml
template = " {{ if home }}{{ home_icon }} {{ else }}{{ folder_icon }} {{ end }}{{ path }} "
```

## Git Span

Available when `type = "git"`:

| Variable | Meaning |
| --- | --- |
| `branch` | Current branch name, or detached HEAD short SHA |
| `branch_icon` | Git branch icon selected from `defaults/icons.toml` or `branch_icon` setting |
| `status` | Rendered Git status string |
| `upstream` | Upstream provider name, such as `github`, `gitlab`, or `azure_devops` |
| `upstream_icon` | Upstream provider icon selected from `defaults/icons.toml` |
| `upstream_url` | Remote URL from local Git config; zsh/pwsh clean it to browser URL when Git `url = true` |

Example:

```toml
template = " {{ upstream_icon }}{{ branch_icon }}{{ branch }}{{ if status }} {{ status }}{{ end }} "
```

## Git Status Templates

Git status templates receive:

| Variable | Meaning |
| --- | --- |
| `count` | Count for current status kind |
| `status_icon` | Icon for current status kind from selected `git_status.<icon_set>` |
| `action` | Git repository action for the `action` status kind |

Status kinds:

| Status | Meaning |
| --- | --- |
| `behind` | Local commits behind upstream, using existing local refs |
| `ahead` | Local commits ahead of upstream, using existing local refs |
| `stashed` | Stash entries |
| `action` | Current Git action, such as `merge`, `cherry`, `rebase-i`, or `bisect` |
| `conflicted` | Files with merge conflicts |
| `staged` | Files staged in index |
| `modified` | Modified files in worktree |
| `untracked` | Untracked files |
| `renamed` | Renamed files |
| `deleted` | Deleted files |

Example:

```toml
[line.span.settings.status]
modified = "{{ status_icon }}{{ count }}"
```

## Text Span

`type = "text"` is used for the prompt character. MVP text spans render `template` with decoration tags only; no variables are currently replaced.
