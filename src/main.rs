mod cli;
// mod core;

fn main() {
 let matches =  cli::build_cli().get_matches();
 

 match  matches.subcommand_name() {
    Some("new") => print!("gnerated new project!"),
    Some("build") =>  println!("build subcommand added"), 
    Some("run") => println!("Runnig sub commands"),
    None => print!("No subcommand"),
        _ => unreachable!(), 
    }
}
