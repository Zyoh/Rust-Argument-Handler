// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

// Don't warn unused code since macros aren't detected as used.
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod tests;

use std::error::Error;
use std::str::FromStr;
use std::any::Any;
use std::collections::HashMap;

pub enum SplitAt {
    Any,
    Space,
    None
}

impl SplitAt {
    #[allow(dead_code)]
    fn matches(&self, character: char) -> bool {
        match self {
            SplitAt::Any => true,
            SplitAt::Space => character == ' ',
            SplitAt::None => false
        }
    }
}

pub struct HelpOptions {
    pub description_offset: usize,
    pub description_max_length: usize,
    pub split_at: SplitAt,
    pub description_newline_extra_padding: usize,
    pub indent_length: usize
}

impl Default for HelpOptions {
    fn default() -> Self {
        Self {
            description_offset: 8,
            description_max_length: 50,
            split_at: SplitAt::Space,
            description_newline_extra_padding: 2,
            indent_length: 4
        }
    }
}

pub enum CLIValue {
    String(String),
    Bool(bool),
    None
}

#[derive(Debug, Copy, Clone)]
pub enum Optional<T> where T: FromStr + Default {
    Some(T),
    None
}

impl<T> Optional<T> where T: FromStr + Default + Clone {
    pub fn to_option(&self) -> Option<T> {
        match self {
            Optional::Some(value) => Some(value.clone()),
            Optional::None => None
        }
    }
}

impl<T> FromStr for Optional<T> where T: FromStr + Default {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let object: Result<Optional<T>, Self::Err> = match T::from_str(s) {
            Ok(value) => Ok(Optional::Some(value)),
            Err(_) => {
                if s == "None" {
                    Ok(Optional::None)
                } else {
                    Err("Invalid value for Optional.".into())
                }
            }
        };
        object
    }
}

impl<T> Default for Optional<T> where T: FromStr + Default {
    fn default() -> Self {
        // let object: Optional<T> = Optional::Some(T::default());
        let object = Self::None;
        object
    }
}

#[derive(Debug)]
struct Argument<T> {
    pub field_name: String,
    pub keys: Vec<String>,
    pub description: String,
    pub optional: bool,
    pub position: Option<usize>,
    pub value: T
}

impl<T> Argument<T> {
    pub fn matches_key(&self, key: &str) -> bool {
        self.keys.iter().any(|k| k == key)
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.keys == other.keys
    }

    pub fn pretty_name(&self) -> String {
        self.keys.join(", ")
    }
}

#[macro_export]
macro_rules! config_setup {
    (
        $(#[$attr:meta])*
        $public_flag:vis struct $struct_name:ident for $executable_name:literal;

        // TODO: Don't allow `-` or `--` prefix when position is specified
        // TODO: Don't allow multiple argument names when position is specified
        $($name:ident: $cast:ty $(= $default:expr)?; [$($cli_name:literal),+ $(; $cli_position:literal)?] $($description:literal)?)+
    ) => {
        $(#[$attr])*
        $public_flag struct $struct_name {
            $(
                pub $name: $cast,
            )*
        }

        #[derive(Debug, Clone)]
        pub enum ArgumentType {
            $(
                #[allow(non_camel_case_types)]
                $name($cast),
            )*

            None
        }

        impl $struct_name {
            pub(crate) fn get_arguments() -> Result<
                                                HashMap<String, Argument<ArgumentType>>, 
                                                Box<dyn Error>
                                            > {
                let mut result: HashMap<String, Argument<ArgumentType>> = HashMap::new();

                $(                
                    let mut cli_names: Vec<String> = Vec::new();
                    $( cli_names.push($cli_name.to_string()); )*

                    #[allow(unused_mut)]
                    let mut description = String::new();
                    $( description.push_str($description); )*

                    let position: Option<usize> = None;
                    $( let position = Some($cli_position); )*

                    let optional = stringify!($cast).starts_with("Optional");

                    let field_name = stringify!($name).to_string();

                    // Default value
                    let default: Option<String> = None;
                    $( let default = Some($default.to_string()); )*

                    let actual_default = match default {
                        Some(default) => ArgumentType::$name(<$cast>::from_str(&default)?),
                        None => ArgumentType::None
                    };

                    let argument = Argument::<ArgumentType> {
                        field_name,
                        keys: cli_names,
                        description: description.to_string(),
                        optional,
                        position,
                        value: actual_default
                    };

                    result.insert(argument.field_name.clone(), argument);
                )*

                return Ok(result);
            }

            pub fn show_help(options: Option<HelpOptions>) -> Result<(), Box<dyn Error>> {    
                let options = options.unwrap_or(HelpOptions::default());

                let arguments = Self::get_arguments()?;
                let longest_name: usize = arguments.values().into_iter().map(
                    |s| {
                        s.pretty_name().len()
                    }).max().unwrap_or(0);
    
                print!("Usage: {}", $executable_name);
                
                // TODO: Add argument values to usage
                for argument in arguments.values().into_iter() {
                    if argument.optional {
                        print!(" [{}]", argument.pretty_name());
                    } else {
                        print!(" <{}>", argument.pretty_name());
                    }
                }

                print!("\n");

                let description_offset_no_name = options.indent_length + longest_name + options.description_offset;

                // Sort arguments by position
                let mut arguments_vector: Vec<(&String, &Argument<ArgumentType>)> = arguments.iter().collect();
                arguments_vector.sort_by(|(_, v), (_, v2)| {
                    let pos = v.position.unwrap_or(usize::MAX);
                    let pos2 = v2.position.unwrap_or(usize::MAX);
                    pos.cmp(&pos2)
                });
                for (field_name, argument) in arguments_vector {
                    print!("{}{:<width$}", 
                        String::from(" ").repeat(options.indent_length), 
                        argument.pretty_name(), 
                        width = (longest_name + options.description_offset
                        ));
    
                    // Split the description into multiple lines if it's too long
                    let mut desc_split: Vec<String> = Vec::new();
    
                    let mut too_long = false;
                    for (i, ch) in argument.description.chars().enumerate() {
                        if i % options.description_max_length == 0 {
                            too_long = true;
                        }
                        if (too_long
                            && options.split_at.matches(ch))
                            || i == 0 {
                            desc_split.push(String::new());
                            too_long = false;
                        }
                        desc_split.last_mut().unwrap().push(ch);
                    }
    
                    // Print description parts
                    let mut desc_split_iter = desc_split.iter();
                    if let Some(d1) = desc_split_iter.nth(0) {
                        print!("{}\n", d1);
                    } else {
                        print!("\n");
                    }
                    for d in desc_split_iter.into_iter() {
                        let padding = String::from(" ").repeat((description_offset_no_name - 1) + options.description_newline_extra_padding);
                        print!("{}{}\n", padding, d);
                    }
                }

                Ok(())
            }
            
            fn cast_value<T: FromStr>(value: &str) -> Result<T, Box<dyn Error>> {
                match value.parse::<T>() {
                    Ok(value) => Ok(value),
                    Err(_) => Err(format!("Invalid value for argument: {}", value).into())
                }
            }

            pub fn get(key: &str) -> CLIValue {
                let args: Vec<String> = std::env::args().collect();
                let mut args = args.into_iter();
                
                // Skip executable name
                args.next();

                for arg in args {
                    if arg == key {
                        return CLIValue::Bool(true);
                    } else if let Some((arg_key, arg_value)) = arg.split_once("=") {
                        if arg_key == key {
                            return CLIValue::String(arg_value.to_string());
                        }
                    }
                }

                CLIValue::None
            }

            pub fn parse_custom(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
                let cliargs: Vec<String> = args.into_iter().skip(1).collect();

                let setup_arguments = Self::get_arguments()?;

                let result = Self {
                    $(
                    $name: {
                        // Unwrap here should be safe
                        let setup_arg = setup_arguments.get(stringify!($name)).unwrap();
                        
                        #[allow(unused_assignments)]
                        let mut value = ArgumentType::None;

                        let cliargs = cliargs.clone();
                        
                        // Check positional arguments
                        $(
                        if let Some(arg) = cliargs.get($cli_position) {
                            if arg.starts_with("-") {
                                if !setup_arg.optional {
                                    return Err(format!("Expected required positional argument `{}`, found keyword argument `{}`.", 
                                        stringify!($name),
                                        arg
                                    ).into());   
                                }
                            } else {
                                value = ArgumentType::$name(Self::cast_value::<$cast>(arg)?);
                            }
                        }
                        )?

                        // Check keyword arguments
                        for carg in cliargs.iter() {
                            if !carg.starts_with("-") {
                                continue;
                            }

                            $(
                            if carg == $cli_name {
                                // Cast value to bool
                                value = ArgumentType::$name(Self::cast_value::<$cast>("true")?);
                            } else if let Some((arg_key, arg_value)) = carg.split_once("=") {
                                if arg_key == $cli_name {
                                    value = ArgumentType::$name(Self::cast_value::<$cast>(arg_value)?);
                                }
                            }
                            )?
                        }

                        // --------------------

                        let mut result: $cast = <$cast>::default();

                        // Default from setup
                        let default: ArgumentType = setup_arg.value.clone();
                        if let ArgumentType::$name(v) = default {
                            result = v;
                        }

                        if let ArgumentType::$name(v) = value {
                            result = v;
                        }

                        result
                    },
                    )*
                };

                Ok(result)
            }

            pub fn parse() -> Result<Self, Box<dyn Error>> {
                let args: Vec<String> = std::env::args().collect();
                Self::parse_custom(args)
            }
        }
    };
}
