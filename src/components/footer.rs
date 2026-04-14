use leptos::*;
use leptos_meta::Script;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <Script async_="true" src="https://www.googletagmanager.com/gtag/js?id=G-LMD9JFSZTB"></Script>
            <Script>{r#"
                window.dataLayer = window.dataLayer || [];
                function gtag(){dataLayer.push(arguments);}
                gtag('js', new Date());
                gtag('config', 'G-LMD9JFSZTB');
            "#}</Script>
            <span>"© 2026 Alex Ramirez"</span>
            <span>"Medellín, Colombia · Built with Rust & Leptos"</span>
        </footer>
    }
}
