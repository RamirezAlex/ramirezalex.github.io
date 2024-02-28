use crate::components::layout::Layout;
use crate::components::posts_list::PostsList;
use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Layout>
            <div id="content">
                <PostsList/>
            </div>
        </Layout>
    }
}
