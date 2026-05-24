use crate::api::post::get_post;
use crate::components::bio::Bio;
use crate::components::layout::Layout;
use leptos::*;
use leptos_router::{use_params_map, A};
#[cfg(feature = "hydrate")]
use std::time::Duration;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
export function highlight_blog_code() {
  if (window.highlightBlogCode) {
    window.highlightBlogCode();
  }
}
"#)]
extern "C" {
    fn highlight_blog_code();
}

#[component]
pub fn Post() -> impl IntoView {
    let params = use_params_map();
    let slug = params().get("slug").cloned();

    let slug = match slug {
        Some(slug) => slug,
        None => return view! { <Layout>{"Blog Index"}</Layout> },
    };

    let post = create_resource(move || slug.clone(), |slug| async move { get_post(slug).await });

    #[cfg(feature = "hydrate")]
    create_effect(move |_| {
        if post.get().is_some() {
            set_timeout(highlight_blog_code, Duration::from_millis(0));
        }
    });

    view! {
        <Layout>
            <div id="content" class="page-blog">
                <Suspense fallback=move || view! { <article>"Loading post..."</article> }>
                    {move || {
                        post.get().map(|post| {
                            match post {
                                Ok(post) => {
                                    let content = markdown::to_html(&post.content);
                                    let title = post.meta.title.clone();
                                    let date = post.meta.date.clone();

                                    view! {
                                        <article class="blog-post">
                                            <section class="blog-headings">
                                                <A class="blog-back-link" href="/blog">"Back to blog"</A>
                                                <h1>{title}</h1>
                                                <time>{date}</time>
                                           </section>
                                            <section class="blog-article">
                                                <div inner_html=content />
                                            </section>
                                        </article>
                                        <Bio />
                                    }.into_view()
                                }
                                Err(_) => view! { <article class="blog-post">"404 content not found!"</article> }
                                    .into_view(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        </Layout>
    }
}
