use crate::components::footer::Footer;
use crate::components::nav::Nav;

use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let (dark, set_dark) = create_signal(false);
    let class = move || if dark.get() { "layout dark" } else { "layout" };

    view! {
        <div class=class>
            <Nav dark=dark set_dark=set_dark />
            {children()}
            <Footer />
        </div>
    }
}
