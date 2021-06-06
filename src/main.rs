mod cli;
mod core;

fn main() {
 let matches =  cli::build_cli().get_matches();
 

 match  matches.subcommand() {
      ("new",Some(new_sub_commands)) => {
       println!("new command parsed! {}",new_sub_commands.value_of("lib").unwrap())
      }
      ("build",Some(_build_sub_commands)) => {
        println!("build subcommand added")
      } 
      ("run",Some(_run_sub_commands)) => {
        println!("Runnig sub commands")
      }
     ("",None) =>core::new::new_with_prompt(),
        _ => unreachable!(), 
    }
}
