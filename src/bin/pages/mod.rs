use axum::{response::Html, routing::get, Router};
use html_node::{html, Node};
pub mod index;
pub mod posts;
use htmx_rs_template::{components::navbar::navbar, PageRoute};

pub fn route() -> Router {
    PageRoute {
        page,
        layout,
        ..PageRoute::root()
    }
    .route("/hello", get("hello :)"))
    .nest("/posts", posts::router)
    .build()
}

pub fn layout(children: Node) -> Node {
    html! {
        <!doctype html>
        <html lang="en">
            <head>
                <link href="/assets/main.css" rel="stylesheet" />
                <link href="https://rsms.me/inter/inter.css" rel="stylesheet" />
                <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=IBM+Plex+Mono"/>
                <!-- "htmx from the unpkg CDN - your mileage may vary" -->
                <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            </head>
            <body class="bg-tasman-600">
                {navbar()}
                <div id="content">
                    <!-- "Inheriting pages will have their content rendered here, similar to app root in React, Angular, etc." -->
                    {children}
                </div>
            </body>
        </html>
    }
}

pub fn page() -> Node {
    html! {
        <main class="w-full p-4 flex justify-center place-items-center">
            <div class="max-w-prose flex flex-col gap-4 font-mono text-tasman-200">
                <h1 class="text-3xl text-center font-bold text-accent">"This webpage is a test"</h1>
                <img
                    class="mx-8 shadow-2xl aspect-video w-full object-cover"
                    src="/assets/images/cat.jpg"
                    alt="a cute cat sitting next to a fireplace"
                >
                <p class="text-center italic opacity-60 text-sm">"This is a cat"</p>

                <p>"Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequatur, ea blanditiis quis ipsam fugit ut dignissimos repellendus debitis neque delectus quaerat cum nisi, sed accusantium, modi eaque. Consequatur, sed vitae!"</p>
                <h2 class="text-lg font-semibold">"Lorem ipsum dolor sit amet."</h2>
                <p>"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Velit voluptatem vero quam rem ratione doloremque placeat iste officiis dolorem praesentium voluptate quis, neque facere atque sequi eius laborum quos aspernatur a porro laudantium! Nesciunt molestiae iure, fugit molestias quas laboriosam esse quidem voluptatum, veritatis perferendis maiores eveniet, magnam adipisci unde."</p>
            </div>
        </main>
    }
}
