use std::time::Duration;
use gloo_timers::future::TimeoutFuture;
use rand::Rng;

use sycamore::{easing, motion::{create_tweened_signal }, prelude::*, futures::spawn_local_scoped};


#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let mut rng = rand::thread_rng();

    let state = create_signal(cx, 0.0f32);
    let tweened_state = create_tweened_signal(cx, 0.0f32, Duration::from_millis(250), easing::quad_out);

    create_effect(cx, || {
        tweened_state.set(*state.get());
    }); 

    spawn_local_scoped(cx, async move {
        loop {
            TimeoutFuture::new(1000).await;
            let n: f32 = rng.gen();

            state.set(n);
        }
    });

    // let increment = |_| state.set(*state.get() + 1.0f32);
    // let decrement = |_| state.set(*state.get() - 1.0f32);
    // let reset = |_| state.set(0.0f32);

    view! { cx,
        div {
            label { (format!("{:.2} ", tweened_state.get())) }
            progress(prop:value=*tweened_state.get())
        }
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            table {
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
                tr {
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                    td { App { }}
                }
            }
        }
    });
}
