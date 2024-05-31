use std::sync::atomic::{AtomicUsize, Ordering};

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
                    <Route path="/home" view=Home/>
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
            <A href="home">"Home"</A>
            <A href="fib">"Fib"</A>
        </nav>
    }
}

// #[server(CalcFib, "/api")]
// pub async fn calc_fib(input: String) -> Result<u128, ServerFnError> {
//     fn fib_memo(n: u128, memo: &mut [u128; 2]) -> u128 {
//         let [a, b] = *memo;
//         let c = a.saturating_add(b);

//         if n == 0 {
//             c
//         } else {
//             *memo = [b, c];
//             fib_memo(n - 1, memo)
//         }
//     }

//     let target: u128 = input.parse().unwrap_or_default();

//     if target < 2 {
//         Ok(1)
//     } else {
//         Ok(fib_memo(target - 2, &mut [0, 1]))
//     }
// }

#[server(CalcFib, "/api")]
pub async fn calc_fib(input: String) -> Result<String, ServerFnError> {
    const MAX_RECURSION: usize = 4000;
    static FN_CALLS: AtomicUsize = AtomicUsize::new(0);

    fn fib_memo(n: usize, memo: &mut [Natural; 2]) -> Result<Natural, ServerFnError> {
        let lim = FN_CALLS.fetch_add(1, Ordering::Relaxed);
        if lim >= MAX_RECURSION {
            FN_CALLS.store(0, Ordering::Relaxed);
            Err(ServerFnError::new(
                "Max recursion limit reached. Aborting to avoid stack overflow",
            ))
        } else {
            let a = &memo[0];
            let b = &memo[1];
            let c = a + b;

            if n == 0 {
                FN_CALLS.store(0, Ordering::Relaxed);
                Ok(c)
            } else if n == 1 {
                FN_CALLS.store(0, Ordering::Relaxed);
                Ok(a.clone())
            } else {
                *memo = [b.clone(), c];
                // println!("{:?}", FN_CALLS);
                fib_memo(n - 1, memo)
            }
        }
    }

    let mut options = ToSciOptions::default();
    options.set_precision(30);
    // let what = res.to_sci_with_options(options).to_string();
    // println!("{}", res.to_sci_with_options(options));
    let target: usize = input.parse().unwrap_or_default();

    if target < 2 {
        let fib = Natural::from(1u32).to_sci_with_options(options).to_string();
        Ok(fib)
    } else {
        let zero = Natural::from(0u32);
        let one = Natural::from(1u32);
        match fib_memo(target - 2, &mut [zero, one]) {
            Ok(f) => Ok(f.to_sci_with_options(options).to_string()),
            Err(e) => Err(e),
        }
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
        // <p>
        // The result was:
        // {move || if action.value().get().is_none() {
        //     "Nothing right now".to_string()
        // } else if action.value().get().is_some() {
        //     let final_fib = action.value().get().unwrap().unwrap();
        //     {final_fib}
        // } else {
        //     format!("{:?}", "shit".to_string())
        // }
        // }
        //     </p>
        <Transition>
        <p>
        The result::
        {move || match action.value().get() {
            Some(val) => val.unwrap_or("Possible recursion limit reached".to_string()),
            None => "Nothing right now".to_string(),
        }}
        </p>
        </Transition>
    }
}
