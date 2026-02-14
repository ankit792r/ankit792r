use leptos::prelude::*;
use crate::models::BlogPost;

#[component]
pub fn BlogsPage() -> impl IntoView {
    let posts: Vec<BlogPost> = vec![
        BlogPost {
            id: "framer-template",
            title: "How I built my Framer template empire",
            date: "Jul 2025",
            description: "I built my Framer template empire",
        },
        BlogPost {
            id: "journey-10k",
            title: "My journey from $0 to $10K MRR",
            date: "Jun 2025",
            description: "My journey from $0 to $10K MRR",
        },
        BlogPost {
            id: "vibe-coding",
            title: "Is vibe coding here to stay?",
            date: "Jun 2025",
            description: "Is vibe coding here to stay?",
        },
        BlogPost {
            id: "dont-complain",
            title: "Don't complain, create!",
            date: "May 2025",
            description: "Don't complain, create!",
        },
    ];

    view! {
        <div class="space-y-8">
            <h1 class="text-4xl font-bold mb-6">"Blog"</h1>
            <div class="space-y-0">
                {posts.iter().map(|post| {
                    view! {
                        <a href="/blogs" class="block group">
                            <div class="flex items-start justify-between py-4 border-b border-gray-200 last:border-0">
                                <div class="flex-1">
                                    <h3 class="text-lg font-medium group-hover:text-gray-900 transition-colors">
                                        {post.title}
                                    </h3>
                                </div>
                                <span class="text-sm text-gray-500 ml-4">{post.date}</span>
                            </div>
                        </a>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
