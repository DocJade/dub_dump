// refreshes the non static elements of the screen.

use crate::{Statistics};

use super::draw_text::{self, PrintableText, easy_draw_text};

pub fn draw_non_static(stats: &Statistics, current_index: usize) {

    // first up, draw the statistics
    easy_draw_text(format!("{: >5}",stats.total_clips.to_string()), 74, 1);
    easy_draw_text(format!("{: >5}",stats.dumped_clips.to_string()), 74, 2);
    easy_draw_text(format!("{: >5}",stats.cut_ratio.to_string()), 74, 3);
    easy_draw_text(format!("{: >5}",stats.old_run_time.round().to_string()), 74, 4);
    easy_draw_text(format!("{: >5}",stats.new_run_time.round().to_string()), 74, 5);
    easy_draw_text(format!("{: >5}",stats.time_saved.round().to_string()), 74, 6);

    // now draw the total dir progress
    let mut total_progress_bar: String = "'.".repeat(39);
    total_progress_bar.push('\''); // put final dot
    // reverse the string so we can pop off the end
    total_progress_bar = total_progress_bar.chars().rev().collect();
    // now calculate our progress
    let mut total_progress_percentage: f64 = 0.0;
    let current_float_index: f64 = current_index as f64;
    total_progress_percentage = (stats.dumped_clips as i64 - stats.total_clips as i64) as f64;
    total_progress_percentage = current_float_index / total_progress_percentage;

    let pop_amount: u16 = (79 as f64 * total_progress_percentage).floor() as u16;
    // pop off the dots
    for _ in 0..pop_amount+1 {
        _ = total_progress_bar.pop();
    }
    // add the vert line
    total_progress_bar = total_progress_bar.chars().rev().collect();
    total_progress_bar.push('|');

    // progress bar now looks something like this
    // '.'.'.'.'.'.'.|
    // TODO add the percentage, for now lets just print the bar
    // clear the old bar
    easy_draw_text(" ".repeat(79), 0, 8);
    easy_draw_text(total_progress_bar, 0, 8);

}
