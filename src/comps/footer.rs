use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-gray-200 py-8">
            <div class="max-w-3xl mx-auto px-6">
                <div class="flex flex-col md:flex-row items-center justify-between gap-4">
                    <p class="text-gray-600">"© Ankit Prajapati | 2026"</p>
                    <nav class="flex space-x-6 text-sm">
                    <A href="https://x.com/ankit792r">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "Twitter"
                        </span>
                    </A>
                    <A href="https://github.com/ankit792r">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "Github"    
                        </span>
                    </A>
                    <A href="https://linkedin.com/in/ankit792r">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "LinkedIn"
                        </span>
                    </A>
                    <A href="https://reddit.com/u/ankit792r">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "Reddit"
                        </span>
                    </A>
                    <A href="https://youtube.com/@ankit792r">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "YouTube"
                        </span>
                    </A>
                    </nav>
                </div>
            </div>
        </footer>
    }
}
