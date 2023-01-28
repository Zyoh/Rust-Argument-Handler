// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::*;

    config_setup! {
        #[derive(Debug)] // Meta attributes.
        // [pub] struct <NAME> for "<EXECUTABLE_NAME>";
        // Example:
        pub struct Config for "application.exe";
    
        //                        --- Optional ---               - Optional -  ---        Optional        ---
        // struct_field_name: type = default_value; ["cli_name", cli_position] "Description of the argument."
    
        // --- Examples ---
        input_file: PathBuf; ["input_file"; 0]
        output_file: Optional<PathBuf>; ["output"; 1] "Saves to this file. Defaults to `out.txt` in the input file's parent directory."
        verbose: bool; ["-V", "--verbose"] "Enables verbose logging."
        help: bool; ["-h", "--help"] "Shows this help message."
        template: Optional<String> = "default_template_string" ; ["-t", "--template"] "The template to use."
    }

    #[test]
    fn argument_info() {
        // This should always work given correct configuration.
        // TODO: Check if incorrect configuration unnoticed in compile-time is possible.
        let args = Config::get_arguments().unwrap();
        println!("{:#?}", args);
    }

    #[test]
    fn help_message() {
        let options = HelpOptions {
            description_offset: 8, // (Default: 8) Offset of the description starting from the longest argument name.
            description_max_length: 50, // (Default: 50) Maximum length of the description.
            split_at: SplitAt::Space, // (Default: Space) When to split the description. Can be Any, Space, or None.
            description_newline_extra_padding: 2, // (Default: 2) Extra padding for the description when it's on a new line.
            indent_length: 4 // (Default: 4) Indentation of the arguments.
        };
        Config::show_help(Some(options)).unwrap();
    }

    #[test]
    fn parsing() {
        let mut args = vec![
            "appname.exec",
            "/dev/null/input_file", // This is a required positional argument.
            "/dev/null/output_file", // This is an optional positional argument.
            "-V", // This is a flag.
            "-h", // This is a flag.
            // "--template", "template_string" // This is an optional argument.
        ];
        let args: Vec<String> = args.iter_mut().map(|arg| arg.to_string()).collect();

        let config = Config::parse_custom(args);
        println!("{:#?}", config);
    }
}
