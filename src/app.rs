use crate::error_template::{AppError, ErrorTemplate};
use html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use server_fn::error::ServerFnErrorSerde;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/webbed_site.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <NavBar/>
                <Routes>
                    <Route path="/home" view=Home/>
                    <Route path="/fib" view=Fib/>
                </Routes>
            </main>
        </Router>
    }
}

// /// Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }
#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
            </button>
        </main>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-gray-200 shadow shadow-gray-300 w-100 px-8 md:px-auto space-x-4 justify-between">
            <A href="home">"Home"</A>
            <A href="fib">"Fib"</A>
        </nav>
    }
}

#[server(CalcFib, "/api")]
pub async fn calc_fib(input: String) -> Result<u128, ServerFnError> {
    fn fib_memo(n: u128, memo: &mut [u128; 2]) -> u128 {
        let [a, b] = *memo;
        let c = a + b;

        if n == 0 {
            c
        } else {
            *memo = [b, c];
            fib_memo(n - 1, memo)
        }
    }

    let target: u128 = input.parse().unwrap_or_default();

    if target < 2 {
        Ok(1)
    } else {
        Ok(fib_memo(target - 2, &mut [0, 1]))
    }
}

#[component]
pub fn Fib() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();

    // a server action can be created by using the server function's type name as a generic
    // the type name defaults to the PascalCased function name
    let action = create_server_action::<CalcFib>();

    view! {
        <h3>Using <code>create_action</code></h3>
        // <p>
        // "Some server functions are conceptually \"mutations,\", which change something on the server. "
        // "These often work well as actions."
        // </p>
        <input node_ref=input_ref placeholder="Give me an Integer"/>
        <button on:click=move |_| {
            let text = input_ref.get().unwrap().value();
            action.dispatch(text.into());
        }>

            Submit
        </button>
        <p>
        The result was:
        {move || if action.value().get().is_none() {
            "Nothing right now".to_string()
        } else if action.value().get().is_some() {
            let eeg = action.value().get().unwrap().unwrap();
            format!("{}", eeg)
        } else {
            format!("{:?}", "shit".to_string())
        }
        }
            </p>
    }
}
