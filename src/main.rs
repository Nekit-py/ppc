use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::fs;
use clap::{Arg, App};

extern crate clap;


fn check_base_path(base_path: &str) -> (String, bool) {
    //checking the existence of a path
    let exist: bool = Path::new(&base_path.trim()).exists();
    (base_path.trim().to_string(), exist)
}

fn check_project_name(ps: &str) -> String {
    //Checking the folder name for correctness
    ps.to_string().retain(|c| !r#"\ / : * ? " < > | "#.contains(c));
    let mut project_name = ps; 
    if project_name.len() == 0 {
        println!("The project name is set by default.");
        project_name = "ppc_new";
    }
    project_name.trim().to_string()
}

fn git_initialization(path: &str) -> Result<(), Box<dyn Error>> {
    println!("Initializing the git repository...");
    let _git_init = Command::new("git").current_dir(path).arg("init").status()?;
    Ok(())
}

fn create_venv(proj_path: &str, project_name: &str) -> Result<(), Box<dyn Error>> {
    //Creating a virtual environment and epmty git
    let proj_venv: &str = &[proj_path, "/venv"].join("");
    println!("Creating a virtual environment...");
    let _cvenv = Command::new("python3")
        .args(&["-m", "venv", proj_venv])
        .status()?;
    fs::create_dir([&proj_path, "/", "git"].join(""))
        .expect("Failed to create git folder...");
    let mut gitignore = File::create(&[proj_path, "/git/.gitignore"].join(""))?;
    gitignore.write_all(b"__pycache__")?;
    let mut py_file = File::create(&[proj_path, "/git/", project_name, ".py"].join(""))?;
    py_file.write_all(
        b"def main():\n\tprint('Hello, world!')\n\nif __name__ == '__main__':\n\tmain()",
    )?;
    Ok(())
}

fn main() {
    let matches = App::new("Python project creator")
        .version("0.2.0")
        .author("Nekit S. <nekit-sns@yandex.ru>")
        .about("Helps you quickly create a python project including a virtual environment and git repository initialization.")
        .arg(Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .takes_value(true)
                 .help("Absolute path to the folder where the project will be created."))
        .arg(Arg::with_name("name")
                 .short("n")
                 .long("name")
                 .takes_value(true)
                 .help("Project name."))
        .arg(Arg::with_name("git")
                 .short("g")
                 .long("git")
                 .takes_value(false)
                 .help("Git repository initialization."))
        .get_matches();
    let base_path = matches.value_of("path").expect("To create a project, the path must be specified");
    let ps = matches.value_of("name").unwrap();
    //let _git = matches.value_of("git");
    let (typing_path, input_path_exists) = check_base_path(&base_path);
    match input_path_exists {
        true => {
            let project_name = check_project_name(&ps);
            let proj_path = [typing_path.clone(), "/".to_string(), project_name.clone()].join("");
            fs::create_dir([&typing_path, "/", &project_name].join(""))
                    .expect("Failed to create project folder...");
            create_venv(&proj_path, &project_name)
                .expect("An error occurred while creating the virtual environment...");
            git_initialization(
                &[typing_path, "/".to_string(), project_name.clone(), "/git".to_string()].join(""),
                    )
                .expect("Error when initializing the git repository...");
            println!("Project '{}' created successfully.", project_name);
        }
        false => println!("Unable to create project, invalid path specified."),
    }
}
