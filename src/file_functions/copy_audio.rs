//todo

// lets clone all of the source folder into a new folder in that folder
use crate::graceful_shutdown;
use fs_extra::dir::copy; // recursive copy
use fs_extra::dir::CopyOptions;
use std::fs::create_dir;
use std::path::Path;

#[must_use]
pub fn copy_audio(master: String) -> String {
    // create the new folder

    // move up a dir from audio folder
    let path = Path::new(&master);
    let parent_path = path.parent().map_or_else(
        || {
            graceful_shutdown(
                "[copy_audio] : no folder above current!"
                    .to_string()
                    .as_str(),
                1,
            )
        },
        |a| a,
    );

    // into string
    let up_path: String = parent_path.to_str().map_or_else(|| graceful_shutdown(
            "[copy_audio] : could not cast path to string!"
                .to_string()
                .as_str(),
            1,
        ), std::string::ToString::to_string);

    // new folder name
    let new: String = format!("{up_path}\\Dub_dump\\");

    match create_dir(&new) {
        Ok(_) => {}
        Err(err) => graceful_shutdown(
            format!("[copy_audio] : error creating new folder: {err:#?}").as_str(),
            1,
        ),
    }
    let mut options: CopyOptions = CopyOptions::new(); //Initialize default values for CopyOptions
    options  = options.content_only(true);
    
    // copy the directory
    match copy(master, &new, &options) {
        Ok(_) => {}
        Err(err) => graceful_shutdown(
            format!("[copy_audio] : error copying files to new folder: {err:#?}").as_str(),
            1,
        ),
    }
    // copy done! return the new dir
    new
}
