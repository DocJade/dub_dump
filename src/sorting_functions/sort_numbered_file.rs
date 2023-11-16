// take in the list of files and spit out a sorted version

use std::path::Path;

use crate::helper_functions::graceful_shutdown::graceful_shutdown;
use permutation;

#[must_use]
pub fn sort_numbered_files(list: &Vec<String>) -> Vec<String> {
    // extract numbers from the clip title

    // now we shall loop over our file list, and sort them accordingly

    // new vec for the numbers
    let mut numbers: Vec<i32> = vec![];

    for file in list.clone() {
        // get its number, and append to the number vec
        numbers.push(extract_numeric_part(&file));
    }
    // reverse the vec

    // now sort the original list with the number list
    let permutation = permutation::sort(&numbers);
    // lots of debugging incase stuff breaks
    debug_println!("list");
    debug_println!("{:#?}", &list);
    debug_println!("numbers");
    debug_println!("{:#?}", &numbers);
    debug_println!("number permutation applied");
    debug_println!("{:#?}", permutation.apply_slice(&numbers));
    debug_println!("list permutation applied");
    debug_println!("{:#?}", permutation.apply_slice(list));
    debug_println!("permutation");
    debug_println!("{:#?}", permutation);
    permutation.apply_slice(list)
}

fn extract_numeric_part(filename: &str) -> i32 {
    // string for numbers
    let mut number: String = String::new();

    // remove the file extension.
    let path = Path::new(filename);

    let removing_extension = path.file_stem().map_or_else(
        || {
            graceful_shutdown(
                "[extract_numeric_part] : no file name from path"
                    .to_string()
                    .as_str(),
                1,
            )
        },
        |s| s,
    );

    debug_println!("{:#?}", removing_extension);

    let removed_extension = removing_extension.to_str().map_or_else(
        || {
            graceful_shutdown(
                "[extract_numeric_part] : could not convert file name back to str"
                    .to_string()
                    .as_str(),
                1,
            )
        },
        |s| s,
    );

    for letter in removed_extension.chars().rev() {
        // work back to front
        if letter.is_numeric() {
            number.push(letter);
        }
        continue;
    }
    // now reverse the string
    number = number.chars().rev().collect::<String>();
    // cast to number
    match number.parse::<i32>() {
        Ok(final_num) => final_num,
        Err(err) => graceful_shutdown(
            format!("[extract_numeric_part] : Error casting to i32 : {err:#?}").as_str(),
            1,
        ),
    }
}

// test that mf

#[test]
fn sort_numbered_files_test_1() {
    // Test input
    let input_files = vec![
        "t-10.mp3".to_string(),
        "t-8.mp3".to_string(),
        "t-9.mp3".to_string(),
        "t-11.mp3".to_string(),
        "t-12.mp3".to_string(),
    ];

    // Expected output
    let expected_sorted_files = vec![
        "t-8.mp3".to_string(),
        "t-9.mp3".to_string(),
        "t-10.mp3".to_string(),
        "t-11.mp3".to_string(),
        "t-12.mp3".to_string(),
    ];

    // Sort the input files
    let sorted_files = sort_numbered_files(&input_files);

    // Assert that the sorted files match the expected output
    assert_eq!(sorted_files, expected_sorted_files);
}
#[test]
fn sort_numbered_files_test_2() {
    // Test input
    let input_files = vec![
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-981.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-982.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-983.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-984.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-985.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-986.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-987.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-988.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-989.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-99.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-990.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-991.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-992.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-993.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-994.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-995.mp3".to_string(),
    ];

    // Expected output
    let expected_sorted_files = vec![
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-99.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-981.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-982.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-983.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-984.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-985.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-986.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-987.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-988.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-989.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-990.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-991.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-992.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-993.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-994.mp3".to_string(),
        "Q:\\\\Dub_dump\\VO_PnumaticMadness-995.mp3".to_string(),
    ];

    // Sort the input files
    let sorted_files = sort_numbered_files(&input_files);

    // Assert that the sorted files match the expected output
    assert_eq!(sorted_files, expected_sorted_files);
}
