use std::fmt::Write;

use anyhow::{Context, Result, bail};

use crate::config::{Config, Span, SpanType};

pub const STATUSES: &[&str] = &[
    "ahead",
    "behind",
    "conflicted",
    "untracked",
    "modified",
    "staged",
    "renamed",
    "deleted",
    "stashed",
];

const DEFAULT_ICONS: &str = include_str!("../../defaults/icons.toml");

pub struct Layout<'a> {
    pub prompt_span: &'a Span,
    pub os: &'a Span,
    pub path: &'a Span,
    pub git: &'a Span,
}

pub fn layout(config: &Config) -> Result<Layout<'_>> {
    let first_line = config
        .line
        .first()
        .context("config must define a first line")?
        .span
        .as_slice();
    let prompt_span = config
        .line
        .get(1)
        .and_then(|line| line.span.first())
        .context("MVP requires second line with text prompt span")?;

    if prompt_span.span_type != SpanType::Text {
        bail!("MVP requires second line first span to have type = \"text\"");
    }

    let os = find_span(first_line, SpanType::Os).context("MVP requires os span on first line")?;
    let path =
        find_span(first_line, SpanType::Path).context("MVP requires path span on first line")?;
    let git =
        find_span(first_line, SpanType::Git).context("MVP requires git span on first line")?;

    Ok(Layout {
        prompt_span,
        os,
        path,
        git,
    })
}

fn find_span(spans: &[Span], span_type: SpanType) -> Option<&Span> {
    spans.iter().find(|span| span.span_type == span_type)
}

pub fn shell_single_quote(input: &str) -> String {
    format!("'{}'", input.replace('\'', "'\\''"))
}

pub fn nu_string(input: &str) -> String {
    let mut out = String::from('"');
    for ch in input.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{1b}' => out.push_str("\\u{1b}"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

pub fn zsh_prompt_single_quote(input: &str) -> String {
    shell_single_quote(&input.replace('%', "%%"))
}

pub fn git_branch_icon(git: &Span) -> Result<String> {
    if let Some(icon) = git.setting_str("branch_icon") {
        return Ok(icon.to_string());
    }

    let icons = default_icons()?;
    Ok(icon(&icons, "git_branch", "nerdfont", "git")
        .unwrap_or("")
        .to_string())
}

pub fn status_separator(git: &Span) -> &str {
    git.setting_str("separator").unwrap_or(" |")
}

pub fn status_template(span: &Span, name: &str) -> Result<String> {
    let icon_set = span.setting_str("icon_set").unwrap_or("minimal");
    let icons = default_icons()?;
    let status_icon = icon(&icons, "git_status", icon_set, name)
        .or_else(|| icon(&icons, "git_status", "minimal", name))
        .unwrap_or("");

    if let Some(template) = span.status_template(name) {
        return Ok(template
            .replace("{{ status_icon }}", status_icon)
            .replace("{{status_icon}}", status_icon));
    }

    Ok(format!("{status_icon}{{{{ count }}}}"))
}

pub fn zsh_status_template_cases(git: &Span) -> Result<String> {
    let mut out = String::new();
    for status in STATUSES {
        let template = status_template(git, status)?;
        writeln!(
            out,
            "    {status}) template={} ; count=${{{status}}} ;;",
            shell_single_quote(&template)
        )
        .unwrap();
    }
    Ok(out)
}

pub fn nu_status_templates(git: &Span) -> Result<String> {
    let mut out = String::from("{");
    for (index, status) in STATUSES.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        let template = status_template(git, status)?;
        write!(out, "{}: {}", status, nu_string(&template)).unwrap();
    }
    out.push('}');
    Ok(out)
}

pub fn zsh_os_icon_cases() -> Result<String> {
    let icons = default_icons()?;
    let table = icon_table(&icons, "os", "nerdfont").context("missing os.nerdfont icons")?;
    let mut out = String::new();
    for (name, value) in table {
        let Some(icon) = value.as_str() else {
            continue;
        };
        writeln!(
            out,
            "      {name}) print -r -- {} ;;",
            shell_single_quote(icon)
        )
        .unwrap();
    }
    Ok(out)
}

pub fn zsh_os_unknown_icon() -> Result<String> {
    let icons = default_icons()?;
    Ok(shell_single_quote(
        icon(&icons, "os", "nerdfont", "unknown").unwrap_or(""),
    ))
}

pub fn nu_os_icon_conditions() -> Result<String> {
    let icons = default_icons()?;
    let table = icon_table(&icons, "os", "nerdfont").context("missing os.nerdfont icons")?;
    let unknown = icon(&icons, "os", "nerdfont", "unknown").unwrap_or("");
    let mut out = String::new();
    let mut first = true;
    let mut entries: Vec<_> = table.iter().collect();
    entries.sort_by_key(|(name, _)| std::cmp::Reverse(name.len()));
    for (name, value) in entries {
        if name == "unknown" {
            continue;
        }
        let Some(icon) = value.as_str() else {
            continue;
        };
        if first {
            write!(
                out,
                "if ($os | str contains {}) {{ {} }}",
                nu_string(name),
                nu_string(icon)
            )
            .unwrap();
            first = false;
        } else {
            write!(
                out,
                " else if ($os | str contains {}) {{ {} }}",
                nu_string(name),
                nu_string(icon)
            )
            .unwrap();
        }
    }
    write!(out, " else {{ {} }}", nu_string(unknown)).unwrap();
    Ok(out)
}

pub fn zsh_upstream_icon_cases() -> Result<String> {
    let icons = default_icons()?;
    let table = icon_table(&icons, "git_upstream", "nerdfont")
        .context("missing git_upstream.nerdfont icons")?;
    let mut out = String::new();
    for (name, value) in table {
        let Some(icon) = value.as_str() else {
            continue;
        };
        writeln!(
            out,
            "      {name}) upstream_icon={} ;;",
            shell_single_quote(icon)
        )
        .unwrap();
    }
    Ok(out)
}

pub fn nu_upstream_icons() -> Result<String> {
    let icons = default_icons()?;
    let table = icon_table(&icons, "git_upstream", "nerdfont")
        .context("missing git_upstream.nerdfont icons")?;
    let mut out = String::from("{");
    for (index, (name, value)) in table.iter().enumerate() {
        let Some(icon) = value.as_str() else {
            continue;
        };
        if index > 0 {
            out.push_str(", ");
        }
        write!(out, "{}: {}", name, nu_string(icon)).unwrap();
    }
    out.push('}');
    Ok(out)
}

pub fn icon_pairs(group: &str, set: &str) -> Result<Vec<(String, String)>> {
    let icons = default_icons()?;
    let table = icon_table(&icons, group, set).context("missing icon table")?;
    let mut pairs = Vec::new();
    for (name, value) in table {
        let Some(icon) = value.as_str() else {
            continue;
        };
        pairs.push((name.to_string(), icon.to_string()));
    }
    Ok(pairs)
}

fn default_icons() -> Result<toml::Value> {
    toml::from_str(DEFAULT_ICONS).context("failed to parse defaults/icons.toml")
}

fn icon<'a>(icons: &'a toml::Value, group: &str, set: &str, key: &str) -> Option<&'a str> {
    icons.get(group)?.get(set)?.get(key)?.as_str()
}

fn icon_table<'a>(
    icons: &'a toml::Value,
    group: &str,
    set: &str,
) -> Option<&'a toml::map::Map<String, toml::Value>> {
    icons.get(group)?.get(set)?.as_table()
}

pub fn color_escape(color: Option<&String>, background: bool) -> Result<String> {
    let Some(color) = color else {
        return Ok(String::new());
    };
    let color = color.as_str();
    if color == "transparent" || color == "default" {
        return Ok(String::new());
    }

    let prefix = if background { 48 } else { 38 };
    if let Ok(index) = color.parse::<u8>() {
        return Ok(format!("\u{1b}[{prefix};5;{index}m"));
    }

    if let Some(hex) = color.strip_prefix('#')
        && hex.len() == 6
    {
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        return Ok(format!("\u{1b}[{prefix};2;{r};{g};{b}m"));
    }

    let code = match color {
        "black" => 0,
        "red" => 1,
        "green" => 2,
        "yellow" => 3,
        "blue" => 4,
        "magenta" => 5,
        "cyan" => 6,
        "white" => 7,
        "bright-black" | "darkGray" => 8,
        "bright-red" | "lightRed" => 9,
        "bright-green" | "lightGreen" => 10,
        "bright-yellow" | "lightYellow" => 11,
        "bright-blue" | "lightBlue" => 12,
        "bright-magenta" | "lightMagenta" => 13,
        "bright-cyan" | "lightCyan" => 14,
        "bright-white" | "lightWhite" => 15,
        other => bail!("unsupported color {other:?}"),
    };
    Ok(format!("\u{1b}[{prefix};5;{code}m"))
}

pub fn ansi_pair(fg: Option<&String>, bg: Option<&String>) -> Result<String> {
    let mut out = String::new();
    out.push_str(&color_escape(fg, false)?);
    out.push_str(&color_escape(bg, true)?);
    Ok(out)
}

pub fn zsh_prompt_ansi(input: &str) -> String {
    input.replace('\u{1b}', "\\e")
}
