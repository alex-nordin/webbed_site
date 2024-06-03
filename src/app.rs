use crate::error_template::{AppError, ErrorTemplate};
use html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use malachite::num::conversion::string::options::ToSciOptions;
use malachite::num::conversion::traits::ToSci;
use malachite::Natural;

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
                    <Route path="" view=Home/>
                    <Route path="/fib" view=Fib/>
                </Routes>
            </main>
        </Router>
    }
}

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
            <A href="">"Home"</A>
            <A href="fib">"Fib"</A>
        </nav>
    }
}

#[server(CalcFib, "/api")]
pub async fn calc_fib_iter(input: String) -> Result<String, ServerFnError> {
    struct Fibonacci {
        a: Natural,
        b: Natural,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci {
                a: Natural::from(1u32),
                b: Natural::from(0u32),
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = Natural;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.b.clone();
            self.b = self.a.clone();
            self.a += res.clone();

            Some(res)
        }
    }
    let target: usize = input.parse().unwrap_or_default();
    let mut options = ToSciOptions::default();
    options.set_precision(30);
    let fib_iterator = Fibonacci::new().nth(target);
    if let Some(f) = fib_iterator {
        Ok(f.to_sci_with_options(options).to_string())
    } else {
        Err(ServerFnError::new("Issue with iterator".to_string()))
    }
}

#[component]
pub fn Fib() -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();

    // a server action can be created by using the server function's type name as a generic
    // the type name defaults to the PascalCased function name
    let action = create_server_action::<CalcFib>();

    view! {
        <div class="mt-20">
        <h3 class="text-center">Using <code>create_action</code></h3>
        <div class="grid justify-center mt-5">
        // <p>
        // "Some server functions are conceptually \"mutations,\", which change something on the server. "
        // "These often work well as actions."
        // </p>
        <div class="flex justify-center">
        <input node_ref=input_ref placeholder="Give me an Integer"/>

        <button on:click=move |_| {
            let text = input_ref.get().unwrap().value();
            action.dispatch(text.into());
        } class="btn">

            Submit
        </button>
        </div>

        <Transition fallback=move || view! { <p>"Loading..."</p>}>
        <p>
        The result::
        {move || match action.value().get(){
            Some(val) => val.unwrap_or("Couldn't generate fibonacci number".to_string()),
            None => "Nothing right now".to_string(),
        }}
        </p>
        </Transition>
        </div>
        </div>
    }
}
