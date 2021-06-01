
use clap::{App, Arg,crate_description};

#[derive(Debug,PartialEq)]
pub struct Args {
    pub run: bool,
    pub new: bool,
    pub build: bool
}

pub fn parse_args(){
    let app = App::new("svere")
        .about(concat!(crate_description!(),"\n","C/C++ build tool!"))
        .version("0.0")
        .arg(
            Arg::with_name("build")
            .help("Build the app")
            .short("b")
            .long("build")
        )
        .arg(
            Arg::with_name("new")
            .short("n")
            .help("Generate new application")
            .long("new")
            )
        .arg(
            Arg::with_name("run")
            .short("r")
            .long("run")
            .help("Compiles and run the app")
            )
        .get_matches();
}
