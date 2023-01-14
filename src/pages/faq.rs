use super::*;

#[function_component(FaqPage)]
pub fn faq_page() -> Html {
    let (nav_items, default_route) = use_auth();

    let livestream =
        use_context::<LiveStreamService>().expect("Live stream context should be provided");

    html! {
        <div>
            <NavMenu<Route, UrlQuery> default_route={default_route} routes={nav_items}/>
            <div class={"flex flex-col my-[130px] px-[32px] max-w-[100%] md:px-[132px]"}>
                <div
                    class={"text-[72px] mb-[56px]"}
                >
                    {"FAQ"}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"What should I wear?"}
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"Our dress code is cocktail/semi-formal. Most importantly, please come in something you are comfortable in!"}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"Is there parking at the ceremony venue?"}
                </div>
                <div
                    class={"text-[18px] leading-loose"}
                >
                {"For the ceremony, there is plenty of free parking at the "}
                <a
                    href={"https://www.google.com/maps/@-27.4675328,153.0247089,3a,75y,198.87h,90.14t/data=!3m6!1e1!3m4!1sIre3LRsJgXiq5TFVTda2hg!2e0!7i16384!8i8192"}
                    class="text-blue-600 cursor-pointer"
                    target="_blank"
                >
                    {"Wilson underground carpark (145 Ann Street) next to Ann Street Church"}
                </a>
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"Please note that this carpark will be locked once the ceremony and afternoon tea conclude. You would need to move your vehicle out then if you do not have an access card."}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"Where is the reception and how do I get there?"}
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"The reception is at Transcontinental Hotel (482 George St, Brisbane City). It’s less than 10 minutes’ walk from Ann Street Church (however please do not leave your car at Ann Street if you don’t have an access card). Alternatively, the reception venue offers all day parking for a small fee."}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"Can I take photos?"}
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"Please do! You can share them on Facebook or Instagram with the hashtag #DnM2023.

The only exception is when the bridal party is walking down the aisle - we ask that you please leave this part to our professional photographer ;)"}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"What gift should I bring?"}
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"Your presence will be absolutely enough to make our day! Please don't feel obligated to bring a gift. Should you wish to bless us with a gift, however, we would greatly appreciate any monetary gifts to help us start our new life together."}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"I can’t make it physically - will there be a livestream?"}
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"Yes, the ceremony will be livestreamed. A link will (should) appear on our website on the day of the wedding! (If it doesn’t show up…click "}
                    <a
                        href={livestream.0}
                        target="_blank"
                        class="text-blue-600 cursor-pointer"
                    >
                        {"here"}
                    </a>
                {")"}
                </div>
                <div
                    class="text-[20px] font-semibold"
                >
                    {"I have more questions that aren’t answered here…"}
                </div>
                <div
                    class={"text-[18px] mb-[28px] leading-loose"}
                >
                {"Contact Mia or David!"}
                </div>
            </div>
        </div>
    }
}
