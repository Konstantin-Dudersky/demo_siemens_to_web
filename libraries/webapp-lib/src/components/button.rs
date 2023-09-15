use leptos::*;

#[component]
pub fn Button<T>(label: String, on_click: T) -> impl IntoView
where
    T: Fn() -> () + 'static + Copy,
{
    view! {
        <div
            on:click=move |_| { on_click() }
            class="pointer-events-auto rounded-md bg-indigo-600 py-2 px-3 text-[0.8125rem] font-semibold leading-5 text-white hover:bg-indigo-500 m-4"
            >
            { label }
        </div>
    }
}
