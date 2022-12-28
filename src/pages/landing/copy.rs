fn join_names(names: Vec<String>) -> String {
    let len = names.len();
    names
        .into_iter()
        .enumerate()
        .map(|(i, e)| {
            if len > 1 {
                if i < len - 2 {
                    return format!("{}, ", e);
                }
                if i == len - 2 {
                    return format!("{} and ", e);
                }
            }
            e
        })
        .collect()
}

pub fn get_today_title() -> String {
    String::from("")
}

pub fn get_today_subtitle() -> String {
    String::from("")
}

pub fn get_coming_title() -> String {
    String::from("")
}

pub fn get_coming_subtitle() -> String {
    String::from("")
}

pub fn get_passed_title() -> String {
    String::from("")
}

pub fn get_passed_subtitle() -> String {
    String::from("")
}

pub fn get_today_invited_title(names: Vec<String>) -> String {
    format!("Dear {}.", join_names(names))
}

pub fn get_today_invited_subtitle() -> String {
    String::from("")
}

pub fn get_coming_invited_title(names: Vec<String>) -> String {
    format!("Dear {}.", join_names(names))
}

pub fn get_coming_invited_subtitle() -> String {
    String::from("Together with our families, we joyfully request your company at the celebration of our marriage")
}

pub fn get_passed_invited_title(names: Vec<String>) -> String {
    format!("Dear {}.", join_names(names))
}

pub fn get_passed_invited_subtitle(did_attend: bool) -> String {
    if did_attend {
        String::from("Thank you for celebrating our special day with us")
    } else {
        String::from("Thank you for celebrating our special day with us in spirit")
    }
}
