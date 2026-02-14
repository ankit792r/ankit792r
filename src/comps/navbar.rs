use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
      <header class="">
    <div class="max-w-5xl mx-auto px-6 py-4 flex justify-between items-center">
      <nav class="flex space-x-8 text-lg font-medium">
        <a href="#about" class="hover:text-blue-600">"About"</a>
        <a href="#work" class="hover:text-blue-600">"Work"</a>
        <a href="#blog" class="hover:text-blue-600">"Blog"</a>
      </nav>
    </div>
  </header>}
}
