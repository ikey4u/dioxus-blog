#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Think {},
        #[route("/post/:name")]
        ThinkPost { name: String },

        #[route("/spark")]
        Spark {},

        #[route("/about")]
        About {},

        #[route("/more")]
        More {},
    #[end_layout]

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn NavBar() -> Element {
    let current_route = use_route::<Route>().to_string();
    rsx! {
        div {
            nav {
                class: "bg-gray-100",
                div {
                    class: "flex items-center justify-center h-16",
                    border: "1px solid green",
                    ul {
                        class: "flex space-x-4 font-bold",
                        li {
                            Link {
                                color: if current_route == "/" { "red" } else { "" },
                                to: Route::Think {},
                                "THINK"
                            }
                        }
                        li {
                            Link {
                                color: if current_route.starts_with("/spark") { "red" } else { "" },
                                to: Route::Spark {},
                                "SPARK" }
                        }
                        li {
                            Link { to: Route::About {}, "ABOUT" }
                        }
                        li {
                            Link { to: Route::More {}, "MORE." }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Spark() -> Element {
    rsx! {
        h1 { "SPARK TODO" }
    }
}

#[component]
fn Think() -> Element {
    rsx! {
        ul {
            li {
                Link {
                    to: Route::ThinkPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::ThinkPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[component]
fn ThinkPost(name: String) -> Element {
    rsx! { h2 { "Blog Post: {name}" } }
}

#[component]
fn About() -> Element {
    rsx!({ "ABOUT TODO" })
}

#[component]
fn More() -> Element {
    rsx!({ "MORE. TODO" })
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}
