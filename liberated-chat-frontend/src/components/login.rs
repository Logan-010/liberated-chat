use leptos::{
    component, create_signal, event_target_value, spawn_local, view, IntoView, SignalGet,
    SignalSet, SignalUpdate,
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (login_result, set_login_result) = create_signal(String::new());

    let login_fn = move || {
        spawn_local(async move {
            let un = move || username.get();
            let pass = move || password.get();

            let res = crate::utils::auth::login(&un(), &pass()).await;

            match res {
                Ok(v) => set_login_result.set(v),
                Err(e) => set_login_result.set(e.to_string()),
            }
        });
    };

    let register_fn = move || {
        spawn_local(async move {
            let un = move || username.get();
            let pass = move || password.get();

            let res = crate::utils::auth::register(&un(), &pass()).await;

            match res {
                Ok(v) => set_login_result.set(v),
                Err(e) => set_login_result.set(e.to_string()),
            }
        });
    };

    view! {
        <div class="flex items-center justify-center h-screen overflow-auto scrollbar-hide">
            <div class="rounded-lg border bg-card text-card-foreground shadow-sm w-full max-w-sm">
                <div class="flex flex-col space-y-1.5 p-3 text-center">
                    <h3 class="whitespace-nowrap tracking-tight text-3xl font-bold">Login:</h3>
                    <p class="text-sm text-muted-foreground">
                        Enter your username and password below to login or register your account.
                    </p>
                </div>
                <div class="p-6 space-y-4">
                    <div class="text-center space-y-2">
                        <label
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                            for="username"
                        >
                            Username:
                        </label>
                        <input
                            class="bg-neutral-800 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            id="username"
                            placeholder="My username"
                            required=""
                            type="username"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_username.update(|v| *v = val);
                            }
                        />

                        <div
                            data-lastpass-icon-root=""
                            style="position: relative !important; height: 0px !important; width: 0px !important; float: left !important;"
                        ></div>
                    </div>
                    <div class="space-y-2 text-center">
                        <label
                            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                            for="password"
                        >
                            Password:
                        </label>
                        <input
                            class="bg-neutral-800 flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                            id="password"
                            required=""
                            type="password"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.update(|v| *v = val);
                            }
                        />

                        <div
                            data-lastpass-icon-root=""
                            style="position: relative !important; height: 0px !important; width: 0px !important; float: left !important;"
                        ></div>
                    </div>
                </div>
                <div class="items-center p-2 flex justify-center">
                    <button
                        class="bg-neutral-800 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2 w-10/12"
                        on:click=move |_| {
                            set_login_result.set(String::new());
                            login_fn();
                        }
                    >

                        Login
                    </button>

                </div>
                <div class="items-center p-2 flex justify-center">
                    <button
                        class="bg-neutral-800 inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2 w-10/12"
                        on:click=move |_| {
                            set_login_result.set(String::new());
                            register_fn();
                        }
                    >

                        Register
                    </button>
                </div>
                <p class="items-center flex justify-center text-center p-2 flex text-indigo-600">
                    {login_result}
                </p>
            </div>
        </div>
    }
}
