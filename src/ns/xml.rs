use std::error::Error;

use crate::types::ns::{Post, RmbRoot, WaMemberRoot};

pub fn parse_wa_members(xml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    Ok(quick_xml::de::from_str::<WaMemberRoot>(xml).and_then(
        |v| Ok(v.members.split(",").map(|s| s.to_string()).collect::<Vec<String>>())
    )?)
}

pub fn parse_rmb_posts(xml: &str) -> Result<Vec<Post>, Box<dyn Error>> {
    Ok(quick_xml::de::from_str::<RmbRoot>(xml).and_then(|v| Ok(v.messages))?)
}