use icondata as i;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Nav(setter: WriteSignal<String>) -> impl IntoView {
    let (icon_mode, set_icon_mode) = create_signal(i::BsMoonStars);
    let handle_click = move |_| {
        set_icon_mode.update(move |mode| {
            if *mode == i::BsMoonStars {
                *mode = i::BsSun;
                setter.set(String::from("dark-mode"));
            } else {
                *mode = i::BsMoonStars;
                setter.set(String::from("light-mode"));
            }
        })
    };

    view! {
        <nav>
            <button on:click=handle_click>
                "Turn off the light"
            </button>
            <Icon icon=icon_mode width="1em" height="1em" style="padding-left: 10px; vertical-align: middle"/>
        </nav>
    }
}
