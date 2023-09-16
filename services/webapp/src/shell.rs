use leptos::*;

#[component]
pub fn ApplicationShell<TNavigation, TContent, IV1, IV2>(
    navigation: TNavigation,
    content: TContent,
) -> impl IntoView
where
    TNavigation: Fn() -> IV1,
    TContent: Fn() -> IV2,
    IV1: IntoView,
    IV2: IntoView,
{
    let (menu_visible, menu_visible_set) = create_signal(false);

    view! {
      <div>
        <div
          class="relative z-50 lg:hidden"
          role="dialog"
          aria-modal="true"
          hidden=move || !menu_visible.get()
        >

          <div class="fixed inset-0 bg-gray-900/80"></div>

          <div class="fixed inset-0 flex">

            <div class="relative mr-16 flex w-full max-w-xs flex-1">

              <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
                <button
                  type="button"
                  class="-m-2.5 p-2.5"
                  on:click=move |_| menu_visible_set.set(false)
                >
                  <span class="sr-only">Close sidebar</span>
                  <svg
                    class="h-6 w-6 text-white"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      d="M6 18L18 6M6 6l12 12"
                    ></path>
                  </svg>
                </button>
              </div>

              <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-2 ring-1 ring-white/10">
                <div class="flex h-16 shrink-0 items-center">
                  <img
                    class="h-8 w-auto"
                    src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
                    alt="Your Company"
                  />
                </div>
                {navigation()}
              </div>
            </div>
          </div>
        </div>

        <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col">

          <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6">
            <div class="flex h-16 shrink-0 items-center">
              <img
                class="h-8 w-auto"
                src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
                alt="Your Company"
              />
            </div>
            {navigation()}
          </div>
        </div>

        // Верхняя строка
        <div class="sticky top-0 z-40 flex items-center gap-x-6 bg-gray-900 px-4 py-4 shadow-sm sm:px-6 lg:hidden">
          <button
            type="button"
            class="-m-2.5 p-2.5 text-gray-400 lg:hidden"
            on:click=move |_| menu_visible_set.update(|v| *v = !*v)
          >
            <span class="sr-only">ss="sr-only"</span>
            <svg
              class="h-6 w-6"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
              ></path>
            </svg>
          </button>
          <div class="flex-1 text-sm font-semibold leading-6 text-white">
            Dashboard
          </div>

        </div>

        <main class="py-10 lg:pl-72">
          <div class="px-4 sm:px-6 lg:px-8">{content()}</div>
        </main>
      </div>
    }
}
