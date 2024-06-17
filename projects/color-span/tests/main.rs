use std::{fs::File, io::Write};

use serde_json::from_str;

use color_span::{
    html::{HtmlWriter, ONE_DARK},
    ColorView,
};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
pub fn test_deserialize() {
    let mut view = ColorView::from("public static class G {}");
    view.dye_offset(0, 6, "keyword");
    view.dye_offset(7, 13, "keyword");
    // assert_eq!(serde_json::to_string(&overlap).unwrap(), include_str!("keyword.json"));
    assert_eq!(view, from_str(include_str!("keyword.json")).unwrap())
}

#[test]
pub fn test_html() {
    let mut html = HtmlWriter::default();
    html.style = Some(ONE_DARK.to_string());
    let mut out = "".to_string();
    let mut view = ColorView::from("public static class G {}");
    view.dye_offset(0, 6, "keyword");
    view.dye_offset(7, 13, "keyword");
    html.write_fmt(&mut out, &view).unwrap();
    let mut file = File::create("tests/keyword.html").unwrap();
    file.write_all(out.as_bytes()).unwrap()
}
