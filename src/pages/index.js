import React from "react"

import Layout from "../components/layout"
import SEO from "../components/seo"

const IndexPage = () => (
  <Layout>
    <SEO title="Home" />
    <div id="content">
      <header>
        <div className="half-block">
          <h1>Alex Ramirez</h1>
          <h2>Software Developer</h2>
        </div>
        <div className="half-block">
          <picture className="profile-image"></picture>
        </div>
      </header>
      <section id="info">
        <p>
          Software Developer * JavaScript Lover * Python Enthusiastic<br />
          Machine Learning Newcomer * Blockchain Padawan * Rust Apprentice<br />
          co-organizer of <a target="_blank" rel="noreferrer" href="https://twitter.com/medellinjs">@MedellinJS</a>
        </p>
      </section>
      <section id="social-media">
        <ul>
          <li className="twitter">
            <a target="_blank" rel="noreferrer" aria-label="Twitter - @RamirezAlex_" href="https://twitter.com/RamirezAlex_"><i className="fab fa-twitter-square"></i></a>
          </li>
          <li className="github">
            <a target="_blank" rel="noreferrer" aria-label="Github - @RamirezAlex" href="https://github.com/RamirezAlex"><i className="fab fa-github-square"></i></a>
          </li>
          <li className="linkedin">
            <a target="_blank" rel="noreferrer" aria-label="LinkedIn - @ramirezalex1" href="https://www.linkedin.com/in/ramirezalex1/"><i className="fab fa-linkedin"></i></a>
          </li>
          <li className="instagram">
            <a target="_blank" rel="noreferrer" aria-label="Instagram - @_ramirezalex" href="https://www.instagram.com/_ramirezalex/"><i className="fab fa-instagram"></i></a>
          </li>
          <li className="facebook">
            <a target="_blank" rel="noreferrer" aria-label="Facebook - @ramirezalex4" href="https://www.facebook.com/ramirezalex4"><i className="fab fa-facebook"></i></a>
          </li>
        </ul>
      </section>
    </div>
  </Layout>
)

export default IndexPage
