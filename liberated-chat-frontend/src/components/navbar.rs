use leptos::{
    component, create_signal, spawn_local, view, IntoView, SignalGet, SignalSet, SignalUpdate,
    WriteSignal,
};

#[component]
pub fn NavBar(toggle_login: WriteSignal<bool>) -> impl IntoView {
    let (clickme_status, set_clickme_status) = create_signal(true);

    let logout_fn = move || {
        spawn_local(async move {
            _ = crate::utils::auth::logout().await;
        });
    };

    view! {
        <nav class="flex items-center h-14 px-4 border-b gap-4 md:gap-6">
            <div class="flex items-center gap-2 text-lg font-semibold">
                <span>Liberated Chat</span>
                <Icon/>
            </div>
            <div class="ml-auto flex items-center gap-4">
                <p class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors">
                    {move || {
                        if clickme_status.get() {
                            "Click me to close the login window ->"
                        } else {
                            ""
                        }
                    }}

                </p>
                <button
                    class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3"
                    on:click=move |_| {
                        set_clickme_status.set(false);
                        toggle_login.update(|val| *val = !*val);
                    }
                >

                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="w-4 h-4 mr-2"
                    >
                        <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path>
                        <polyline points="10 17 15 12 10 7"></polyline>
                        <line x1="15" x2="3" y1="12" y2="12"></line>
                    </svg>
                    <span>Login</span>
                </button>
                <button
                    class="inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3"
                    on:click=move |_| { logout_fn() }
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="w-4 h-4 mr-2"
                    >
                        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
                        <polyline points="16 17 21 12 16 7"></polyline>
                        <line x1="21" x2="9" y1="12" y2="12"></line>
                    </svg>
                    <span>Logout</span>
                </button>
            </div>
        </nav>
    }
}

#[component]
fn Icon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width="20px"
            height="20px"
            viewBox="0 0 64 64"
            id="Layer_1"
            version="1.1"
            xml:space="preserve"
        >
            <g>
                <g>
                    <path
                        d="M43.124,38.264c-0.712,0-1.424-0.272-1.967-0.815c-1.086-1.086-1.086-2.847,0-3.934l7.403-7.403    c2.737-2.737,2.737-7.19,0-9.927c-2.737-2.737-7.19-2.737-9.927,0l-3.169,3.169c-1.086,1.086-2.847,1.086-3.934,0    c-1.086-1.086-1.086-2.847,0-3.934l3.169-3.169c2.377-2.376,5.536-3.685,8.897-3.685c3.361,0,6.52,1.309,8.897,3.685    c4.906,4.906,4.906,12.888,0,17.794l-7.403,7.403C44.548,37.993,43.836,38.264,43.124,38.264z"
                        style="fill:#515151;"
                    ></path>
                </g>
                <g>
                    <path
                        d="M21.875,55.454c-3.361,0-6.52-1.309-8.897-3.685c-2.376-2.376-3.685-5.536-3.685-8.897    c0-3.361,1.309-6.521,3.685-8.897l7.403-7.403c1.086-1.086,2.847-1.086,3.934,0c1.086,1.086,1.086,2.847,0,3.934l-7.403,7.403    c-2.737,2.737-2.737,7.19,0,9.927c1.326,1.326,3.088,2.056,4.963,2.056c1.875,0,3.637-0.73,4.963-2.056l3.169-3.169    c1.086-1.086,2.847-1.086,3.934,0c1.086,1.086,1.086,2.847,0,3.934l-3.169,3.169C28.396,54.145,25.236,55.454,21.875,55.454z"
                        style="fill:#515151;"
                    ></path>
                </g>
                <g>
                    <path
                        d="M34.763,36.384l12.64,12.953c1.081,1.108,1.06,2.883-0.048,3.964    c-1.108,1.081-2.883,1.06-3.964-0.048c-0.163-0.167-0.308-0.362-0.421-0.555l-9.054-15.67c-0.148-0.256-0.06-0.584,0.196-0.733    C34.328,36.169,34.596,36.213,34.763,36.384z"
                        style="fill:#515151;"
                    ></path>
                </g>
                <g>
                    <path
                        d="M43.682,40.592l11.855,3.295c1.505,0.418,2.387,1.978,1.968,3.483    c-0.418,1.505-1.978,2.387-3.483,1.968c-0.342-0.095-0.66-0.256-0.929-0.455l-9.878-7.336c-0.24-0.178-0.29-0.517-0.112-0.757    C43.239,40.607,43.473,40.535,43.682,40.592z"
                        style="fill:#515151;"
                    ></path>
                </g>
                <g>
                    <path
                        d="M21.797,22.727L8.434,18.895c-1.496-0.429-2.362-1.99-1.933-3.486    c0.429-1.496,1.99-2.362,3.486-1.933c0.299,0.086,0.582,0.224,0.825,0.39l11.44,7.898c0.245,0.169,0.307,0.505,0.137,0.75    C22.255,22.71,22.013,22.788,21.797,22.727z"
                        style="fill:#515151;"
                    ></path>
                </g>
                <g>
                    <path
                        d="M29.005,25.121l-9.668-11.999c-0.974-1.209-0.784-2.979,0.425-3.953    c1.209-0.974,2.979-0.784,3.953,0.425c0.174,0.216,0.316,0.463,0.416,0.708l5.791,14.28c0.112,0.275-0.021,0.589-0.296,0.701    C29.401,25.373,29.151,25.301,29.005,25.121z"
                        style="fill:#515151;"
                    ></path>
                </g>
            </g>
        </svg>
    }
}
