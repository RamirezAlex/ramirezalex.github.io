use crate::api::post::get_post;
use crate::components::bio::Bio;
use crate::components::layout::Layout;
use leptos::*;
use leptos_router::use_params_map;
extern crate markdown;

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let slug = params().get("slug").cloned();

    let slug = match slug {
        Some(slug) => slug,
        None => return view! { <Layout>{"Blog Index"}</Layout> },
    };

    let handle_error = |_: &dyn std::error::Error| {
        return view! { <Layout>{"404 content not found!"}</Layout> };
    };

    let post = get_post(&slug).expect("Failed to get post");

    let content = markdown::to_html(&post.content);

    view! {
        <Layout>
            <div id="content" class="page-blog">
                <article>
                    <section class="blog-headings">
                        <h1>
                            {post.meta.title}
                        </h1>
                        <p>
                            {post.meta.date}
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
