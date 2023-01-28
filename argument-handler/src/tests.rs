// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::*;

    config_setup! {
        in config;

        #[derive(Debug)] // Meta attributes.
        // [pub] struct <NAME> for "<EXECUTABLE_NAME>";
        // Example:
        pub struct Config for "application.exe";
    
        //                        --- Optional ---               - Optional -  ---        Optional        ---
        // struct_field_name: type = default_value; ["cli_name", cli_position] "Description of the argument."
    
        // --- Examples ---
        input_file: PathBuf, 
            ["input_file"; 0];
        output_file: Optional<PathBuf>, 
            ["output"; 1], 
            "Saves to this file. Defaults to `out.txt` in the input file's parent directory.";
        verbose: bool, ["-V", "--verbose"], 
            "Enables verbose logging.";
        help: bool, ["-h", "--help"], 
            "Shows this help message.";
        template: Optional<String> = "default_template_string", 
            ["-t", "--template"], 
            "The template to use.";

        // These are fields that are not exposed to the user / command line.
        @internal {
            is_valid: bool = false;
            path_loaded_from: PathBuf = PathBuf::from("");
        }

        // impl into struct
        @impl {
            // This is a custom function that is called after the arguments are parsed.
            // This example adds a validation function that checks if the arguments are valid.
            fn validate(&self) -> Result<(), String> {
                if self.input_file.is_file() {
                    Ok(())
                } else {
                    Err(format!("Input file `{}` does not exist.", self.input_file.display()))
                }
            }
        }
    }

    #[test]
    fn argument_info() {
        // This should always work given correct configuration.
        // TODO: Check if incorrect configuration unnoticed in compile-time is possible.
        let args = config::Config::get_arguments().unwrap();
        println!("{:#?}", args);
    }

    #[test]
    fn help_message() {
        let options = config::HelpOptions {
            description_offset: 8, // (Default: 8) Offset of the description starting from the longest argument name.
            description_max_length: 50, // (Default: 50) Maximum length of the description.
            split_at: config::SplitAt::Space, // (Default: Space) When to split the description. Can be Any, Space, or None.
            description_newline_extra_padding: 2, // (Default: 2) Extra padding for the description when it's on a new line.
            indent_length: 4 // (Default: 4) Indentation of the arguments.
        };
        config::Config::show_help(Some(options)).unwrap();
    }

    #[test]
    fn parsing() {
        let mut args = vec![
            "appname.exec",
            "/dev/null/input_file", // This is a required positional argument.
            // "/dev/null/output_file", // This is an optional positional argument.
            "-V", // This is a flag.
            "-h", // This is a flag.
            // "--template", "template_string" // This is an optional argument.
        ];
        let args: Vec<String> = args.iter_mut().map(|arg| arg.to_string()).collect();

        let config = config::Config::parse_custom(args).unwrap();
        println!("{:#?}", config);

        assert_eq!(config.input_file, PathBuf::from("/dev/null/input_file"));
        assert_eq!(config.output_file.to_option(), None);
        assert_eq!(config.verbose, true);
        assert_eq!(config.help, true);
        assert_eq!(config.template.to_option(), Some("default_template_string".to_string()));
    }
}
