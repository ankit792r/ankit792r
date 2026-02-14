use crate::database::defs::{Identifiable, Tagged};

pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image_alt: String,
    pub url: String,
    pub github_url: String,
    pub tags: Vec<String>,
}

impl Identifiable for Project {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Tagged for Project {
    fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
    
    fn get_tags(&self) -> Vec<&str> {
        self.tags.iter().map(|s| s.as_str()).collect()
    }
}