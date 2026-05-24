use crate::components::footer::Footer;
use crate::components::nav::Nav;

use leptos::*;

#[cfg(feature = "hydrate")]
const THEME_STORAGE_KEY: &str = "theme";

#[cfg(feature = "hydrate")]
fn initial_dark_mode() -> bool {
    let stored_theme = window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(THEME_STORAGE_KEY).ok().flatten());

    !matches!(stored_theme.as_deref(), Some("light"))
}

#[cfg(not(feature = "hydrate"))]
fn initial_dark_mode() -> bool {
    true
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let (dark, set_dark) = create_signal(initial_dark_mode());
    let class = move || if dark.get() { "layout dark" } else { "layout" };

    #[cfg(feature = "hydrate")]
    create_effect(move |_| {
        let theme = if dark.get() { "dark" } else { "light" };

        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item(THEME_STORAGE_KEY, theme);
        }
    });

    view! {
        <div class=class>
            <Nav dark=dark set_dark=set_dark />
            {children()}
            <Footer />
        </div>
    }
}
