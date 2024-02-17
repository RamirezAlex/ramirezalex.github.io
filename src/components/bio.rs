use leptos::*;

#[component]
pub fn Bio() -> impl IntoView {
    view! {
        <section id="bio">
            <picture>
                <img
                    class="profile-biok-image"
                    src="../images/ramirezalex-white.png"
                    alt="Alex Ramirez"
                />
                <p>
                    Written by <strong>Alex Ramirez.</strong> Alex is Software Developer based in Medellin, Colombia with experience in different programming languages and technologies, particularly Rust and TypeScript. He also is very interested in the Web3 ecosystem, espcially in Solana Blockchain.
                    <a href="https://twitter.com/RamirezAlex_">
                        @RamirezAlex_
                    </a>
                </p>
            </picture>
        </section>
    }
}
