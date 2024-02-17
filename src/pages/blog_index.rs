use crate::components::articles::Articles;
use crate::components::layout::Layout;
use leptos::*;

#[component]
pub fn BlogIndex() -> impl IntoView {
    view! {
        <Layout>
            <div id="content">
                <Articles/>
            </div>
        </Layout>
    }
}
