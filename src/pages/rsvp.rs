mod form;
use super::*;
use form::*;

#[function_component(RSVPPage)]
pub fn rsvp_page() -> Html {
    let invitee = Invitee {
        id: "12345".to_string(),
        fname: "David".to_string(),
        lname: "Kwong".to_string(),
        rsvp: Some(true),
        dietary_requirements: "".to_string(),
    };
    html! {
        <div>
            <NavMenu<Route> routes={vec![]}/>
            <div
                class={"flex flex-col my-[159px] pl-[142px]"}
            >
                <div
                    class={"text-[72px] mb-[56px]"}
                >
                    {"RSVP"}
                </div>
                <Form invitee={invitee}/>
                <div class="h-[16px]"/>
                <div>
                    <button type="button"
                        class="
                        p-2 bg-black text-white w-36
                        rounded-full
                    "
                    >{"SUBMIT"}</button>
                </div>
            </div>
        </div>
    }
}
