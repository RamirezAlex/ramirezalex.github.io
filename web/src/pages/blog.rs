use crate::components::bio::Bio;
use crate::components::layout::Layout;
use konfiguration::Konfiguration;
use leptos::*;
use leptos_router::use_params_map;
use serde::Deserialize;
use std::fs;
extern crate markdown;

#[derive(Debug, Deserialize)]
pub struct BlogPost {
    title: String,
    date: String,
}

#[component]
pub fn Blog() -> impl IntoView {
    let params = use_params_map();
    let slug = params().get("slug").cloned();

    let slug = match slug {
        Some(slug) => slug,
        None => return view! { <Layout>{"Blog Index"}</Layout> },
    };

    let file_path = format!("./content/blog/{}/index", slug);

    let handle_error = |e: &dyn std::error::Error| {
        leptos::logging::error!("Error: {}", e);
        return view! { <Layout>{"404 content not found!"}</Layout> };
    };

    let conf = match Konfiguration::from_file(format!("{}.toml", file_path)).parse::<BlogPost>() {
        Ok(conf) => conf,
        Err(e) => return handle_error(&e),
    };

    let content = match fs::read_to_string(format!("{}.md", file_path)) {
        Ok(content) => markdown::to_html(&content),
        Err(e) => return handle_error(&e),
    };

    view! {
        <Layout>
            <div id="content" class="page-blog">
                <article>
                    <section class="blog-headings">
                        <h1>
                            {conf.title}
                        </h1>
                        <p>
                            {conf.date}
                        </p>
                   </section>
                    <section class="blog-article">
                        <div inner_html=content />
                    </section>
                </article>
                <Bio />
                <nav>
                    <ul>
                        <li>
                            <a rel="prev" href="#">Previous</a>
                        </li>
                        <li>
                            <a rel="next" href="#">Next</a>
                        </li>
                    </ul>
                </nav>
            </div>
        </Layout>
    }
}
