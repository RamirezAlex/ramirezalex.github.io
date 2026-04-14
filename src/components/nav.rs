use icondata as i;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn Nav(dark: ReadSignal<bool>, set_dark: WriteSignal<bool>) -> impl IntoView {
    let icon = Signal::derive(move || if dark.get() { i::BsSun } else { i::BsMoonStars });

    view! {
        <nav class="site-nav">
            <A class="nav-logo" href="/">"AR"</A>
            <div class="nav-links">
                <A class="nav-link" href="/blog">"Blog"</A>
                <a
                    class="nav-link"
                    href="https://github.com/RamirezAlex"
                    target="_blank"
                    rel="noreferrer"
                >
                    "GitHub"
                </a>
                <button
                    class="mode-toggle"
                    on:click=move |_| set_dark.update(|d| *d = !*d)
                    aria-label="Toggle dark mode"
                >
                    <Icon icon=icon width="16px" height="16px" />
                </button>
                <a class="nav-cta" href="mailto:alexander.ramirez@gmail.com">"Let's talk"</a>
            </div>
        </nav>
    }
}
