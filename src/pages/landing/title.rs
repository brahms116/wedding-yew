use super::*;

#[function_component(Title)]
pub fn title() -> Html {
    let history = use_history().unwrap();

    let on_click = Callback::from(move |_| history.push(Route::RSVP));

    html! {
        <div class="w-screen min-h-screen max-w-full mb-48">
            <div class="flex flex-col w-full">
                <div class="flex flex-col text-center w-full justify-center items-center h-96">
                    {"Video here"}
                </div>
                <div class="flex flex-col text-center w-full items-center">
                    <div class="text-[4rem] mt-8">
                        {"Mia & David"}
                    </div>
                    <div class="text-2xl">
                        {"23.03.2022 13:00 utc+10"}
                    </div>
                    <div class="mt-8">
                        <button class="bg-black text-white px-4 py-2 \
                                        hover:bg-slate-800 min-w-[6rem]
                        "
                            onclick={on_click}
                        >
                            {"RSVP"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
