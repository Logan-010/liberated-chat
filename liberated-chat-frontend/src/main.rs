use leptos::{component, create_signal, mount_to_body, view, IntoView, Show, SignalGet};
use liberated_chat_frontend::components::chat::Chat;
use liberated_chat_frontend::components::login::LoginPage;
use liberated_chat_frontend::components::navbar::NavBar;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("Initialize logger");

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (login_toggle, set_login_toggle) = create_signal(true);

    view! {
        <style>{include_str!("../style.css")}</style>
        <main>
            <NavBar toggle_login=set_login_toggle/>

            <Show when=move || { !login_toggle.get() } fallback=move || view! { <LoginPage/> }>
                <Chat/>
            </Show>
        </main>
    }
}
