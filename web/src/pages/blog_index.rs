use crate::components::articles::Articles;
use crate::components::bio::Bio;
use crate::components::layout::Layout;
use konfiguration::Konfiguration;
use leptos::prelude::*;
use leptos::*;
use leptos_router::use_params_map;
use std::fs;

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
