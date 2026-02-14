#[derive(Clone)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub year: &'static str,
    pub description: &'static str,
    pub image_alt: &'static str,
}

#[derive(Clone)]
pub struct BlogPost {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub date: &'static str,
}
