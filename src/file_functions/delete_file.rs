// delete the provided file

use std::{fs, ops::Index};

use crate::helper_functions::graceful_shutdown::graceful_shutdown;

#[must_use]
pub fn delete_file(mut file_list: Vec<String>, index: usize) -> Vec<String> {
    // grab string of thing to delete
    let delete_this = file_list.index(index);
    debug_log!("Deleting {delete_this} now...");
    match fs::remove_file(delete_this) {
        Ok(_) => {}
        Err(err) => graceful_shutdown(
            &format!("[delete_file] : Unable to delete file! : {err}"),
            1,
        ),
    }
    debug_log!("Deleted!");
    debug_log!("Removing Vec index...");
    file_list.remove(index);
    debug_log!("Done!");
    debug_log!("File deleted successfully!");
    file_list
}
