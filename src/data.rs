use std::collections::HashMap;

use crate::schema::*;
use lazy_static::lazy_static;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

fn get_projects_map() -> HashMap<String, Project> {
    let mut map = HashMap::new();
    for project in PROJECTS {
        map.insert(project.path.to_string(), project.clone());
    }
    map
}

lazy_static! {
    pub static ref PROJECTS_MAP: HashMap<String, Project> = get_projects_map();
}
