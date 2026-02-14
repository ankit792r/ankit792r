use leptos::prelude::*;
use crate::models::BlogPost;

#[component]
pub fn PostCard(post: BlogPost) -> impl IntoView {
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
}
