use super::*;
#[function_component(ErrorDisplay)]
pub fn error_display() -> Html {
    let invitation_ctx =
        use_context::<InvitationCtxValue>().expect("invitation ctx should be present");

    let is_err = invitation_ctx.fetch_invite_handle().err().is_some()
        || invitation_ctx.rsvp_handle().err().is_some();

    let css = if is_err {
        "bg-red-900 px-4 text-center text-yellow-400 w-screen h-screen fixed inset-0 z-50 flex flex-col justify-center items-center text-xl"
    } else {
        "hidden"
    };

    let err_msg = {
        if let Some(e) = invitation_ctx.fetch_invite_handle().err() {
            e.to_string()
        } else if let Some(e) = invitation_ctx.rsvp_handle().err() {
            e.to_string()
        } else {
            "".to_string()
        }
    };

    html! {
        <div class={css}>
            <div>
                {"Oops, som'ting wong, please contact David for help"}
            </div>
            <div>
                {err_msg}
            </div>
        </div>
    }
}
