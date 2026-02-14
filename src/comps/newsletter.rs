use leptos::prelude::*;

#[component]
pub fn Newsletter() -> impl IntoView {
    view! {
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
    }
}
