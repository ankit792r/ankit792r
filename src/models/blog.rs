use crate::database::defs::{Identifiable, Tagged};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub image_alt: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl Identifiable for BlogPost {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Tagged for BlogPost {
    fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
    
    fn get_tags(&self) -> Vec<&str> {
        self.tags.iter().map(|s| s.as_str()).collect()
    }
}
