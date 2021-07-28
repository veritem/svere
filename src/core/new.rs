use dialoguer::{theme::ColorfulTheme,Select,Input};
use dialoguer::console::Term;
use std::{fs, panic};
use std::path::{PathBuf,Path};

struct FileBuilder{
 path: PathBuf,
 body: String
}

pub struct Project{
  root: PathBuf
}

impl FileBuilder {
    
    fn mkdir_p(path: &Path){
        fs::create_dir_all(path)
            .unwrap_or_else(|e| panic!("Failed to mkdir_p {:#?}:{:#?}",path.to_str(),e))
    }

    pub fn new(path: PathBuf,body: &str) -> FileBuilder {
        FileBuilder {
            path,
            body: body.to_string(),
        }
    }
    

    // fn dirname(&self) -> &Path {
    //    self.path.parent().unwrap()
    // }

fn mk(&self){
      //mkdir_p(self.dirname(&self));
       fs::write(&self.path, &self.body)
            .unwrap_or_else(|e| panic!("Could not create file {}:{}",self.path.display(),e))
    }
}


impl Project {
  pub fn root(&self) -> PathBuf {
     self.root.clone()
   }

  pub fn build_dir(&self) -> PathBuf {
    self.root.join("build")
  }

  pub fn bin(&self,b:&str) -> PathBuf {
   self.build_dir().join("debug").join(&format!("Failed to create bin"))
  }

}

pub fn new_with_prompt() {
   let project_name: String = Input::with_theme(&ColorfulTheme::default())
       .with_prompt("project name")
       .interact_text()
       .unwrap();

    let select_options = &["library project","binary project","blank project"];
    let project_type_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project type ")
        .items(&select_options[..])
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();


    if let Some(selection) = project_type_selection {
        println!("Hello {}",select_options[selection]);
    }else {
        println!("You didn't select anything!");
    } 

   // TODO: Generate project from here
   ProjectBuilder::new(Project.root().join(project_name));    
}
