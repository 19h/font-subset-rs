pub use woff2_sys;
pub use hb_subset;

fn main() {
    let ttf_font_bytes = include_bytes!("../a.ttf");
    //let woff2_font_bytes = woff2_sys::convert_ttf_to_woff2(ttf_font_bytes, &[], 11, true).unwrap();
    //println!("WOFF2 font bytes: {:?}", woff2_font_bytes);

    let subset_font = hb_subset::subset(ttf_font_bytes, ['a']).unwrap();
    println!("Subset font bytes: {:?}", subset_font.len());
}
