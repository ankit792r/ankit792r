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

                    <A href="/about">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "About"
                        </span>
                    </A>
                    <A href="/work">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "Work"
                        </span>
                    </A>
                    <A href="/blogs">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "Blog"
                        </span>
                    </A>
                    <A href="/contact">
                        <span class="text-gray-600 hover:text-gray-900 transition-colors">
                            "Contact"
                        </span>
                    </A>
                    </nav>
                </div>
            </div>
        </footer>
    }
}
