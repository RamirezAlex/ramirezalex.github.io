use crate::components::layout::Layout;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout>
            // ── Hero ──────────────────────────────────────────────────
            <section class="hero">
                <div class="hero-left">
                    <div class="eyebrow">
                        <span class="eyebrow-dot eyebrow-dot-purple"></span>
                        "Medellín, Colombia · Available for work"
                    </div>
                    <div class="hero-headline">
                        <span>"Senior Engineer."</span>
                        <span class="hl-accent">"Rust. Solana."</span>
                        <span>"Products that ship."</span>
                    </div>
                    <p class="hero-tagline">
                        "I build technically strong systems that work in the real world — from Solana
                        smart contracts to full-stack applications. Based in Medellín, shipping globally."
                    </p>
                    <div class="hero-ctas">
                        <a class="btn-primary" href="/blog">"Read the blog"</a>
                        <a
                            class="btn-secondary"
                            href="https://github.com/RamirezAlex"
                            target="_blank"
                            rel="noreferrer"
                        >
                            "View GitHub"
                        </a>
                    </div>
                </div>
                <div class="hero-right">
                    <div class="terminal-card">
                        <div class="terminal-bar">
                            <span class="terminal-dot red"></span>
                            <span class="terminal-dot yellow"></span>
                            <span class="terminal-dot green"></span>
                            <span class="terminal-bar-title">"alex@ramirezalex ~ "</span>
                        </div>
                        <div class="terminal-body">
                            <p class="t-line cmd">"$ whoami"</p>
                            <p class="t-line out">"Senior Engineer — Rust · TypeScript · Solana"</p>
                            <p class="t-line cmd">"$ cat stack.txt"</p>
                            <p class="t-line muted">"Rust / Anchor / Solana / TypeScript / Next.js"</p>
                            <p class="t-line muted">"Node.js / PostgreSQL / Prisma / Axum / Leptos"</p>
                            <p class="t-line cmd">"$ cat now.txt"</p>
                            <p class="t-line cyan">"Dare Market · Mootrack · PushFlip · SOLClaw"</p>
                            <p class="t-line green">"$ building since 2021 on Solana ▊"</p>
                        </div>
                    </div>
                </div>
            </section>

            <div class="section-divider"></div>

            // ── Skills ────────────────────────────────────────────────
            <section class="skills-section">
                <div class="section-header">
                    <div class="section-eyebrow">
                        <span class="eyebrow-dot eyebrow-dot-cyan"></span>
                        "What I build"
                    </div>
                    <h2 class="section-title">"Deep expertise." <br/> "Multiple layers."</h2>
                </div>
                <div class="skills-grid">
                    <div class="skill-card">
                        <span class="skill-num">"01"</span>
                        <strong class="skill-title">"Solana Programs"</strong>
                        <p class="skill-desc">
                            "Smart contracts, on-chain game logic, token mechanics, and verifiable
                            computation. Building since 2021."
                        </p>
                    </div>
                    <div class="skill-card">
                        <span class="skill-num">"02"</span>
                        <strong class="skill-title">"Backend Systems"</strong>
                        <p class="skill-desc">
                            "API design, distributed systems, PostgreSQL, Prisma. From Axum in Rust
                            to Node.js at scale."
                        </p>
                    </div>
                    <div class="skill-card">
                        <span class="skill-num">"03"</span>
                        <strong class="skill-title">"Product Engineering"</strong>
                        <p class="skill-desc">
                            "Full-stack execution from ambiguous idea to shipped product.
                            TypeScript, React, Next.js, rapid prototyping."
                        </p>
                    </div>
                    <div class="skill-card">
                        <span class="skill-num">"04"</span>
                        <strong class="skill-title">"AI-Native Products"</strong>
                        <p class="skill-desc">
                            "Agent workflows, LLM integrations, farm copilots, moderation systems.
                            AI as real product leverage."
                        </p>
                    </div>
                </div>
            </section>

            <div class="section-divider"></div>

            // ── Projects ──────────────────────────────────────────────
            <section class="projects-section">
                <div class="section-header">
                    <div class="section-eyebrow">
                        <span class="eyebrow-dot eyebrow-dot-pink"></span>
                        "Selected work"
                    </div>
                    <h2 class="section-title">"Real products." <br/> "Real users."</h2>
                </div>
                <div class="proj-grid">
                    <div class="proj-row">
                        <div class="proj-card">
                            <div class="proj-tags">
                                <span class="proj-tag purple">"Solana"</span>
                                <span class="proj-tag cyan">"Rust"</span>
                                <span class="proj-tag muted">"TypeScript"</span>
                            </div>
                            <h3 class="proj-title">"Dare Market"</h3>
                            <p class="proj-desc">
                                "Web3-powered challenge marketplace on Solana. Users create, fund, and win
                                crypto-backed dares. Built across Solana contracts, Next.js, PostgreSQL,
                                and AI moderation."
                            </p>
                            <span class="proj-role purple">"Role: Senior Engineer / Product Engineer"</span>
                        </div>
                        <div class="proj-card">
                            <div class="proj-tags">
                                <span class="proj-tag purple">"Solana"</span>
                                <span class="proj-tag cyan">"AI"</span>
                                <span class="proj-tag muted">"LATAM"</span>
                            </div>
                            <h3 class="proj-title">"Mootrack"</h3>
                            <p class="proj-desc">
                                "Solana-powered cattle traceability and farm management platform.
                                AI-native operating system for livestock farms. On-chain animal identity,
                                ownership, and financing rails."
                            </p>
                            <span class="proj-role pink">"Role: Co-founder / Lead Engineer"</span>
                        </div>
                    </div>
                    <div class="proj-row">
                        <div class="proj-card featured-purple">
                            <div class="proj-tags">
                                <span class="proj-tag purple">"Solana"</span>
                                <span class="proj-tag cyan">"ZK Proofs"</span>
                                <span class="proj-tag muted">"Game"</span>
                            </div>
                            <h3 class="proj-title">"PushFlip"</h3>
                            <p class="proj-desc">
                                "Crypto-native push-your-luck card game on Solana. Token mechanics, AI
                                opponent flows, and ZK-based deck verification for provable fairness."
                            </p>
                            <span class="proj-badge purple">"Selected Project"</span>
                        </div>
                        <div class="proj-card featured-cyan">
                            <div class="proj-tags">
                                <span class="proj-tag purple">"Solana"</span>
                                <span class="proj-tag cyan">"AI Agents"</span>
                                <span class="proj-tag muted">"Protocol"</span>
                            </div>
                            <h3 class="proj-title">"SOLClaw"</h3>
                            <p class="proj-desc">
                                "Agent runtime and protocol for putting AI agents on-chain on Solana.
                                Agent identity, memory anchoring, task execution, and payment rails as
                                protocol primitives."
                            </p>
                            <span class="proj-badge cyan">"Selected Project"</span>
                        </div>
                    </div>
                </div>
            </section>

            <div class="section-divider"></div>

            // ── CTA ───────────────────────────────────────────────────
            <section class="cta-section">
                <div class="section-eyebrow">
                    <span class="eyebrow-dot eyebrow-dot-purple"></span>
                    "Open to opportunities"
                </div>
                <h2 class="cta-headline">"Let's build something" <br/> "that ships."</h2>
                <p class="cta-sub">
                    "Whether it's a Solana program, a backend system, or a full product — I bring
                    technical depth and product thinking to every engagement."
                </p>
                <div class="cta-buttons">
                    <a class="btn-primary" href="mailto:alexander.ramirez@gmail.com">
                        "alexander.ramirez@gmail.com"
                    </a>
                    <a
                        class="btn-secondary"
                        href="https://x.com/RamirezAlex"
                        target="_blank"
                        rel="noreferrer"
                    >
                        "x.com/RamirezAlex"
                    </a>
                </div>
            </section>
        </Layout>
    }
}
