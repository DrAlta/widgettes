use std::cell::RefCell;

use macroquad::prelude::*;
use super::*;

//this broken in the latest macroquad
#[async_std::test]
async fn test_simple_case() {
    let font = load_ttf_font("./FreeSans.ttf").await.unwrap();

    let mut a = Wigette::new_vbox(
        10, 
        10, 
        true, 
        true,
        vec![
            Wigette::new_label(
                5, 
                5, 
                true, 
                true, 
                "text".into(), 
                RefCell::new(font), 
                10, 
                BLANK
            )
        ]
    );
    let _ = a.get_child_mut(0);
    let _ = a.get_child_mut(0);

}