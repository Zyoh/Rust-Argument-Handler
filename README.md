# Rust-Argument-Handler
 
A command line argument handler for Rust.

# Usage

```rust
// Used to specify optional arguments.
use argument_handler::Optional;

config_setup! {
    #[derive(Debug)] // Meta attributes.
    // [pub] struct <NAME> for "<EXECUTABLE_NAME>";
    // Example:
    pub struct Config for "application.exe";

    //                        --- Optional ---               - Optional -  ---        Optional        ---
    // struct_field_name: type = default_value; ["cli_name", cli_position] "Description of the argument."

    // --- Examples ---
    input_file: PathBuf; ["input_file"; 0]
    // Use argument_handler::Optional instead of Option!
    output_file: Optional<PathBuf>; ["output"; 1] "Saves to this file. Defaults to `out.txt` in the input file's parent directory."
    verbose: bool; ["-V", "--verbose"] "Enables verbose logging."
    help: bool; ["-h", "--help"] "Shows this help message."
    template: Optional<String> = "default_template_string"; ["-t", "--template"] "The template to use."
}

// Parse command line arguments. You can use parse_custom() to give your own argument array.
let config = Config::parse().unwrap();

// Access fields like a normnal struct
config.input_file

// Optional arguments need to be converted to Options
config.template.to_option()
```

# License

<!-- TODO -->
To be added.
