use leptos::*;
use leptos_router::*;

#[component]
pub fn GlobalNavigation() -> impl IntoView {
    view! {
      <nav class="flex flex-1 flex-col">
        <ul role="list" class="flex flex-1 flex-col gap-y-7">
          <li>
            <ul role="list" class="-mx-2 space-y-1">
              <li>
                <A
                  href="/"
                  class="bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                >
                  <i class="fa-solid fa-home h-6 w-6 shrink-0"></i>
                  Главная
                </A>

              </li>
              <li>
                <A
                  href="/app2"
                  class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                >

                  <i class="fa-solid fa-gears h-6 w-6 shrink-0"></i>
                  Настройки
                </A>
              </li>

            </ul>
          </li>

        </ul>
      </nav>
    }
}
