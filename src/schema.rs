#[derive(Clone)]
pub struct Project {
    pub name: &'static str,
    pub thumbnail: &'static str,
    pub readme: &'static str,
    pub technologies: &'static [&'static str],
    pub repo: Option<&'static str>,
    pub path: &'static str,
}

pub struct Skill {
    pub name: &'static str,
    pub content: &'static str,
}

pub struct Technology {
    pub name: &'static str,
    pub content: &'static str,
}

pub struct About {
    pub skills: &'static [Skill],
    pub technologies: &'static [Technology],
    pub about_me: &'static str,
}

pub struct Contact {
    pub html: &'static str,
} 
