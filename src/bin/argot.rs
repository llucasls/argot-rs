#![cfg(feature = "cli")]

#[cfg(not(any(feature = "json", feature = "toml")))]
compile_error!("The CLI requires at least one serialization format.");

use std::env;
use std::ffi::OsStr;
use std::fs::{File, create_dir_all};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{PathBuf, Path};
use std::process::ExitCode;

#[cfg(feature = "json")]
use serde_json::Value as JsonValue;

use argot_cli::{
    ArgParser,
    OptionValue,
    entries,
    read_json_config,
};

/// Read an environment variable and return its value as a PathBuf.
/// If the variable is unset, return a default value instead.
fn dir_path<O, P>(dir_path: O, default: P) -> PathBuf
where
    O: AsRef<OsStr>,
    P: AsRef<Path>,
{
    match env::var_os(dir_path) {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(default.as_ref()),
    }
}

/// Take user input, parse options and parameters and store in a state file
fn save_state<I, S>(
    parser: &ArgParser,
    arg_list: I,
    file: &mut File,
) -> io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<String> = arg_list
        .into_iter()
        //.map(|arg| arg.as_ref().to_os_string()) // TODO: support OsString
        .map(|arg| arg.as_ref().to_string_lossy().to_string())
        .collect();

    let Ok(data) = parser.parse(args) else {
        let kind = io::ErrorKind::InvalidData;
        let error = io::Error::from(kind);
        return Err(error);
    };
    let json_data: Vec<u8> = serde_json::to_vec(&data)?;

    let mut writer = BufWriter::new(file);
    writer.write_all(&json_data)?;
    let _ = writer.write(b"\n")?;
    Ok(())
}

/// Read state file and return its parsed data
fn read_state<P>(filepath: P) -> io::Result<JsonValue>
where
    P: AsRef<Path>,
{
    let file = match File::open(filepath.as_ref()) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            eprintln!("{}", e);
            let kind = io::ErrorKind::NotFound;
            let msg = format!("state file {:?} was not found", filepath.as_ref());
            let error = io::Error::new(kind, msg);
            return Err(error);
        },
        Err(e) => { return Err(e); },
    };

    let reader = BufReader::new(file);
    let state: JsonValue = serde_json::from_reader(reader)?;

    Ok(state)
}

fn has_whitespace(text: &str) -> bool {
    for c in text.chars() {
        if c.is_whitespace() {
            return true;
        }
    }
    false
}

unsafe extern "C" { fn getuid() -> u32; }

fn main() -> ExitCode {
    macro_rules! error {
        ($msg:expr) => {{
            eprintln!("{}", $msg);
            return ExitCode::FAILURE;
        }};
        ($exit_code:literal, $msg:expr) => {{
            eprintln!("{}", $msg);
            return ExitCode::from($exit_code);
        }};
        ($exit_code:literal, $template:literal, $( $arg:expr ),+) => {{
            eprintln!($template, $( $arg ),+);
            return ExitCode::from($exit_code);
        }}
    }

    let mut args = env::args_os();
    let _argv0 = args.next();

    let argot_cli_configs = config! {
        "config" => Text,
        "c" => Alias { target: "config" },
        "state-file" => Text,
        "f" => Alias { target: "state-file" },
        "sep" => Text,
        "s" => Alias { target: "sep" },
    };

    let argot_cli_parser = ArgParser::new(argot_cli_configs.unwrap());
    let string_args: Vec<String> = args
        .map(|arg| arg.to_string_lossy().to_string())
        .collect();
    let argot_cli_data = match argot_cli_parser.parse(string_args) {
        Ok(res) => res,
        Err(e) => error!(2, e),
    };

    let options = argot_cli_data.options();
    let operands = argot_cli_data.operands();

    let uid: u32 = unsafe { getuid() };

    let runtime_dir = dir_path(
        "XDG_RUNTIME_DIR",
        format!("/tmp/argot/{}", uid),
    );
    let state_dir = runtime_dir.join("argot");
    let home_dir = env::home_dir().expect("HOME not set");
    let config_dir = dir_path("XDG_CONFIG_HOME", home_dir.join(".config"));

    let config_file_path: PathBuf = match options.get("config") {
        Some(OptionValue::Text(path)) => PathBuf::from(path),
        _ => {
            let current_dir = match env::current_dir() {
                Ok(dir) => dir,
                Err(e) => error!(e),
            };
            if match current_dir.join("argot.json").try_exists() {
                Ok(file_exists) => file_exists,
                Err(e) => error!(e),
            } {
                current_dir.join("argot.json")
            } else if match config_dir.join("argot.json").try_exists() {
                Ok(file_exists) => file_exists,
                Err(e) => error!(e),
            } {
                config_dir.join("argot.json")
            } else {
                error!(
                    1,
                    "Error: {}\n{}",
                    "no configuration file was found",
                    "try using the -c option or create an ./argot.json file"
                );
            }
        }
    };

    let state_file: PathBuf = match options.get("state-file") {
        Some(OptionValue::Text(path)) => PathBuf::from(path),
        _ => state_dir.join("state.json"),
    };

    let sep: &str = match options.get("sep") {
        Some(OptionValue::Text(sep)) => sep,
        _ => ":",
    };

    let mut iter = operands.iter();
    let command: &str = match iter.next() {
        Some(cmd) => cmd,
        None => error!("no command provided"),
    };

    let client_configs = match read_json_config(config_file_path) {
        Ok(configs) => configs,
        Err(e) => error!(3, e),
    };
    let client_parser = ArgParser::new(client_configs);
    let client_args: Vec<&String> = iter.collect();

    match command {
        "parse" => {
            if let Err(e) = create_dir_all(state_dir) { error!(4, e) };
            let mut file = match File::create(&state_file) {
                Ok(file) => file,
                Err(e) => error!(5, e),
            };
            if let Err(e) = save_state(&client_parser, client_args, &mut file) {
                error!(6, e)
            };
            println!("parsed values saved to file: {:?}", state_file);
        },
        "json" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            let json: String = match serde_json::to_string_pretty(&state) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            println!("{}", json);
        },
        "operands" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            if let JsonValue::Object(obj) = state
            && let Some(JsonValue::Array(list)) = obj.get("operands") {
                for operand in list {
                    if let JsonValue::String(arg) = operand {
                        println!("{}", arg);
                    }
                }
            }
        },
        "options" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            if let JsonValue::Object(obj) = state
            && let Some(JsonValue::Object(map)) = obj.get("options") {
                for (name, value) in map {
                    if has_whitespace(name) {
                        print!("{:?}", name);
                    } else {
                        print!("{}", name);
                    }
                    print!("=");
                    match value {
                        JsonValue::Bool(_) => { println!("true"); },
                        JsonValue::Number(num) => { println!("{}", num); },
                        JsonValue::String(text) => {
                            if has_whitespace(text) {
                                println!("{:?}", text);
                            } else {
                                println!("{}", text);
                            }
                        },
                        JsonValue::Array(list) => {
                            let mut use_quotes = false;
                            let mut output = String::with_capacity(1024);
                            let mut iter = list.iter().peekable();
                            while let Some(JsonValue::String(arg)) = iter.next() {
                                if has_whitespace(arg) {
                                    use_quotes = true;
                                }
                                output.push_str(arg);
                                if iter.peek().is_some() {
                                    output.push_str(sep);
                                }
                            }
                            if use_quotes {
                                println!("{:?}", output);
                            } else {
                                println!("{}", output);
                            }
                        },
                        _ => {},
                    }
                }
            }
        },
        "parameters" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            if let JsonValue::Object(obj) = state
            && let Some(JsonValue::Object(map)) = obj.get("parameters") {
                for (name, value) in map {
                    if has_whitespace(name) {
                        print!("{:?}", name);
                    } else {
                        print!("{}", name);
                    }
                    print!("=");
                    let text_value: &str = match value {
                        JsonValue::String(text) => text,
                        _ => {
                            error!(
                                1,
                                "value {:?} of parameter {} is not a string",
                                value,
                                name
                            );
                        },
                    };
                    if has_whitespace(text_value) {
                        println!("{:?}", text_value);
                    } else {
                        println!("{}", text_value);
                    }
                }
            }
        },
        "operand" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            let Some(index_arg) = client_args.first() else {
                error!("no index was provided");
            };
            let Ok(index) = index_arg.parse::<usize>() else {
                error!("index arg cannot be parsed into an integer");
            };
            if let JsonValue::Object(obj) = state
            && let Some(JsonValue::Array(list)) = obj.get("operands")
            && let Some(JsonValue::String(arg)) = list.get(index) {
                println!("{}", arg);
            }
        },
        "option" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            let Some(key) = client_args.first() else {
                error!("no option name was provided");
            };
            if let JsonValue::Object(obj) = state
            && let Some(JsonValue::Object(map)) = obj.get("options") {
                let name: &str = key.as_ref();
                match map.get(name) {
                    Some(JsonValue::Bool(_)) => { println!("true"); },
                    Some(JsonValue::Number(num)) => { println!("{}", num); },
                    Some(JsonValue::String(text)) => {
                        println!("{}", text);
                    },
                    Some(JsonValue::Array(list)) => {
                        let mut output = String::with_capacity(1024);
                        let mut iter = list.iter().peekable();
                        while let Some(JsonValue::String(arg)) = iter.next() {
                            output.push_str(arg);
                            if iter.peek().is_some() {
                                output.push_str(sep);
                            }
                        }
                        println!("{}", output);
                    },
                    _ => {},
                }
            }
        },
        "parameter" => {
            let state = match read_state(&state_file) {
                Ok(data) => data,
                Err(e) => error!(e),
            };
            let Some(key) = client_args.first() else {
                error!("no parameter name was provided");
            };
            if let JsonValue::Object(obj) = state
            && let Some(JsonValue::Object(map)) = obj.get("parameters") {
                let name: &str = key.as_ref();
                if let Some(JsonValue::String(value)) = map.get(name) {
                    println!("{}", value);
                }
            }
        },
        cmd => {
            error!(8, "command {:?} is not supported", cmd);
        },
    };

    ExitCode::SUCCESS
}
