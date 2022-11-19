use super::*;
use web_sys::HtmlVideoElement;

#[function_component(Title)]
pub fn title() -> Html {
    let accept = use_state(|| false);
    let accept_set = accept.clone();

    let vid_ref = use_node_ref();

    let vid_ref_passed = vid_ref.clone();
    let vid_ref_process = vid_ref.clone();

    let on_click = Callback::from(move |_: MouseEvent| {
        accept_set.set(true);
        let element = vid_ref_process
            .cast::<HtmlVideoElement>()
            .expect("Ref should be video element");
        element.set_loop(true);
        element.play().unwrap();
    });

    html! {
        <div class="
            mt-20 w-screen h-screen max-w-full flex 
            items-center justify-center
        ">
            if !(*accept) {
                <div class="
                    w-screen fixed h-screen bg-bg z-20 top-0 flex justify-center
                    items-center
                ">
                    <button type="button"
                        onclick={on_click}
                        class="
                            py-2 px-8 bg-black text-white 
                        "
                    >{"OPEN INVITATION"}</button>
                </div>
            }
            <div class="
               flex items-center justify-center flex-col text-center max-w-md
               text-[1.125rem] overflow-x-hidden
               mb-16 px-4
            ">
                <div class="mb-4 italic">
                    {"Dear Linden and Emma,"}
                </div>
                <div class="italic">
                    {"Together with our families, we joyfully request your company at the celebration of our marriage"}
                </div>
                <div class="h-[300px]">
                    <video class="h-[200px] relative z-0" ref={vid_ref_passed}>
                        <source src="video.mp4" type="video/mp4"/>
                    </video>
                </div>
                <div class="text-5xl font-serif mb-6">
                    {"David & Mia"}
                </div>
                <div class="text-[1.125rem]">
                    {"Ann St Presbyterian"}
                </div>
                <div class="text-[1.125rem] mb-6">
                    {"11.03.2022 1PM UTC+10"}
                </div>
                <div>
                    <button type="button"
                        class="
                            p-2 bg-black text-white w-36
                        "
                    >{"RSVP"}</button>
                </div>
            </div>
        </div>
    }
}
