use dialoguer::{theme::ColorfulTheme,Select,Input};
use dialoguer::console::Term;

pub fn new_with_prompt() {
   let project_name: String = Input::with_theme(&ColorfulTheme::default())
       .with_prompt("project name")
       .interact_text()
       .unwrap();


    println!("The project name is {}",project_name);

    let select_options = &["library project","binary project","blank project"];
    let project_type_selection = Select::with_theme(&ColorfulTheme::default())
        .items(&select_options[..])
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();


    if let Some(selection) = project_type_selection {
        println!("Hello {}",select_options[selection]);
    }else {
        println!("You didn't select anything!");
    } 
}
