use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="border-b border-gray-200">
            <div class="max-w-3xl mx-auto px-6 py-4 flex justify-between items-center">
            <div class="flex items-center gap-4">
                <A href="/">
                    <span class="font-semibold hover:text-gray-900 transition-colors">
                        "AP"
                    </span>
                </A>
                <nav class="flex space-x-4 text-base font-medium">
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
                </nav>
            </div>
            <div class="flex items-center gap-2">
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
            </div>
            </div>
        </header>
    }
}
