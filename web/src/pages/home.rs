use crate::components::layout::Layout;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout>
            <div id="content">
            <header>
                <div class="half-block">
                <h1>Alex Ramirez</h1>
                <h2>Software Engineer</h2>
                </div>
                <div class="half-block">
                <picture class="profile-image"></picture>
                </div>
            </header>
            <section id="info">
                <p>
                Software Engineer * JavaScript Lover * Python Enthusiastic<br />
                Go Learner * Blockchain Padawan * Rust Apprentice<br />
                co-organizer of <a target="_blank" rel="noreferrer" href="https://twitter.com/rustmedellin">@RustMedellin</a> and
                <a target="_blank" rel="noreferrer" href="https://twitter.com/medellinjs">@MedellinJS</a>
                </p>
            </section>
            <section id="social-media">
                <ul>
                <li class="twitter">
                    <a target="_blank" rel="noreferrer" aria-label="Twitter - @RamirezAlex_" href="https://twitter.com/RamirezAlex_"><i class="fab fa-twitter-square"></i></a>
                </li>
                <li class="github">
                    <a target="_blank" rel="noreferrer" aria-label="Github - @RamirezAlex" href="https://github.com/RamirezAlex"><i class="fab fa-github-square"></i></a>
                </li>
                <li class="linkedin">
                    <a target="_blank" rel="noreferrer" aria-label="LinkedIn - @ramirezalex1" href="https://www.linkedin.com/in/ramirezalex1/"><i class="fab fa-linkedin"></i></a>
                </li>
                <li class="instagram">
                    <a target="_blank" rel="noreferrer" aria-label="Instagram - @_ramirezalex" href="https://www.instagram.com/_ramirezalex/"><i class="fab fa-instagram"></i></a>
                </li>
                <li class="facebook">
                    <a target="_blank" rel="noreferrer" aria-label="Facebook - @ramirezalex4" href="https://www.facebook.com/ramirezalex4"><i class="fab fa-facebook"></i></a>
                </li>
                </ul>
            </section>
            </div>
        </Layout>
    }
}
