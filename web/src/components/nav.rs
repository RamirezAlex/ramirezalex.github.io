use leptos::*;

#[component]
pub fn Nav(setter: WriteSignal<bool>) -> impl IntoView {
    let handle_click = move |_| {
        leptos::logging::log!("Turning off the light");
        setter.update(|mode| {
            leptos::logging::log!("mode: {}", mode);
            *mode = !*mode
        });
    };

    view! {
        <nav>
            <button on:click=handle_click>Turn off the light</button>
        </nav>
    }
}
