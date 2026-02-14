//! Example usage of the ReferenceDatabase
//! 
//! This module demonstrates how to use the database with BlogPost and Project models

use crate::database::defs::{Database, TaggedDatabase};
use crate::database::reference::ReferenceDatabase;
use crate::models::blog::BlogPost;
use crate::models::project::Project;

/// Example: Using ReferenceDatabase with BlogPost
pub fn example_blog_database() {
    // Create a new database
    let mut blog_db: ReferenceDatabase<BlogPost> = ReferenceDatabase::new();
    
    // Insert a blog post
    let post = BlogPost {
        id: "post-1".to_string(),
        title: "My First Post".to_string(),
        description: "This is my first blog post".to_string(),
        date: "2025-01-01".to_string(),
        image_alt: "First post image".to_string(),
        content: "Full content here...".to_string(),
        tags: vec!["rust".to_string(), "programming".to_string()],
    };
    
    match blog_db.insert(post.clone()) {
        Ok(inserted) => println!("Inserted: {}", inserted.title),
        Err(e) => println!("Error inserting: {}", e),
    }
    
    // Get a post by ID
    match blog_db.get("post-1") {
        Ok(post) => println!("Found post: {}", post.title),
        Err(e) => println!("Error getting post: {}", e),
    }
    
    // Get all posts
    match blog_db.get_all(10, 0) {
        Ok(posts) => println!("Found {} posts", posts.len()),
        Err(e) => println!("Error getting posts: {}", e),
    }
    
    // Get posts by tag (using TaggedDatabase)
    match blog_db.get_all_by_tag("rust", 10, 0) {
        Ok(posts) => println!("Found {} posts with tag 'rust'", posts.len()),
        Err(e) => println!("Error getting posts by tag: {}", e),
    }
}

/// Example: Using ReferenceDatabase with Project
pub fn example_project_database() {
    let mut project_db: ReferenceDatabase<Project> = ReferenceDatabase::new();
    
    let project = Project {
        id: "proj-1".to_string(),
        name: "My Project".to_string(),
        description: "A cool project".to_string(),
        image_alt: "Project image".to_string(),
        url: "https://example.com".to_string(),
        github_url: "https://github.com/example".to_string(),
        tags: vec!["web".to_string(), "rust".to_string()],
    };
    
    match project_db.insert(project) {
        Ok(p) => println!("Inserted project: {}", p.name),
        Err(e) => println!("Error: {}", e),
    }
}

/// Example: Initialize database with existing data
pub fn example_with_data() {
    let initial_posts = vec![
        BlogPost {
            id: "1".to_string(),
            title: "Post 1".to_string(),
            description: "Description 1".to_string(),
            date: "2025-01-01".to_string(),
            image_alt: "Image 1".to_string(),
            content: "Content 1".to_string(),
            tags: vec!["tech".to_string()],
        },
        BlogPost {
            id: "2".to_string(),
            title: "Post 2".to_string(),
            description: "Description 2".to_string(),
            date: "2025-01-02".to_string(),
            image_alt: "Image 2".to_string(),
            content: "Content 2".to_string(),
            tags: vec!["rust".to_string()],
        },
    ];
    
    let db = ReferenceDatabase::with_data(initial_posts);
    println!("Database initialized with {} items", db.len());
}
