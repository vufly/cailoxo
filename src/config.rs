use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    #[serde(default)]
    pub final_space: bool,
    #[serde(default)]
    pub transient: Option<Transient>,
    #[serde(default)]
    pub line: Vec<Line>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transient {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub success_foreground: Option<String>,
    #[serde(default)]
    pub error_foreground: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Line {
    #[allow(dead_code)]
    #[serde(default)]
    pub newline: bool,
    #[serde(default)]
    pub span: Vec<Span>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Span {
    #[serde(rename = "type")]
    pub span_type: SpanType,
    pub template: String,
    #[serde(default)]
    pub foreground: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub dirty_background: Option<String>,
    #[serde(default)]
    pub success_foreground: Option<String>,
    #[serde(default)]
    pub error_foreground: Option<String>,
    #[serde(default)]
    pub head: Option<String>,
    #[serde(default)]
    pub tail: Option<String>,
    #[serde(default)]
    pub separator: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    pub invert_head: bool,
    #[serde(default)]
    pub invert_tail: bool,
    #[serde(default)]
    pub invert_separator: bool,
    #[serde(default = "default_settings")]
    pub settings: toml::Value,
}

fn default_settings() -> toml::Value {
    toml::Value::Table(toml::map::Map::new())
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpanType {
    Os,
    Path,
    Git,
    Text,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("unsupported config version {0}; expected version 1")]
    Version(u32),
    #[error("config must define at least one [[line]]")]
    NoLines,
    #[error("line {0} must define at least one [[line.span]]")]
    EmptyLine(usize),
    #[error(
        "line {line}: span {span} has separator and span {next_span} has head; boundary can only define one edge"
    )]
    EdgeConflict {
        line: usize,
        span: usize,
        next_span: usize,
    },
}

impl Config {
    pub fn parse(input: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(input)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.version != 1 {
            return Err(ConfigError::Version(self.version));
        }
        if self.line.is_empty() {
            return Err(ConfigError::NoLines);
        }

        for (line_index, line) in self.line.iter().enumerate() {
            if line.span.is_empty() {
                return Err(ConfigError::EmptyLine(line_index + 1));
            }

            for (span_index, pair) in line.span.windows(2).enumerate() {
                if pair[0].separator.is_some() && pair[1].head.is_some() {
                    return Err(ConfigError::EdgeConflict {
                        line: line_index + 1,
                        span: span_index + 1,
                        next_span: span_index + 2,
                    });
                }
            }
        }

        Ok(())
    }
}

impl Span {
    pub fn setting_str(&self, key: &str) -> Option<&str> {
        self.settings.get(key)?.as_str()
    }

    pub fn setting_bool(&self, key: &str) -> Option<bool> {
        self.settings.get(key)?.as_bool()
    }

    pub fn status_template(&self, key: &str) -> Option<&str> {
        self.settings.get("status")?.get(key)?.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_edge_conflict() {
        let config = Config::parse(
            r#"
version = 1

[[line]]
[[line.span]]
type = "path"
template = "{{ path }}"
separator = ""

[[line.span]]
type = "git"
template = "{{ branch }}"
head = ""
"#,
        )
        .unwrap();

        let err = config.validate().unwrap_err().to_string();
        assert!(err.contains("boundary can only define one edge"));
    }
}
