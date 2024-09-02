use std::{collections::HashMap, convert::Infallible, future::Future, net::SocketAddr};

use axum::{body::Body, extract::{Query, State}, handler::HandlerWithoutStateExt, response::IntoResponse, routing::get, Router};
use clap::Parser;
use futures::{stream, StreamExt};
use hyper::Uri;

mod utils;
mod routes;
mod controllers;
use frontend::{ServerApp, ServerAppProps};
use tower_http::services::ServeDir;
use utils::clap::Opt;
use yew::platform::Runtime;

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>
) -> impl IntoResponse {
    let url = url.to_string();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        uri: url.into(),
        queries,
    });

    Body::from_stream(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

// An executor to process requests on the Yew runtime.
// 
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
// 
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

pub async fn run() {
    let _exec = Executor::default();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let app = Router::new()
        .fallback_service(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    get(render)
                    .with_state((index_html_before.clone(), index_html_after.clone()))
                    .into_service(),
                ),
        );
    
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    println!("LISTENING: http://localhost:8000");

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to serve");
}