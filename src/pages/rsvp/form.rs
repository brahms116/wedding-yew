use super::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlTextAreaElement};

#[derive(PartialEq, Properties)]
pub struct FormProps {
    pub invitee: Invitee,
    pub on_change: Callback<Invitee>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    {
        let invitee = props.invitee.clone();

        let on_radio_change = {
            let invitee = invitee.clone();
            let cb = props.on_change.clone();
            Callback::from(move |e: Event| {
                let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let mut invitee = invitee.clone();
                invitee.rsvp = Some(target.value() == "coming");
                cb.emit(invitee);
            })
        };

        let on_text_area_change = {
            let invitee = invitee.clone();
            let cb = props.on_change.clone();
            Callback::from(move |e: Event| {
                let mut invitee = invitee.clone();
                let target = e
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlTextAreaElement>()
                    .unwrap();
                invitee.dietary_requirements = target.value();
                cb.emit(invitee);
            })
        };

        debug!(value = invitee.rsvp.unwrap_or(false));

        html! {
            <div>
                <div
                    class="text-[36px] mb-[30px] font-semibold"
                >
                    {format!("{} {}", invitee.fname, invitee.lname)}
                </div>
                <div class="text-[28px] mb-[30px]">
                    {"Will you be joining us?"}
                </div>
                <div class="mb-[50px]">
                    <div class="mb-[21px]">
                        <input
                            type={"radio"}
                            id={format!("{}-coming", invitee.id)}
                            name={format!("{}-rsvp",invitee.id)}
                            value={"coming"}
                            checked={invitee.rsvp.unwrap_or(false)}
                            onchange={on_radio_change.clone()}
                            class="invisible absolute"
                        />
                        <label for={format!("{}-coming", invitee.id)} class="radio-label text-[18px] cursor-pointer">
                            {"Yes"}
                        </label>
                    </div>
                    <div>
                        <input
                            id={format!("{}-not-coming", invitee.id)}
                            type={"radio"}
                            name={format!("{}-rsvp",invitee.id)}
                            onchange={on_radio_change.clone()}
                            checked={!invitee.rsvp.unwrap_or(true)}
                            value={"not-coming"}
                            class="invisible absolute"
                        />
                        <label for={format!("{}-not-coming", invitee.id)} class="radio-label text-[18px] cursor-pointer">
                            {"No"}
                        </label>
                    </div>
                </div>
                <div class="text-[28px] mb-[19px]">
                    {"Dietary Requirements?"}
                </div>
                <div>
                    <textarea
                        id={format!("{}-dietary-requirements", invitee.id)}
                        value={invitee.dietary_requirements}
                        onchange={on_text_area_change}
                        class="
                            w-[100%] max-w-[400px] bg-bg h-[64px] border
                            border-solid
                            resize-none
                            rounded-[2px]
                            border-black
                            p-4
                        "
                    />
                </div>
            </div>

        }
    }
}
