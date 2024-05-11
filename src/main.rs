use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// type Project = {
///   name: string
///   status: 'active' | 'inactive' | 'archived'
///   focus: number
/// }

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Active,
    Inactive,
    Archived,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    status: Status,
    focus: u8, // 0-100
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
}

fn find_project_by_name<'a>(projects: &'a [Project], query: &str) -> Option<&'a Project> {
    projects.iter().find(|p| p.name == query)
}

fn read_projects_from_file(file_path: &str) -> Result<Vec<Project>> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON data into a Vec<Project>
    let projects = serde_json::from_reader(reader)?;
    Ok(projects)
}

fn write_to_json(projects: &[Project], path: &Path) -> Result<()> {
    let file = File::create(path)?; // `?` will automatically convert std::io::Error to anyhow::Error
    serde_json::to_writer(file, projects)?; // Same here for serde_json::Error
    Ok(())
}

fn write_projects_to_file(projects: &[Project], file_path: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, projects)?;
    Ok(())
}

fn update_or_add_project(projects: &mut Vec<Project>, new_project: Project) {
    match projects.iter_mut().find(|p| p.name == new_project.name) {
        Some(project) => {
            // Update the existing project
            *project = new_project;
        }
        None => {
            // Add new project since it does not exist
            projects.push(new_project);
        }
    }
}

fn main() -> Result<()> {
    let projects = vec![
        Project {
            name: "one".to_string(),
            status: Status::Active,
            focus: 100,
        },
        Project {
            name: "two".to_string(),
            status: Status::Archived,
            focus: 75,
        },
    ];

    let file_path = "projects.json";
    let path = Path::new(file_path);

    // WRITE
    write_to_json(&projects, path).expect("Failed to write to file");

    // FIND ONE
    let search_name = "two";
    match find_project_by_name(&projects, search_name) {
        Some(project) => println!(
            "Found project: {:?}, Status: {:?}, Focus: {}",
            project.name, project.status, project.focus
        ),
        None => println!("Project not found."),
    }

    // GET ALL
    match read_projects_from_file(file_path) {
        Ok(projects) => {
            for project in projects {
                println!(
                    "Project Name: {}, Status: {:?}, Focus: {}",
                    project.name, project.status, project.focus
                );
            }
        }
        Err(e) => {
            println!("Failed to read projects: {}", e);
        }
    }

    // UPDATE
    // Read the existing projects
    let mut projects = read_projects_from_file(file_path)?;

    // Create a new project or modify an existing project's data
    let updated_project = Project {
        name: "two".to_string(),
        status: Status::Active,
        focus: 33, // Updated focus or any other fields
    };

    // Update the project list
    update_or_add_project(&mut projects, updated_project);

    // Write the updated list back to the file
    write_projects_to_file(&projects, file_path)?;

    // GET ALL AGAIN
    let updated_projects = read_projects_from_file(file_path);
    println!("Updated Projects:");
    match updated_projects {
        Ok(updated_projects) => {
            for project in updated_projects {
                println!(
                    "Project Name: {}, Status: {:?}, Focus: {}",
                    project.name, project.status, project.focus
                );
            }
        }
        Err(e) => {
            println!("Failed to read projects: {}", e);
        }
    }

    Ok(())
}
