//x: 80, y: 30
//    00000000001111111111222222222233333333334444444444555555555566666666667777777777
//    01234567890123456789012345678901234567890123456789012345678901234567890123456789

//00         .____        _      .____                          |     Statistics:
//01         |    \  _ _ | [_    |    \  _ _  _____ .___        | Total clips:  12345
//02         |  |  || | || . |   |  |  || | || , , || . |       | Dumped clips: 12345
//03         |____/ |___||___|   |____/ |___||_|_|_||  _|       | Cut ratio:    .0985 // logo is horizontally centered in its box
//04                                                |_|         | Old run time: 0.00H
//05   -----space for version text----------------------------- | New run time: 0.00H
//06   -----MOTD---------------56-chars-of-space--------------- | Time saved:   0.00H
//07  ----------------------------------------------------------+-------------------- // divider
//08   --total-directory-progress-bar--------------------------------------------000% // style is 1234 ##|'.'.'. 1234 //.'.' might change // index/total
//09  ------------------------------------------------------------------------------- // divider
//11  ###############################################################################
//12  ############################################################################### // extra space? what goes here?
//13  ############################################################################### // i could also reduce the app hight
//15                                                                                  // gap
//16             .                     .:####.                                        // audio clip waveform
//17          .:|#.                 .:|#######.                                       // audio clip waveform
//18      .:|######:            .:|############:                                      // audio clip waveform  .:|#
//19  -:|###########:--------:|#################:------------------------------------ // audio clip waveform  -:|#
//20      ':|######:            ':|############:                                      // audio clip waveform  ':|#
//21          ':|#'                 ':|#######'                                       // audio clip waveform
//22             '                     :|####'                                        // audio clip waveform
//23                                                                                  // gap
//24  --clip-progress-bar------------------------------------------------------------  // style is 0m00s -|----- 0m00s
//23                                                                                  // gap
//24  Volume: ???%     Speed: ???%                                                    // Volume and speed indicators
//25  ------------------------------------------------------------------------------- // divider
//26  ############################################################################### // timeline view, shows relative clip lengths to each other, current clip in middle
//27  ------------------------------------------------------------------------------- // divider
//28  space: replay    a: back    s: skip    d: dump    x: reset speed    ^c: exit    // volume / speed?
//29  up/down: volume     left/right: speed                                 [DocJade] // keybinds

// keybinds

// space: again (play sound again from beginning)
// s: skip / save
// d: dump / discard (deletes the copy)
// up: vol up
// down: vol down
// right: speed up
// left: slow down
// x: reset speed.

use crate::helper_functions::graceful_shutdown::graceful_shutdown;
use crossterm::{terminal::Clear, ExecutableCommand};
use std::{
    io::{self},
    ops::Index,
    vec,
};

use super::draw_text::{draw_text, PrintableText};

//TODO
// de-dupe some drawing logic

#[allow(clippy::pedantic)] // shut up about the line count, i know, ill fix it later // break each part into its own function
pub fn draw_static_bits(splash: String, version_text: String) {
    debug_log!("Drawing the title screen...");
    // only draw the parts of the screen that do not change

    // clear the screen
    match io::stdout().execute(Clear(crossterm::terminal::ClearType::All)) {
        Ok(_) => {}
        Err(err) => graceful_shutdown(
            &format!("[draw_static_bits] : Couldn't clear the screen! {err}"),
            1,
        ),
    };

    // draw the logo!
    draw_logo();

    // draw the splash
    draw_splash(splash);

    // now for the version number
    draw_version(version_text);

    // draw the statistics now
    draw_statics_bg();

    // draw the controls
    draw_controls();

    // draw the lines
    draw_lines();
}

fn draw_logo() {
    let logo_text_lines: Vec<String> = vec![
        "       .____        _      .____                          ".to_string(),
        "       |    \\  _ _ | [_    |    \\  _ _  _____ .___        ".to_string(),
        "       |  |  || | || . |   |  |  || | || , , || . |       ".to_string(),
        "       |____/ |___||___|   |____/ |___||_|_|_||  _|       ".to_string(),
        "                                              |_|         ".to_string(),
    ];
    for n in 0u16..5 {
        let usize_int: usize = n.into();
        let result = draw_text(&PrintableText {
            text_color: colored::Color::BrightBlue,
            message: logo_text_lines.index(usize_int).to_string(), // is this slow? who cares it only runs 4 times
            pos_x: 0,
            pos_y: n,
        });
        match result {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                &format!("[draw_static_bits] : Issue drawing logo! {err:#?}"),
                1,
            ),
        }
    }
}

fn draw_splash(splash: String) {
    // find splash center point
    // shift over by half length
    let center: u16 = match ((56/2) - (splash.len()/2)).try_into(){
        Ok(good) => {good},
        Err(err) => graceful_shutdown(&format!("[draw_static_bits] : splash text math did not fit into u16! {err:#?} : number was {}",((56/2) - (splash.len()/2))), 1),
    }; // todo, somehow prove that this can never fail and inform compiler?

    // print splash text
    match draw_text(&PrintableText {
        text_color: colored::Color::BrightGreen,
        message: splash,
        pos_x: center,
        pos_y: 6,
    }) {
        Ok(()) => {}
        Err(err) => graceful_shutdown(
            &format!("[draw_static_bits] : Issue drawing splash! {err:#?}"),
            1,
        ),
    }
}

fn draw_controls() {
    // draw controls
    // this function is haunted
    let statistics_lines: Vec<String> = vec![
        "space: replay    a: back    s: skip    d: dump    x: reset speed    ^c: exit   "
            .to_string(),
        "up/down: volume     left/right: speed                                 [DocJade]"
            .to_string(),
    ];
    for n in 0u16..2 {
        let usize_int: usize = n.into();
        let result = draw_text(&PrintableText {
            text_color: colored::Color::White,
            message: statistics_lines.index(usize_int).to_string(),
            pos_x: 0,
            pos_y: n + 28,
        });
        match result {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                &format!("[draw_static_bits] : Issue drawing statistics! {err:#?}"),
                1,
            ),
        }
    }
}

fn draw_version(version: String) {
    let center = match ((56/2) - (version.len()/2)).try_into(){
        Ok(good) => {good},
        Err(err) => graceful_shutdown(&format!("[draw_static_bits] : version text text math did not fit into u16! {err:#?} : number was {}",((56/2) - (version.len()/2))), 1),
    }; // todo, somehow prove that this can never fail and inform compiler?

    // print version text
    match draw_text(&PrintableText {
        text_color: colored::Color::BrightWhite,
        message: version,
        pos_x: center,
        pos_y: 5,
    }) {
        Ok(()) => {}
        Err(err) => graceful_shutdown(
            &format!("[draw_static_bits] : Issue drawing version! {err:#?}"),
            1,
        ),
    }
}

fn draw_statics_bg() {
    let statistics_lines: Vec<String> = vec![
        "     Statistics:    ".to_string(),
        " Total clips:  ?????".to_string(),
        " Dumped clips: ?????".to_string(),
        " Cut ratio:    ?????".to_string(),
        " Old run time: ?????".to_string(),
        " New run time: ?????".to_string(),
        " Time saved:   ?????".to_string(),
    ];
    for n in 0u16..7 {
        let usize_int: usize = n.into();
        let result = draw_text(&PrintableText {
            text_color: colored::Color::White,
            message: statistics_lines.index(usize_int).to_string(),
            pos_x: 59,
            pos_y: n,
        });
        match result {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                &format!("[draw_static_bits] : Issue drawing statistics! {err:#?}"),
                1,
            ),
        }
    }
}

fn draw_lines() {
    // draw the lines
    // lines are white
    let mut line_hights: Vec<u16> = vec![7, 9, 25, 27];
    let dash_line: String = "-".repeat(79);

    for line in line_hights {
        let result = draw_text(&PrintableText {
            text_color: colored::Color::BrightWhite,
            message: dash_line.clone(),
            pos_x: 0,
            pos_y: line,
        });
        match result {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                &format!("[draw_static_bits] : Issue drawing lines! {err:#?}"),
                1,
            ),
        }
    }

    // now draw the vertical line for statistics ( col 58 )
    line_hights = vec![0, 1, 2, 3, 4, 5, 6];
    for line in line_hights {
        let result = draw_text(&PrintableText {
            text_color: colored::Color::BrightWhite,
            message: "|".to_string(),
            pos_x: 58,
            pos_y: line,
        });
        match result {
            Ok(()) => {}
            Err(err) => graceful_shutdown(
                &format!("[draw_static_bits] : Issue drawing lines! {err:#?}"),
                1,
            ),
        }
    }

    // add decorative plus to the logo box
    match draw_text(&PrintableText {
        text_color: colored::Color::White,
        message: "+".to_string(),
        pos_x: 58,
        pos_y: 7,
    }) {
        Ok(()) => {}
        Err(err) => graceful_shutdown(
            &format!("[draw_static_bits] : Issue drawing the plus! {err:#?}"),
            1,
        ),
    }
}
