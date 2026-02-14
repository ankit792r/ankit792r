use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadBlogParams {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditBlogParams {
    pub id: String,
}
