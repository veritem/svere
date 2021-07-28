use clap::{App, Arg};


pub fn build_cli() -> App<'static> {
    // App::new("svere")
    //            .about(concat!(crate_description!(),"\n","A C/C++ build tool!"))
    //            .version("0.0")
    //             .author(crate_authors!())
    //         .subcommand(SubCommand::with_name("new")
    //             .about("Generates new project")
    //                .long("lib")
    //                .takes_value(true)
    //                .help("Generate a new flesh project")
    //                .required(true))
    //             .arg(Arg::with_name("project_name")
    //                 .long("binary")
    //                 .takes_value(true)
    //                 .help("Generate a new binary binary project"))
    //         .subcommand(SubCommand::with_name("build")
    //             .about("Compiles your project"))
    //         .subcommand(SubCommand::with_name("run")
    //             .about("Compiles and run all your project"))
    //         .get_matches();
    App::new("svere")
        .version("0.0.1")
      .subcommand(
        App::new("add")
        .about("generate new project")
        .arg(
           Arg::new("name")
           .about("project name")
           .index(1)
           .required(true)
        )
    )
}
