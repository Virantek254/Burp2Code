// Libs
use clap::{Arg, Command, value_parser};

pub struct CMD {
    pub req: String,
    pub code: String,
}

pub fn initialize() -> CMD {
    let app = Command::new("Burp2Code")
        .version("1.0")
        .author("Vartex")
        .about("A Burp Request to Code Converter")

        // BURP2CODE Options
        .arg(
            Arg::new("request")
                .short('r')
                .long("request")
                .required(true)
                .value_parser(value_parser!(String))
                .help_heading("BURP2CODE Options")
                .help("Set request file"),
        )
        .arg(
            Arg::new("code")
                .short('c')
                .long("code")
                .required(true)
                .value_parser(value_parser!(String))
                .help_heading("BURP2CODE Options")
                .help("Set programming Language"),
        )

        .get_matches();

    let mut req = String::new();
    let mut code = String::new();

    if let Some(value) = app.get_one::<String>("request") {
        req = String::from(value);
    }
    if let Some(value) = app.get_one::<String>("code") {
        code = String::from(value);
    }

    CMD {
        req,
        code,
    }
}