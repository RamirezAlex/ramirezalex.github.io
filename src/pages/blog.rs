use crate::components::layout::Layout;
use crate::components::posts_list::PostsList;
use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Layout>
            <div id="content" class="page-blog-list">
                <section class="blog-index-header">
                    <p class="section-eyebrow">
                        <span class="eyebrow-dot eyebrow-dot-purple"></span>
                        "Writing"
                    </p>
                    <h1>"Blog"</h1>
                </section>
                <PostsList/>
            </div>
        </Layout>
    }
}
