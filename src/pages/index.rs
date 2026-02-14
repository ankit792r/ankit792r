use leptos::prelude::*;
use crate::models::{Project, BlogPost};

#[component]
pub fn IndexPage() -> impl IntoView {
    let projects: Vec<Project> = vec![
        Project {
            id: "trailtype",
            title: "TrailType",
            year: "2025",
            description: "Modern design inspired by the great outdoors.",
            image_alt: "nature and person",
        },
        Project {
            id: "spark-sessions",
            title: "The Spark Sessions",
            year: "2024",
            description: "A podcast for creators chasing meaningful work.",
            image_alt: "podcast mic",
        },
        Project {
            id: "into-the-wild",
            title: "Into the Wild",
            year: "2023",
            description: "A raw journey through nature's extremes.",
            image_alt: "wild elephant",
        },
    ];

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
        <div class="space-y-20">
            <section class="space-y-6">
                <div>
                    <h1 class="text-5xl md:text-7xl font-bold mb-4">"Ankit Prajapati"</h1>
                    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2 mb-6">
                        // <p class="text-xl text-gray-600">"Founder - x64-Tech"</p>
                        <p class="text-gray-700">"📍 Mumbai"</p>
                    </div>
                </div>
                <div class="space-y-4 text-lg text-gray-700 leading-relaxed">
                    <p>
                        "Hi there! I'm an adventurer based in Mumbai, with a background in technology and innovation. "
                        "These days, I'm driven by a love for creativity and innovation, constantly exploring new ways "
                        "to connect ideas and build something meaningful."
                    </p>
                    <p>
                        "When I'm not immersed in my projects, you'll find me outdoors — scaling rock faces, hiking "
                        "scenic trails, and embracing the energy of nature. Life is all about climbing to new heights, "
                        "both literally and figuratively!"
                    </p>
                </div>
            </section>

            <section class="space-y-6">
                <div class="flex items-center justify-between">
                    <h2 class="text-3xl font-semibold">"Latest Projects"</h2>
                    <a href="/work" class="text-gray-600 hover:text-gray-900 transition-colors">
                        "View all"
                    </a>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    {projects.iter().map(|project| {
                        view! {
                            <div class="group cursor-pointer">
                                <div class="aspect-video bg-gray-100 rounded-lg mb-4 overflow-hidden">
                                    <div class="w-full h-full flex items-center justify-center text-gray-400">
                                        <span class="text-sm">{project.image_alt}</span>
                                    </div>
                                </div>
                                <div class="flex items-start justify-between mb-2">
                                    <h3 class="text-xl font-semibold">{project.title}</h3>
                                    <span class="text-sm text-gray-500">{project.year}</span>
                                </div>
                                <p class="text-gray-600">{project.description}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class="space-y-6">
                <div class="flex items-center justify-between">
                    <h2 class="text-3xl font-semibold">"Latest Posts"</h2>
                    <a href="/blogs" class="text-gray-600 hover:text-gray-900 transition-colors">
                        "View all"
                    </a>
                </div>
                <div class="space-y-0">
                    {posts.iter().map(|post| {
                        view! {
                            <a href="/blogs" class="block group cursor-pointer">
                                <div class="flex items-start justify-between py-4 border-b border-gray-200 last:border-0">
                                    <div class="flex-1">
                                        <h3 class="text-lg font-medium group-hover:text-gray-900 transition-colors">
                                            {post.title}
                                        </h3>
                                        <p class="text-gray-600 text-sm line-clamp-2">{post.description}</p>
                                    </div>
                                    <span class="text-sm text-gray-500 ml-4">{post.date}</span>
                                </div>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class="py-16 border-t border-gray-200">
                <div class="max-w-3xl mx-auto px-6">
                    <div class="flex flex-col md:flex-row items-center gap-8">
                        <div class="flex-1 w-full md:w-auto">
                            <div class="aspect-video bg-gray-100 rounded-lg overflow-hidden">
                                <div class="w-full h-full flex items-center justify-center text-gray-400">
                                    <span class="text-sm">"hiker in nature"</span>
                                </div>
                            </div>
                        </div>
                        <div class="flex-1">
                            <h2 class="text-2xl font-semibold mb-4">"Subscribe to my Newsletter"</h2>
                            <p class="text-gray-600 mb-6">
                                "Sign up to stay updated about my latest work and adventures. "
                                <em class="text-gray-500">"No Spam, No BS. Promise!"</em>
                            </p>
                            <form class="flex flex-col sm:flex-row gap-3">
                                <input
                                    type="email"
                                    placeholder="Enter your email"
                                    class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-900"
                                />
                                <button
                                    type="submit"
                                    class="px-6 py-2 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors"
                                >
                                    "Subscribe"
                                </button>
                            </form>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
