use leptos::*;
use leptos_meta::Script;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <Script async_="true" src="https://www.googletagmanager.com/gtag/js?id=G-LMD9JFSZTB"></Script>
            <Script>{r#"
                window.dataLayer = window.dataLayer || [];
                    function gtag(){dataLayer.push(arguments);}
                    gtag('js', new Date());
                    gtag('config', 'G-LMD9JFSZTB');
            "#}</Script>
            RamirezAlex - 2026
        </footer>
    }
}
