use crate::components::layout::Layout;
use icondata as i;
use leptos::*;
use leptos_icons::Icon;

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
                        <a target="_blank" rel="noreferrer" aria-label="Twitter - @RamirezAlex_" href="https://twitter.com/RamirezAlex_">
                            <Icon icon=i::BsTwitterX class="fab" />
                        </a>
                    </li>
                    <li class="github">
                        <a target="_blank" rel="noreferrer" aria-label="Github - @RamirezAlex" href="https://github.com/RamirezAlex">
                            <Icon icon=i::FaGithubBrands class="fab" />
                        </a>
                    </li>
                    <li class="linkedin">
                        <a target="_blank" rel="noreferrer" aria-label="LinkedIn - @ramirezalex1" href="https://www.linkedin.com/in/ramirezalex1/">
                            <Icon icon=i::FaLinkedinBrands class="fab" />
                        </a>
                    </li>
                    <li class="instagram">
                        <a target="_blank" rel="noreferrer" aria-label="Instagram - @_ramirezalex" href="https://www.instagram.com/_ramirezalex/">
                            <Icon icon=i::FaInstagramBrands class="fab" />
                        </a>
                    </li>
                    <li class="facebook">
                        <a target="_blank" rel="noreferrer" aria-label="Facebook - @ramirezalex4" href="https://www.facebook.com/ramirezalex4">
                            <Icon icon=i::FaFacebookBrands class="fab" />
                        </a>
                    </li>
                    </ul>
                </section>
            </div>
        </Layout>
    }
}
