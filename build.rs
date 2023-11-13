use std::fs;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub thumbnail: String,
    pub readme: String,
    pub technologies: Vec<String>,
    pub repo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Technology {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct About {
    pub skills: Vec<Skill>,
    pub technologies: Vec<Technology>,
    pub about_me: String,
}

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub html: String,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub projects: Vec<Project>,
    pub about: About,
    pub contact: Contact,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=content.yaml");

    let content_path = "content.yaml";
    let content = fs::read_to_string(content_path).unwrap();
    let content: Content = serde_yaml::from_str(&content).unwrap();

    let mut projects = Vec::new();
    for project in &content.projects {
        let name = &project.name;
        let thumbnail = &project.thumbnail;
        let readme_url = &project.readme;
        let technologies = &project.technologies;
        let readme = reqwest::get(readme_url).await?;
        if !readme.status().is_success() {
            panic!("Failed to fetch readme for project {}", name);
        }
        let readme = readme.text().await?;
        let repo = &project.repo;
        let repo = match repo {
            Some(repo) => quote::quote!{Some(#repo)},
            None => quote::quote!{None},
        };
        
        projects.push(quote::quote!{
            Project {
                name: #name,
                thumbnail: #thumbnail,
                readme: #readme,
                technologies: &[#(#technologies),*],
                repo: #repo,
            }
        });
    }

    let mut skills = Vec::new();
    for skill in &content.about.skills {
        let name = &skill.name;
        let content = &skill.content;
        skills.push(quote::quote!{
            Skill {
                name: #name,
                content: #content,
            }
        });
    }

    let mut technologies = Vec::new();
    for technology in &content.about.technologies {
        let name = &technology.name;
        let content = &technology.content;
        technologies.push(quote::quote!{
            Technology {
                name: #name,
                content: #content,
            }
        });
    }
    let about_me = &content.about.about_me;

    let contact_html = &content.contact.html;

    let code = quote::quote!{
        pub const PROJECTS: &'static [Project] = &[
            #(#projects),*
        ];
        pub const ABOUT: About = About {
            skills: &[
                #(#skills),*
            ],
            technologies: &[
                #(#technologies),*
            ],
            about_me: #about_me,
        };
        pub const CONTACT: Contact = Contact {
            html: #contact_html,
        };
    };

    let out_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut out_file = fs::File::create(&out_path).unwrap();

    write!(out_file, "{}", code).unwrap();

    Ok(())
}
