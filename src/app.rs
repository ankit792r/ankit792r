use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::comps::{footer::Footer, navbar::Navbar};
use crate::pages::{blogs::BlogsPage, index::IndexPage, tools::ToolsPage, contact::ContactPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ankit792r.css"/>
        <Title text="Ankit Prajapati - Portfolio"/>
        <Meta name="description" content="Portfolio of Ankit Prajapati - Founder of x64-Tech. Exploring creativity, innovation, and building meaningful projects." />

        <Router>
            <Navbar />
            <main class="max-w-4xl md:max-w-3xl mx-auto px-6 py-16 min-h-screen">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=IndexPage ssr=leptos_router::SsrMode::Async/>
                    <Route path=StaticSegment("/contact") view=ContactPage ssr=leptos_router::SsrMode::Async />

                    <ParentRoute path=StaticSegment("/blogs") view=BlogsPage >
                        <Route path=StaticSegment("") view=NotFound />
                    </ParentRoute>

                    <ParentRoute path=StaticSegment("/work") view=ToolsPage >
                        <Route path=StaticSegment("") view=NotFound />
                    </ParentRoute>

                    <Route path=WildcardSegment("any") view=NotFound ssr=leptos_router::SsrMode::Async/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
