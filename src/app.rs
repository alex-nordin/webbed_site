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
    <div class="mt-20">
            <main class="my-auto mx-auto max-w-3xl text-center">
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
            </div>
        }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
          <nav class="bg-white dark:bg-gray-900 fixed w-full z-20 top-0 start-0 border-b border-gray-200 dark:border-gray-600">
          <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
              <div class="items-center justify-between hidden w-full md:flex md:w-auto md:order-1" id="navbar-sticky">
                <ul class="flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                  <li>
                    <A href="" class="block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500">"Home"</A>
                  </li>
                  <li>
                    <A href="fib" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 md:dark:hover:text-blue-500 dark:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700">"Fib"</A>
                  </li>
                </ul>
              // <A href="">"Home"</A>
              // <A href="fib">"Fib"</A>
              </div>
          </div>
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
        <h3 class="text-center p-6 text-4xl">"Generate nth number in fibonacci sequence"</h3>
        <div class="grid justify-center mt-5">
        // <p>
        // "Some server functions are conceptually \"mutations,\", which change something on the server. "
        // "These often work well as actions."
        // </p>
        <div class="flex justify-center text-left text-xl py-8">
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
