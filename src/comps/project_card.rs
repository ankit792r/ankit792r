use leptos::prelude::*;
use crate::model::Project;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
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
}
