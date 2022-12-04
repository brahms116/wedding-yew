use super::*;

#[derive(PartialEq, Properties)]
pub struct FormProps {
    pub invitee: Invitee,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    {
        let invitee = props.invitee.clone();
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
                            id={"coming"}
                            name="rsvp"
                            value={"coming"}
                            class="invisible absolute"
                        />
                        <label for="coming" class="radio-label text-[18px]">
                            {"Yes"}
                        </label>
                    </div>
                    <div>
                        <input
                            type={"radio"}
                            id={"not-coming"}
                            name="rsvp"
                            value={"not-coming"}
                            class="invisible absolute"
                        />
                        <label for="not-coming" class="radio-label text-[18px]">
                            {"No"}
                        </label>
                    </div>
                </div>
                <div class="text-[28px] mb-[19px]">
                    {"Dietary Requirements?"}
                </div>
                <div>
                    <textarea
                        id="dietary-requirements"
                        class="
                            w-[100%] max-w-[400px] bg-bg h-[84px] border
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
