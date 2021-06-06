use clap::{App, AppSettings, Arg, SubCommand, crate_authors, crate_description};


pub fn build_cli() -> App<'static,'static> {
    App::new("svere")
               .about(concat!(crate_description!(),"\n","A C/C++ build tool!"))
               .version("0.0")
                .author(crate_authors!())
           .settings(&[AppSettings::ColoredHelp])
            .subcommand(SubCommand::with_name("new")
                .about("Generates new project")
                .arg(Arg::with_name("projet_name")
                   .long("lib")
                   .takes_value(true)
                   .help("Generate a new flesh project")
                   .required(true))
                .arg(Arg::with_name("project_name")
                    .long("binary")
                    .takes_value(true)
                    .help("Generate a new binary binary project"))
                )
            .subcommand(SubCommand::with_name("build")
                .about("Compiles your project"))
            .subcommand(SubCommand::with_name("run")
                .about("Compiles and run all your project"))
}
