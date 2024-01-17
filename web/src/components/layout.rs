use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::nav::Nav;

use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let (dark_mode, set_dark_mode) = create_signal(false);

    let mode_class = if dark_mode.get() {
        "dark-mode"
    } else {
        "light-mode"
    };

    leptos::logging::log!("mode_class: {}", mode_class);
    view! { <main class={mode_class}><Nav setter=set_dark_mode /> <div class="overflow-x-hidden bg-white dark:bg-black"><Header/>{children()} <Footer/></div></main> }
}
