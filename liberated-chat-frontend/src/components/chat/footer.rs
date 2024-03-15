use leptos::{
    component, create_signal, event_target_value, spawn_local, view, IntoView, SignalGet, SignalSet,
};

#[component]
pub fn Footer() -> impl IntoView {
    let (message, set_message) = create_signal(String::new());
    let (status, set_status) = create_signal(String::new());

    let send_fn = move || {
        spawn_local(async move {
            let msg = move || message.get();
            match crate::utils::auth::send_message(msg()).await {
                Ok(_) => (),
                Err(e) => set_status.set(format!("{e:?}")),
            }
        });
    };

    view! {
        <div class="p-4 border-t sticky bottom-0 bg-neutral-900">
            <div class="mx-auto max-w-3xl">
                <div class="flex rounded-lg border">
                    <textarea
                        class="min-h-[60px] flex-1 rounded-l-lg bg-neutral-800 p-2"
                        placeholder="Enter your message"
                        on:change=move |ex| {
                            set_message.set(event_target_value(&ex));
                        }
                    >
                    </textarea>
                    <button
                        class="rounded-r-lg bg-neutral-900 p-2"
                        on:click=move |_| {
                            send_fn();
                        }
                    >

                        Send
                    </button>
                </div>
                <p>{status}</p>
            </div>
        </div>
    }
}
