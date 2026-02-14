use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-4xl font-bold mb-6">"About"</h1>
            <div class="space-y-6 text-lg text-gray-700 leading-relaxed">
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
        </div>
    }
}
