use std::collections::HashMap;

use serde::Serialize;

use crate::types::{ConfigEntry, OptionValue};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ParseResult {
    options: HashMap<String, OptionValue>,
    parameters: HashMap<String, String>,
    operands: Vec<String>,
}

impl ParseResult {
    pub fn options(&self) -> &HashMap<String, OptionValue> {
        &self.options
    }

    pub fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    pub fn operands(&self) -> &[String] {
        &self.operands
    }
}

pub struct ArgParser {
    configs: HashMap<String, ConfigEntry>,
}

#[derive(Debug, Clone, PartialEq)]
enum CliArg {
    /// A GNU-style long option
    Long { name: String, value: Option<String> },

    /// A Unix-style short option
    Short { flags: String },

    /// A name=value parameter assignment
    Parameter(String, String),

    /// A positional argument
    Operand,
}

fn get_opt_value(arg: &str) -> CliArg {
    if arg == "--" {
        CliArg::Operand
    } else if let Some(stripped) = arg.strip_prefix("--") {
        let mut parts = stripped.splitn(2, '=');
        let name = parts.next().unwrap_or_default().to_string();
        let value = parts.next().map(|v| v.to_string());
        if name.is_empty() {
            CliArg::Operand
        } else {
            CliArg::Long { name, value }
        }
    } else if let Some(flags) = arg.strip_prefix('-') {
        CliArg::Short { flags: flags.to_string() }
    } else {
        let mut parts = arg.splitn(2, '=');
        let name: String = parts.next().unwrap_or_default().to_string();
        if let Some(value) = parts.next().map(|v: &str| v.to_string()) {
            CliArg::Parameter(name, value)
        } else {
            CliArg::Operand
        }
    }
}

impl ArgParser {
    pub fn new(configs: HashMap<String, ConfigEntry>) -> Self {
        Self { configs }
    }

    pub fn parse<I, S>(&self, arg_list: I) -> Result<ParseResult, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut options: HashMap<String, OptionValue> = HashMap::new();
        let mut parameters: HashMap<String, String> = HashMap::new();
        let mut operands: Vec<String> = Vec::new();

        let mut stop_parsing = false;
        let mut args = arg_list.into_iter().peekable();

        while let Some(arg) = args.next() {
            let arg: &str = arg.as_ref();

            if arg == "--" && !stop_parsing {
                stop_parsing = true;
                continue;
            }

            if stop_parsing {
                operands.push(arg.into());
                continue;
            }

            match get_opt_value(arg) {
                CliArg::Short { flags } => {
                    let (skip, mut pairs) = self
                        .parse_short_option(&flags, args.peek())?;

                    for (name, value) in pairs.drain() {
                        options.insert(name, value);
                    }

                    if skip {
                        let _ = args.next();
                    }
                },
                CliArg::Long { name, value } => {
                    let (name, value) = self
                        .parse_long_option(&name, value.as_deref())?;
                    match self.configs.get(name) {
                        Some(ConfigEntry::Count) => {
                            match options.get_mut(name) {
                                Some(OptionValue::Int(old_value)) => {
                                    if let OptionValue::Int(new_value) = value {
                                        *old_value += new_value;
                                    } else {
                                        todo!(); /* parsed value is not int */
                                    };
                                },
                                None => {
                                    options.insert(name.into(), value);
                                },
                                _ => { todo!() /* stored value is not int */ },
                            }
                        },
                        Some(ConfigEntry::List { .. }) => {
                            match options.get_mut(name) {
                                Some(OptionValue::List(old_value)) => {
                                    if let OptionValue::List(new_value) = value {
                                        old_value.extend_from_slice(&new_value);
                                    } else {
                                        todo!(); /* parsed value is not list */
                                    };
                                },
                                None => {
                                    options.insert(name.into(), value);
                                },
                                _ => { todo!() /* stored value is not list */ },
                            }
                        },
                        /* other entry types */ _ => {
                            options.insert(name.into(), value);
                        },
                    }
                },
                CliArg::Parameter(name, value) => {
                    parameters.insert(name, value);
                },
                CliArg::Operand => {
                    operands.push(arg.into());
                },
            }
        }

        Ok(ParseResult {
            options,
            parameters,
            operands,
        })
    }

    fn parse_long_option<'parser, 'option, 'result>(
        &'parser self,
        name: &'option str,
        value: Option<&str>,
    ) -> Result<(&'result str, OptionValue), String>
    where
        'parser: 'result,
        'option: 'result,
    {
        macro_rules! flag_option {
            ($name:ident) => {{ Ok(($name, OptionValue::Flag)) }}
        }

        macro_rules! text_option {
            ($name:ident, $value:ident, $default:ident) => {{
                if let Some(text) = $value {
                    Ok(($name, OptionValue::Text(text.into())))
                } else if let Some(text) = $default {
                    Ok(($name, OptionValue::Text(text.into())))
                } else {
                    Err("null arg".into())
                }
            }}
        }

        macro_rules! int_option {
            ($name:ident, $value:ident, $default:ident) => {{
                match $value {
                    Some(text) if !text.is_empty() => {
                        if let Ok(num) = text.parse::<i64>() {
                            Ok(($name, OptionValue::Int(num)))
                        } else {
                            Err("invalid int".into())
                        }
                    },
                    _ => {
                        if let Some(num) = $default {
                            Ok(($name, OptionValue::Int(*num)))
                        } else {
                            Err("null int".into())
                        }
                    }
                }
            }}
        }

        macro_rules! count_option {
            ($name:ident, $val:ident) => {{
                if $val.is_some() && $val.unwrap().parse::<i64>().is_err() {
                    Err("invalid int".into())
                } else if let Some(text) = $val {
                    let num = text.parse().unwrap();
                    Ok(($name, OptionValue::Int(num)))
                } else {
                    Ok(($name, OptionValue::Int(1)))
                }
            }}
        }

        macro_rules! list_option {
            ($name:ident, $value:ident, $sep:ident) => {{
                if $value.is_some() && $value.unwrap().is_empty() {
                    Ok(($name, OptionValue::List(Vec::new())))
                } else if let Some(text) = $value {
                    let sep: &str = $sep.as_deref().unwrap_or(",");
                    let list: Vec<String> = text
                        .split(sep)
                        .map(|item: &str| item.to_string())
                        .collect();
                    Ok(($name, OptionValue::List(list)))
                } else {
                    Err("null arg".into())
                }
            }}
        }

        if let Some(entry) = self.configs.get(name) {
            match entry {
                ConfigEntry::Flag => flag_option!(name),
                ConfigEntry::Text { default } => {
                    text_option!(name, value, default)
                },
                ConfigEntry::Int { default } => {
                    int_option!(name, value, default)
                },
                ConfigEntry::Count => count_option!(name, value),
                ConfigEntry::List { sep } => {
                    list_option!(name, value, sep)
                },
                ConfigEntry::Alias { target } => {
                    if let Some(target_entry) = self.configs.get(target) {
                        match target_entry {
                            ConfigEntry::Flag => flag_option!(target),
                            ConfigEntry::Text { default } => {
                                text_option!(target, value, default)
                            },
                            ConfigEntry::Int { default } => {
                                int_option!(target, value, default)
                            },
                            ConfigEntry::Count => count_option!(target, value),
                            ConfigEntry::List { sep } => {
                                list_option!(target, value, sep)
                            },
                            ConfigEntry::Alias { .. } => {
                                Err("alias to alias".into())
                            },
                        }
                    } else {
                        Err("target option not found".into())
                    }
                },
            }
        } else {
            Err("config option not found".into())
        }
    }

    fn parse_short_option<S>(
        &self,
        arg: &str,
        next_arg: Option<&S>,
    ) -> Result<(bool, HashMap<String, OptionValue>), String>
    where
        S: AsRef<str>,
    {
        let n: usize = arg.len();
        let mut pairs: HashMap<String, OptionValue> = HashMap::new();
        let mut iter = arg.char_indices();
        let _ = iter.next(); /* discard leading - */

        while let Some((i, flag)) = iter.next() {
            let name: String = String::from(flag);
            let next_value: Option<&str> = next_arg.map(|v| v.as_ref());
            let Some(entry) = self.configs.get(&name) else {
                return Err("option not supported".into());
            };

            match entry {
                ConfigEntry::Flag => {
                    pairs.insert(name, OptionValue::Flag);
                },
                ConfigEntry::Text { default } => {
                    if i < n - flag.len_utf8() {
                        let value = arg[i + flag.len_utf8()..n].to_string();
                        pairs.insert(name, OptionValue::Text(value));
                        return Ok((false, pairs));
                    } else if let Some(value) = default {
                        pairs.insert(name, OptionValue::Text(value.into()));
                        return Ok((false, pairs));
                    } else if let Some(value) = next_value {
                        pairs.insert(name, OptionValue::Text(value.into()));
                        return Ok((true, pairs));
                    }
                    return Err("null arg".into());
                },
                ConfigEntry::Int { default } => {
                    if i < n - flag.len_utf8() {
                        let value = arg[i + flag.len_utf8()..n].to_string();
                        if let Ok(num) = value.parse() {
                            pairs.insert(name, OptionValue::Int(num));
                            return Ok((false, pairs));
                        } else {
                            return Err("invalid int".into());
                        }
                    } else if let Some(num) = default {
                        pairs.insert(name, OptionValue::Int(*num));
                        return Ok((false, pairs));
                    } else if let Some(value) = next_value {
                        if let Ok(num) = value.parse() {
                            pairs.insert(name, OptionValue::Int(num));
                            return Ok((true, pairs));
                        } else {
                            return Err("invalid int".into());
                        }
                    }
                    return Err("null int".into());
                },
                ConfigEntry::Count => {
                    let default = OptionValue::Int(0);
                    let old_value: &OptionValue = pairs.get(&name)
                        .unwrap_or(&default);
                    pairs.insert(name, old_value.clone() + 1);
                },
                ConfigEntry::List { sep } => {
                    let sep: &str = sep.as_deref().unwrap_or(",");
                    if i < n - flag.len_utf8() {
                        let value = arg[i + flag.len_utf8()..n].to_string();
                        let parsed_value: Vec<String> = value
                            .split(sep)
                            .map(|item: &str| item.to_string())
                            .collect();
                        pairs.insert(name, OptionValue::List(parsed_value));
                        return Ok((false, pairs));
                    } else if let Some(value) = next_value {
                        let parsed_value = if value.is_empty() {
                            Vec::new()
                        } else {
                            value
                                .split(sep)
                                .map(|item: &str| item.to_string())
                                .collect()
                        };
                        pairs.insert(name, OptionValue::List(parsed_value));
                        return Ok((true, pairs));
                    }
                    return Err("null arg".into());
                },
                ConfigEntry::Alias { target } => {
                    if let Some(target_entry) = self.configs.get(target) {
                        let target: String = target.clone();
                        match target_entry {
                            ConfigEntry::Flag => {
                                pairs.insert(target, OptionValue::Flag);
                            },
                            ConfigEntry::Text { default } => {
                                if i < n - flag.len_utf8() {
                                    let value = arg[i + flag.len_utf8()..n]
                                        .to_string();
                                    pairs.insert(
                                        target,
                                        OptionValue::Text(value.into())
                                    );
                                    return Ok((false, pairs));
                                } else if let Some(value) = default {
                                    pairs.insert(
                                        target,
                                        OptionValue::Text(value.into())
                                    );
                                    return Ok((false, pairs));
                                } else if let Some(value) = next_value {
                                    pairs.insert(
                                        target,
                                        OptionValue::Text(value.into())
                                    );
                                    return Ok((true, pairs));
                                }
                                return Err("null arg".into());
                            },
                            ConfigEntry::Int { default } => {
                                if i < n - flag.len_utf8() {
                                    let value = arg[i + flag.len_utf8()..n]
                                        .to_string();
                                    if let Ok(num) = value.parse() {
                                        pairs.insert(
                                            target,
                                            OptionValue::Int(num),
                                        );
                                        return Ok((false, pairs));
                                    } else {
                                        return Err("invalid int".into());
                                    }
                                } else if let Some(num) = default {
                                    pairs.insert(
                                        target,
                                        OptionValue::Int(*num),
                                    );
                                    return Ok((false, pairs));
                                } else if let Some(value) = next_value {
                                    if let Ok(num) = value.parse() {
                                        pairs.insert(
                                            target,
                                            OptionValue::Int(num),
                                        );
                                        return Ok((true, pairs));
                                    } else {
                                        return Err("invalid int".into());
                                    }
                                }
                                return Err("null int".into());
                            },
                            ConfigEntry::Count => {
                                let default = OptionValue::Int(0);
                                let old_value = pairs.get(&target)
                                    .unwrap_or(&default);
                                pairs.insert(target, old_value.clone() + 1);
                            },
                            ConfigEntry::List { sep } => {
                                let sep: &str = sep.as_deref().unwrap_or(",");
                                if i < n - flag.len_utf8() {
                                    let value = arg[i + flag.len_utf8()..n]
                                        .to_string();
                                    let parsed_value: Vec<String> = value
                                        .split(sep)
                                        .map(|item: &str| item.to_string())
                                        .collect();
                                    pairs.insert(
                                        target,
                                        OptionValue::List(parsed_value),
                                    );
                                    return Ok((false, pairs));
                                } else if let Some(value) = next_value {
                                    let parsed_value = if value.is_empty() {
                                        Vec::new()
                                    } else {
                                        value
                                            .split(sep)
                                            .map(|item: &str| item.to_string())
                                            .collect()
                                    };
                                    pairs.insert(
                                        target,
                                        OptionValue::List(parsed_value),
                                    );
                                    return Ok((true, pairs));
                                }
                                return Err("null arg".into());
                            },
                            ConfigEntry::Alias { .. } => {
                                return Err("alias to alias".into());
                            },
                        }
                    } else {
                        return Err("target option not found".into());
                    }
                },
            }
        }

        Ok((false, pairs))
    }
}

#[cfg(test)]
mod test_parse {
    use std::env;
    use super::*;
    use crate::entries;

    #[test]
    fn parse_input() {
        let configs = entries! {
            "quiet" => Flag,
            "color" => Text,
        };
        let parser = ArgParser::new(configs);
        let mut input = ["--quiet", "build", "CC=clang"];
        let result = parser.parse(input);

        let operands = ["build"];
        let options: HashMap<String, OptionValue> = HashMap::from([
            ("quiet".to_string(), OptionValue::Flag),
        ]);
        let parameters: HashMap<String, String> = HashMap::from([
            ("CC".to_string(), "clang".to_string()),
        ]);

        assert_eq!(result.clone().unwrap().options, options);
        assert_eq!(result.clone().unwrap().parameters, parameters);
        assert_eq!(result.clone().unwrap().operands, operands);
    }
}
