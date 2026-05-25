use leptos::*;

#[component]
pub fn Bio() -> impl IntoView {
    view! {
        <section id="bio">
            <div class="bio-avatar">
                <img
                    class="profile-bio-image"
                    src="/images/ramirezalex-white.jpg"
                    alt="Alex Ramirez"
                />
            </div>
            <p>
                Written by <strong>Alex Ramirez.</strong> Alex is a software developer based in Medellin, Colombia with experience in different programming languages and technologies, particularly Rust and TypeScript. He is also very interested in the Web3 ecosystem, especially Solana.
                " "
                <a href="https://x.com/RamirezAlex">
                    "@RamirezAlex"
                </a>
            </p>
        </section>
    }
}
