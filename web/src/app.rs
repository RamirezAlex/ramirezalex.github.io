use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::blog::Blog;
use crate::pages::blog_index::BlogIndex;
use crate::pages::home::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/blog/:slug" view=Blog/>
                    <Route path="/blog" view=BlogIndex/>
                </Routes>
            </main>
        </Router>
    }
}
