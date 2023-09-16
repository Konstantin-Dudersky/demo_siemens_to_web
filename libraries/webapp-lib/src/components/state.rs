use leptos::*;

// TODO - доделать цвета состояний
#[component]
pub fn State(
    text: Signal<String>,
    state_inactive: Signal<bool>,
    state_active: Signal<bool>,
    state_warning: Signal<bool>,
    state_error: Signal<bool>,
) -> impl IntoView {
    view! {
        <span class="inline-flex items-center rounded-md px-2 py-1 text-sm font-semibold ring-1 ring-inset ring-gray-500/10"

        class=("bg-grey-50", state_inactive)
        class=("text-gray-600", state_inactive)

        class=("bg-green-50", state_active)
        class=("text-green-700", state_active)

        class=("bg-green-50", state_warning)
        class=("text-green-700", state_warning)

        class=("bg-green-50", state_error)
        class=("text-green-700", state_error)
        >
            { text }
        </span>
    }
}
