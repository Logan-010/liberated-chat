use leptos::{
    component, create_signal, spawn_local, view, CollectView, IntoView, Show, SignalGet, SignalSet,
};

#[component]
pub fn Messages() -> impl IntoView {
    let (result, set_result) = create_signal(String::new());
    let (messages, set_messages) = create_signal(Vec::new());
    let (first_click, set_first_click) = create_signal(true);

    let load_messages_fn = move || {
        spawn_local(async move {
            set_first_click.set(false);

            match super::Post::new().await {
                Ok(v) => {
                    set_result.set(String::new());
                    set_messages.set(v)
                }
                Err(e) => set_result.set(format!("{e:?}")),
            }
        });
    };

    view! {
        <button
            class="whitespace-nowrap shrink-0 font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none shadow-sm hover:bg-secondary/80 text-xs hidden md:flex md:flex-col bg-black text-zinc-500  md:aspect-video text-center items-center justify-center gap-2 cursor-pointer rounded-lg h-auto w-auto  disabled:opacity-50 disabled:cursor-pointer pointer-events-auto max-h-full basis-full md:basis-auto py-2 md:py-2.5 px-2.5"
            on:click=move |_| {
                load_messages_fn();
            }
        >

            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="15"
                height="15"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="w-4 h-4"
            >
                <path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"></path>
                <path d="M21 3v5h-5"></path>
                <path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"></path>
                <path d="M8 16H3v5"></path>
            </svg>
            <span>Reload</span>
        </button>

        <Show
            when=move || { first_click.get() }
            fallback=move || {
                view! {}
            }
        >

            <p>{"^ Click me to load messages!"}</p>
        </Show>

        <Show
            when=move || { result.get().is_empty() }
            fallback=move || {
                view! { <h1 class="text-red">{format!("Error: {}", result.get())}</h1> }
            }
        >

            {move || {
                messages
                    .get()
                    .iter()
                    .map(|post| {
                        view! {
                            <Message username=&post.user message=&post.message time=&post.time/>
                        }
                    })
                    .collect_view()
            }}

        </Show>
    }
}

#[component]
fn Message<'a>(username: &'a str, message: &'a str, time: &'a str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-2">
            <div class="grid gap-1 text-sm">
                <div class="font-semibold">{username.to_string()} :</div>
                <div class="text-sm">{message.to_string()}</div>
                <div class="text-xs text-gray-500 dark:text-gray-400">- {time.to_string()}</div>
            </div>
        </div>
    }
}
