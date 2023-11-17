use super::graceful_shutdown::graceful_shutdown;
use std::ops::Index;
use rand::random;

#[must_use]
pub fn pick_splash(splashes: &[u8]) -> String {

    let split_splashes: Vec<&str> = match std::str::from_utf8(splashes) {
        Ok(text) => {
            text.split('\n').collect()
        },
        Err(err) => graceful_shutdown(
            format!("[main] : error getting splashes! : {err:#?}").as_str(),
            1,
        ),
    };
    // now remove the quotes
    let mut final_splashes: Vec<String> = vec![];
    for splash in split_splashes {
        final_splashes.push(splash.replace('"', ""));
    }
    // now we shall pick a random splash
    return final_splashes.index(random::<usize>() % final_splashes.len()).to_string();
}

#[test]
fn pick_splash_test_no_quotes(){
    // make sure the function is discarding quotes correctly
    let quote_boi: &[u8] = b"\"this is a test quote!\"";
    assert_eq!(pick_splash(quote_boi), "this is a test quote!");
}
#[test]
fn pick_splash_proper_line_split(){
    // make sure the function is splitting quotes correctly
    let quote_boi: &[u8] = b"\"this is a test quote!\"\n\"this too!\"";
    
    // make sure there's actually a splash in there
    assert!(!pick_splash(quote_boi).is_empty());
    // make sure there isnt a newline character in there
    assert!(!pick_splash(quote_boi).contains('\n'));
}