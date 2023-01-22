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
        input_file: PathBuf; ["input_file"]
        output_file: Optional<PathBuf>; ["-o", "--output"] "Saves to this file. Defaults to `out.txt` in the input file's parent directory."
        verbose: bool; ["-V", "--verbose"] "Enables verbose logging."
        help: bool; ["-h", "--help"] "Shows this help message."
        template: Optional<String>; ["-t", "--template"] "The template to use."
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
        Config::show_help(Some(options));
    }

    #[test]
    fn values() {
        let mut args = vec![
            "application.exec",
            "/dev/null/input_file",
            // "-o", "/dev/null/output_file",
            "-V",
            "-h"
        ];
        let args: Vec<String> = args.iter_mut().map(|arg| arg.to_string()).collect();

        let config = Config::parse_custom(args);
        println!("{:#?}", config);
        println!("{:#?}", config.unwrap().input_file);
    }
}
