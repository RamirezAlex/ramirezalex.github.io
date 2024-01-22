use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::nav::Nav;

use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let (mode, set_mode) = create_signal(String::from("light-mode"));

    view! {
        <div class=mode>
            <Nav setter=set_mode />
            <div>
                <Header/>{children()} <Footer/>
            </div>
        </div>
    }
}
