use leptos::prelude::*;
use crate::models::Project;

#[component]
pub fn ToolsPage() -> impl IntoView {
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

    view! {
        <div class="space-y-8">
            <h1 class="text-4xl font-bold mb-6">"Work"</h1>
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
        </div>
    }
}
