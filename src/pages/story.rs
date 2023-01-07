use super::*;

#[function_component(StoryPage)]
pub fn story_page() -> Html {
    let (nav_items, default_route) = use_auth();
    html! {
        <div>
            <NavMenu<Route, UrlQuery> default_route={default_route} routes={nav_items}/>
            <div class={"flex flex-col my-[130px] px-[32px] max-w-[100%] md:px-[132px]"}>
                <div
                    class={"text-[72px] mb-[56px]"}
                >
                    {"Our Story"}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"David and Mia had their first encounter in 2018 when Mia visited the Queensland Conservatorium for lessons. Their only impressions of each other were the “afro dude” and the “try-hard Canberra girl who wanted to get into Con”."}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"Fast-forward to 2020 when Mia moved to Brisbane, they were surprised to meet each other again at UniConnect - a Christian uni students event held at Ann Street Church. “He/she is a Christian too?” They asked themselves in their befuddled minds. From there on they saw each other everywhere (unintentionally) - with their fellow piano friends, in the Con Christian group, at church…this forced them to at least remember each other’s names."}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"Having said that, the place where David and Mia got to know each other most was in the legendary virtual meeting room zoomzoom40hrs - a room that marks fond memories of Skribbl.io, Bible studies, Youtube screen-shares, and late night chats with groups of friends during COVID. Before they knew it, they had acquired a habit of sharing joys and sorrows with each other, praying for and encouraging each other. When lockdown lifted, with fear and trembling David approached Mia and said, “Um, I think I’ve caught some feelings for you.” To his shock, Mia answered, “Ok, so what shall we do?”… It took Mia 2 months to muster up the courage to commit to a relationship, with an equal degree of fear and trembling."}
                </div>
                <div
                    class={"text-[18px] mb-[12px] leading-loose"}
                >
                {"2.5 years later, the two have become the bestest friends and are trembling much less, though some things never change: David won’t give up trying to teach Mia code while she struggles to stay awake, and Mia won’t stop dragging David to come shopping with her despite his groaning protests. They can live with that for the rest of their lives."}
                </div>
            </div>
        </div>
    }
}
