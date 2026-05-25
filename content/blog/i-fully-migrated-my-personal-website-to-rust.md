---
title = "I fully migrated my personal website to Rust"
date = "2024-02-22"
description = "I have migrated my personal website to Rust with Leptos and Axum. I tell you how I did Full Stack with Rust and WASM."
image_url = "/images/rust-leptos.webp"
---

I had a Gatsby site for a long time. It worked, it was familiar, and for a personal website it was more than enough. But at some point the stack stopped feeling like mine.

Most of my day-to-day work had moved closer to Rust, Solana, backend systems, and product infrastructure. My personal site was still sitting on a JavaScript static-site stack with old Gatsby files, old package metadata, and old assumptions about how I wanted to write software. I did not migrate because Gatsby was bad. I migrated because I wanted this site to be a small real example of the kind of software I like building now.

So I rewrote it in Rust with Leptos and Axum.

![Rust and Leptos](/images/rust-leptos.webp)

This post is the story of that migration: what changed, how the app is structured, what I had to fix after the first pass, and what I learned from using Rust for something as simple as a personal website.

## Starting point

The first meaningful migration commit was `feat: Rewrite in Rust` in January 2024. That commit did something pretty blunt: it moved the old Gatsby site into a `tmp/` directory and introduced a new Rust app under `web/`.

That old site had the usual Gatsby shape:

- `gatsby-config.js`
- `gatsby-node.js`
- React components
- page templates
- blog content under `content/blog`
- `package.json`
- `yarn.lock`

The new Rust app started from a Leptos/Axum structure:

- `src/app.rs` for the Leptos root component and routes
- `src/main.rs` for the Axum server
- `src/lib.rs` for the WASM hydration entrypoint
- `src/fileserv.rs` for static files and server rendering fallback
- `style/main.scss` for the site styles
- `public/` for assets

At first, the Rust version was intentionally small. One home page. One layout. One server. One stylesheet. I wanted the migration to land before I got clever.

That was the right call.

## The architecture

The app is a Leptos application served by Axum. It has two main compilation modes:

- `ssr`, which builds the server-side Axum binary
- `hydrate`, which builds the browser-side WASM bundle

The root app lives in `src/app.rs`. It sets up metadata, stylesheets, scripts, and routes:

```rust
<Routes>
    <Route path="/" view=Home/>
    <Route path="/blog/:slug" view=Post/>
    <Route path="/blog" view=Blog/>
</Routes>
```

The server entrypoint lives in `src/main.rs`. Axum generates the Leptos routes, registers server functions under `/api/*fn_name`, serves the app routes, and falls back to the static file handler when needed.

```rust
let routes = generate_route_list(App);

let app = Router::new()
    .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
    .leptos_routes(&leptos_options, routes, App)
    .fallback(file_and_error_handler)
    .with_state(leptos_options);
```

That is one of the reasons I like this stack. The boundary is explicit. Axum owns the HTTP server. Leptos owns the UI and routing. Server functions give me a clean way to cross from hydrated UI into server-side Rust without inventing an API layer for every small feature.

The hydration entrypoint is in `src/lib.rs`:

```rust
#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
```

That is the browser side of the app. Same components, different target.

## Bringing the blog back

The first Rust rewrite did not fully keep the old blog. A few days later I added it back.

The first version of the blog was simple. It read files directly from `content/blog`, parsed a small TOML metadata file, rendered Markdown, and showed a page. That worked for server-side rendering, but it had a weakness: the component was doing filesystem work directly.

That is fine until you remember that Leptos components can also compile for the browser. The browser does not have `std::fs`. The more the site became a real hydrated app, the more that boundary mattered.

The current version moves blog loading into server functions:

```rust
#[server]
pub async fn get_posts_list() -> Result<Vec<PostMeta>, ServerFnError> {
    get_posts_list_from_fs().map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server]
pub async fn get_post(slug: String) -> Result<Post, ServerFnError> {
    get_post_by_slug(&slug).map_err(|_| {
        ServerFnError::ServerError(String::from("Failed to get post by slug"))
    })
}
```

The actual filesystem code is compiled only for SSR:

```rust
#[cfg(feature = "ssr")]
fn get_post_by_slug(slug: &str) -> Result<Post, Box<dyn std::error::Error>> {
    let matter = Matter::<TOML>::new();

    let file_path = Path::new("./content/blog/")
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

    let full_file_path = format!("{}/{}.md", file_path.to_str().unwrap(), slug);
    let file_content = fs::read_to_string(&full_file_path)?;
    let post = matter
        .parse_with_struct::<PostMeta>(&file_content)
        .ok_or_else(|| "Failed to parse post".to_string())?;

    Ok(Post {
        meta: post.data,
        content: post.content,
    })
}
```

The UI side uses a resource:

```rust
let post = create_resource(
    move || slug.clone(),
    |slug| async move { get_post(slug).await },
);
```

That pattern is the important part. The component asks for data. The data comes from a server function. The server function owns filesystem access. The hydrated browser build gets a callable stub instead of trying to read local files.

This is the kind of architecture I prefer: small, direct, and explicit about what runs where.

## Content format

The blog posts are Markdown files with TOML front matter:

```toml
title = "I fully migrated my personal website to Rust"
date = "2024-02-22"
description = "I have migrated my personal website to Rust with Leptos and Axum."
image_url = "/images/rust-leptos.webp"
```

The Rust type is small:

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostMeta {
    #[serde(default)]
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}
```

I like this because it keeps the content portable. If I ever move away from this implementation, the posts are still Markdown. There is no database, no CMS, no schema migration. Just files.

## Styling was more work than the Rust

The Rust part was not the hardest part. The harder part was making the site feel like a finished personal site again.

The first Rust version was functional, but it still carried the shape of the starter app. Later, I did a full redesign. That redesign changed the site from a basic page into something closer to how I want to present my work now: Rust, Solana, backend systems, product engineering, and projects that actually ship.

The latest cleanup also fixed the blog template itself. The old blog card markup had an anchor wrapping a list item, while the CSS expected the opposite. It worked until it did not. I changed the blog index to use semantic list items with links inside them, and replaced old hard-coded colors with the same design tokens used by the rest of the site.

That was a good reminder: a migration is not done when the compiler passes. It is done when the product still works.

## Dark mode and small UX details

The site now defaults to dark mode and stores the selected theme in `localStorage`.

```rust
#[cfg(feature = "hydrate")]
fn initial_dark_mode() -> bool {
    let stored_theme = window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("theme").ok().flatten());

    !matches!(stored_theme.as_deref(), Some("light"))
}
```

On the server, the default is dark. In the hydrated browser build, the app checks storage and preserves the user's choice.

That is a small feature, but it touches the exact line that matters in a Leptos app: server rendering versus browser-only APIs. Browser storage belongs behind `#[cfg(feature = "hydrate")]`. Otherwise, the server build learns about things it should never touch.

## Syntax highlighting

The blog now supports syntax highlighting for fenced code blocks. The scalable version is not a hand-written Rust tokenizer. I tried that direction and it was the wrong layer.

The Markdown renderer already emits language classes for fenced code blocks:

```html
<pre><code class="language-rust">...</code></pre>
```

So the better solution is to let Markdown produce normal HTML and let Highlight.js handle the language-specific work in the browser. That means future posts can use fenced blocks like this:

```rust
fn main() {
    println!("hello from Rust");
}
```

Or this:

```cpp
#include <iostream>

int main() {
    std::cout << "hello from C++\n";
}
```

The post page triggers highlighting after async content loads, which matters because the blog content comes from a Leptos resource.

## Deployment

The app is built with `cargo-leptos`. The Dockerfile installs the Rust toolchain, adds the WASM target, installs `cargo-leptos`, installs Sass, builds the app, and copies the server binary plus generated site assets into a smaller runtime image.

The production server runs on Fly.io. The app listens on `0.0.0.0:8080`, and Fly handles HTTPS and machine lifecycle.

```toml
[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
```

For a personal site, this is probably more infrastructure than I strictly need. But I like that it is a real server. It gives me room to add server-side features without changing the whole deployment model.

## What went well

The best part of the migration was how much of the app became regular Rust.

Routing is Rust. Components are Rust. Blog parsing is Rust. Server functions are Rust. The server is Axum. The deployment artifact is one server binary plus static assets.

I also like how clear the feature flags make the architecture:

```toml
hydrate = [
  "dep:web-sys",
  "leptos/hydrate",
  "leptos_meta/hydrate",
  "leptos_router/hydrate",
]

ssr = [
  "dep:axum",
  "dep:tokio",
  "dep:tower",
  "dep:tower-http",
  "dep:leptos_axum",
  "leptos/ssr",
  "leptos_meta/ssr",
  "leptos_router/ssr",
]
```

That split forces me to think about where code runs. I like that pressure. It prevents a lot of accidental architecture.

## What was annoying

The roughest parts were not about Rust syntax. They were about toolchain and deployment edges.

WASM means you need the right target installed. Leptos means `cargo-leptos` needs to coordinate the server build, the WASM build, generated JS, and CSS. Sass means Node still shows up in the build pipeline. Deployment means environment variables like `LEPTOS_SITE_ADDR` and `LEPTOS_SITE_ROOT` need to be correct.

There was also a `Fix wasm-bingen and deps` commit later, which is exactly the kind of thing you expect when a Rust/WASM app sits for a bit and the ecosystem moves. The tradeoff is real: you get a strong type system and one language across the stack, but you also inherit a more complex build pipeline than a plain static site.

For this site, I am okay with that tradeoff.

## Was it worth it?

Yes, for me.

Not because every personal site should be Rust. Most should not. If all I wanted was a static page and a few Markdown posts, Gatsby, Astro, Next.js, Hugo, or plain HTML would all be reasonable choices.

It was worth it because this site is also a small lab. I use it to keep my Rust web stack sharp, to test ideas in a real deployed app, and to have a codebase that reflects the kind of engineering I want to do more of.

The final shape is simple:

- Leptos for UI and routing
- Axum for the server
- Leptos server functions for blog data
- Markdown files for posts
- Sass for styling
- WASM hydration for browser interactivity
- Docker and Fly.io for deployment

It is not the smallest possible personal website. It is the personal website I wanted to maintain.

And now, when I say this site is built with Rust, I mean the whole thing: the server, the UI, the blog pipeline, and the deployment artifact.
