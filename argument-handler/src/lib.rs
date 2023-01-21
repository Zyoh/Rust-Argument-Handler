// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

pub enum SplitAt {
    Any,
    Space,
    None
}

impl SplitAt {
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

#[macro_export]
macro_rules! config_setup {
    (
        $(#[$attr:meta])*
        $public_flag:vis struct $struct_name:ident for $executable_name:literal;

        $($name:ident: $cast:ty $(= $default:expr)?; [$cli_name:literal $(, $cli_position:literal)?] $($description:literal)?)+
    ) => {
        $(#[$attr])*
        $public_flag struct $struct_name {
            $(
                pub $name: $cast,
            )*
        }

        impl $struct_name {
            pub fn show_help(options: Option<HelpOptions>) {
                let mut help_parts: Vec<String> = Vec::new();
                let mut cli_names: Vec<String> = Vec::new();
    
                let options = options.unwrap_or(HelpOptions::default());

                let description_offset = options.description_offset;
                let description_max_length = options.description_max_length;
                let split_at = options.split_at;
                let description_newline_extra_padding = options.description_newline_extra_padding;
                let indent_length = options.indent_length;
                
                let indent = String::from(" ").repeat(indent_length);
    
                $(
                    #[allow(unused_variables)]
                    let desc = "";
                    $(let desc = $description;)? 
                    help_parts.push(String::from(desc)); 
                )*
                $( cli_names.push(String::from($cli_name)); )*
    
                let longest_name = cli_names.iter().map(|s| s.len()).max().unwrap_or(0);
    
                println!("Usage: {}", $executable_name);
                for (name, desc) in cli_names.iter().zip(help_parts.iter()) {
                    let description_offset_no_name = indent_length + longest_name + description_offset;
                    print!("{}{:<width$}", indent, name, width = (longest_name + description_offset));
    
                    // Split the description into multiple lines if it's too long
                    let mut desc_split: Vec<String> = Vec::new();
    
                    let mut too_long = false;
                    for (i, ch) in desc.chars().enumerate() {
                        if i % description_max_length == 0 {
                            too_long = true;
                        }
                        if (too_long
                            && split_at.matches(ch))
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
                        let padding = String::from(" ").repeat((description_offset_no_name - 1) + description_newline_extra_padding);
                        print!("{}{}\n", padding, d);
                    }
                }
            }
        }

        trait IsOptional {
            fn is_optional(&self) -> bool {
                true
            }
        }

        trait IsNotOptional {
            fn is_optional(&self) -> bool {
                false
            }
        }

        // TODO: Make this work
        impl<T> IsOptional for Option<T> {}
        impl IsOptional for bool {}
        impl<T> IsNotOptional for T {}
    
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    config_setup! {
        #[derive(Debug)] // Meta attributes.
        // [pub] struct <NAME> for "<EXECUTABLE_NAME>";
        // Example:
        pub struct Config for "application.exe";
    
        //                        --- Optional ---               - Optional -  ---        Optional        ---
        // struct_field_name: type = default_value; ["cli_name", cli_position] "Description of the argument."
    
        // --- Examples ---
        input_file: PathBuf; ["package", 0]
        output_file: Option<PathBuf> = None; ["--output"] "Saves to this file. Defaults to `out.txt` in the input file's parent directory."
        verbose: bool = false; ["--verbose"] "Enables verbose logging."
        help: bool = false; ["--help"] "Shows this help message."
    }

    #[test]
    fn it_works() {
        let options = HelpOptions {
            description_offset: 8, // (Default: 8) Offset of the description starting from the longest argument name.
            description_max_length: 50, // (Default: 50) Maximum length of the description.
            split_at: SplitAt::Space, // (Default: Space) When to split the description. Can be Any, Space, or None.
            description_newline_extra_padding: 2, // (Default: 2) Extra padding for the description when it's on a new line.
            indent_length: 4 // (Default: 4) Indentation of the arguments.
        };
        Config::show_help(Some(options));
    }
}
