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

pub enum Value {
    String(String),
    Bool(bool),
    None
}

#[derive(Debug)]
pub enum Optional<T> where T: FromStr {
    Some(T),
    None
}

impl<T> FromStr for Optional<T> where T: FromStr {
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

#[derive(Debug)]
struct Argument {
    pub keys: Vec<String>,
    pub description: String,
    pub optional: bool,
    pub position: Option<usize>,
}

impl Argument {
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

        impl $struct_name {
            pub(crate) fn get_arguments() -> Vec<Argument> {
                let mut result: Vec<Argument> = Vec::new();
                $(
                
                    let mut cli_names: Vec<String> = Vec::new();
                    $( cli_names.push($cli_name.to_string()); )*

                    let mut description = String::new();
                    $( description.push_str($description); )*

                    let mut position: Option<usize> = None;
                    $( position = Some($cli_position); )*

                    let mut optional = false;

                    let a = Argument {
                        keys: cli_names,
                        description: description.to_string(),
                        optional,
                        position
                    };

                    result.push(a);
                )*
             
                result
            }

            // pub fn show_help(options: Option<HelpOptions>) {    
            //     let options = options.unwrap_or(HelpOptions::default());

            //     let field_info_array = Self::field_info();
            //     let longest_name = field_info_array.iter().map(|s| s.joined_name.len()).max().unwrap_or(0);
    
            //     print!("Usage: {}", $executable_name);
                
            //     // TODO: Add argument values to usage
            //     for field_info in &field_info_array {
            //         if field_info.optional {
            //             print!(" [{}]", field_info.joined_name);
            //         } else {
            //             print!(" <{}>", field_info.joined_name);
            //         }
            //     }

            //     print!("\n");

            //     let description_offset_no_name = options.indent_length + longest_name + options.description_offset;

            //     for field_info in &field_info_array {
            //         print!("{}{:<width$}", 
            //             String::from(" ").repeat(options.indent_length), 
            //             field_info.joined_name, 
            //             width = (longest_name + options.description_offset
            //             ));
    
            //         // Split the description into multiple lines if it's too long
            //         let mut desc_split: Vec<String> = Vec::new();
    
            //         let mut too_long = false;
            //         for (i, ch) in field_info.description.chars().enumerate() {
            //             if i % options.description_max_length == 0 {
            //                 too_long = true;
            //             }
            //             if (too_long
            //                 && options.split_at.matches(ch))
            //                 || i == 0 {
            //                 desc_split.push(String::new());
            //                 too_long = false;
            //             }
            //             desc_split.last_mut().unwrap().push(ch);
            //         }
    
            //         // Print description parts
            //         let mut desc_split_iter = desc_split.iter();
            //         if let Some(d1) = desc_split_iter.nth(0) {
            //             print!("{}\n", d1);
            //         } else {
            //             print!("\n");
            //         }
            //         for d in desc_split_iter.into_iter() {
            //             let padding = String::from(" ").repeat((description_offset_no_name - 1) + options.description_newline_extra_padding);
            //             print!("{}{}\n", padding, d);
            //         }
            //     }
            // }
            
            // fn cast_value<T: FromStr>(value: &str) -> Result<T, Box<dyn Error>> {
            //     match value.parse::<T>() {
            //         Ok(value) => Ok(value),
            //         Err(_) => Err(format!("Invalid value for argument: {}", value).into())
            //     }
            // }

            // pub fn get(key: &str) -> Value {
            //     let args: Vec<String> = std::env::args().collect();
            //     let mut args = args.into_iter();
                
            //     // Skip executable name
            //     args.next();

            //     for arg in args {
            //         if arg == key {
            //             return Value::Bool(true);
            //         } else if let Some((arg_key, arg_value)) = arg.split_once("=") {
            //             if arg_key == key {
            //                 return Value::String(arg_value.to_string());
            //             }
            //         }
            //     }

            //     Value::None
            // }

            // pub fn parse_custom(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
            //     let args: Vec<String> = args.into_iter().skip(1).collect();

            //     let result = Self {
            //         $(
            //             $name: {
            //                 let mut value: Box<dyn Any> = Box::new(None::<$cast>);

            //                 // Set default value
            //                 $(
            //                 match stringify!($default) {
            //                     "None" => {
            //                         // bool should default to false since it's set to true if the argument is present
            //                         value = Box::new(<$cast>::default());
            //                     },
            //                     _ => {
            //                         value = Box::new($default);
            //                     }
            //                 }
            //                 )*

            //                 let args = args.clone();
                            
            //                 // Check positional arguments
            //                 $(
            //                 if let Some(arg) = args.get($cli_position) {
            //                     value = Box::new(Self::cast_value::<$cast>(arg)?);
            //                 }
            //                 )?

            //                 // Check keyword arguments
            //                 for arg in args.iter() {
            //                     if !arg.starts_with("-") {
            //                         continue;
            //                     }

            //                     $(
            //                     if arg == $cli_name {
            //                         value = Box::new(true);
            //                     } else if let Some((arg_key, arg_value)) = arg.split_once("=") {
            //                         if arg_key == $cli_name {
            //                             value = Box::new(Self::cast_value::<$cast>(arg_value)?);
            //                         }
            //                     }
            //                     )?
            //                 }

            //                 // if value.is::<Option<$cast>>() {
            //                 //     return Err(format!("Missing required argument").into());
            //                 // }

            //                 // Downcast value to the correct type
            //                 match value.downcast::<$cast>() {
            //                     Ok(value) => *value,
            //                     Err(_) => return Err(format!("Invalid value for argument {}", stringify!($name)).into()) 
            //                 }
            //             },
            //         )*
            //     };

            //     Ok(result)
            // }

            // pub fn parse() -> Result<Self, Box<dyn Error>> {
            //     let args: Vec<String> = std::env::args().collect();
            //     Self::parse_custom(args)
            // }
        }
    };
}
