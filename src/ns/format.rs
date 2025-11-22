lazy_static::lazy_static! {
    static ref LOWERCASE_WORDS: Vec<&'static str> = vec![
        "a","an","the","and","but","or","for","nor","on","at","to","in","of"
    ];
}

pub fn prettify_name(name: &str) -> String {
    let words: Vec<String> = name.replace("_", " ").split(" ").map(
        |word| {
            let mut copy = word.to_owned();
            if LOWERCASE_WORDS.contains(&copy.as_str()) { copy } else {
                if let Some(ch) = copy.get_mut(0..1) {
                    ch.make_ascii_uppercase();
                }
                copy
            }
        }
    ).collect();

    words.join(" ")
}

pub fn canonicalize_name(name: &str) -> String {
    name.replace(" ", "_").to_ascii_lowercase()
}

pub fn nation_link(nation: &str) -> String {
    format!("https://www.nationstates.net/nation={}", canonicalize_name(nation))
}

pub fn region_link(region: &str) -> String {
    format!("https://www.nationstates.net/region={}", canonicalize_name(region))
}

pub fn rmb_link(region: &str, postid: &str) -> String {
    format!(
        "https://www.nationstates.net/page=display_region_rmb/region={}?postid={}#p{}", 
        canonicalize_name(region), postid, postid
    )
}