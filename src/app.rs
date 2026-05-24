use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::blog::Blog;
use crate::pages::home::Home;
use crate::pages::post::Post;
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
        <Stylesheet
            id="highlight-theme"
            href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css"
        />
        <Script
            defer="true"
            src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"
        />
        <Script>{r#"
            window.highlightBlogCode = function () {
                if (!window.hljs) return;

                document.querySelectorAll('.blog-article pre code').forEach(function (block) {
                    if (!block.dataset.highlighted) {
                        window.hljs.highlightElement(block);
                    }
                });
            };

            document.addEventListener('DOMContentLoaded', function () {
                window.highlightBlogCode();
            });
        "#}</Script>

        // sets the document title
        <Title text="Alex Ramirez | Software Engineer"/>

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
                    <Route path="/blog/:slug" view=Post/>
                    <Route path="/blog" view=Blog/>
                </Routes>
            </main>
        </Router>
    }
}
