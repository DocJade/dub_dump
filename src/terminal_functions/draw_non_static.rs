// refreshes the non static elements of the screen.

use std::io::Write;

use crate::Statistics;

use super::draw_text::easy_draw_text;

#[allow(clippy::cast_precision_loss)] // no way we will ever have > 4,503,599,600,000,000 files to index.
pub fn draw_non_static(stats: &Statistics, current_index: usize) {
    // first up, draw the statistics
    easy_draw_text(format!("{: >5}", stats.total_clips.to_string()), 74, 1);
    easy_draw_text(format!("{: >5}", stats.dumped_clips.to_string()), 74, 2);
    // a little extra work for the cut percentage
    // round it 
    let percent_round = (stats.cut_ratio * 100.0).round() / 100.0;
    let cut_percent = format!("{percent_round:.2}");
    easy_draw_text(format!("{cut_percent: >5}"), 74, 3);
    easy_draw_text(
        format!("{: >5}", stats.old_run_time.round().to_string()),
        74,
        4,
    );
    easy_draw_text(
        format!("{: >5}", stats.new_run_time.round().to_string()),
        74,
        5,
    );
    easy_draw_text(
        format!("{: >5}", stats.time_saved.round().to_string()),
        74,
        6,
    );

    // now draw the total dir progress
    let mut total_progress_bar: String = "'.".repeat(39);
    total_progress_bar.push('\''); // put final dot
                                   // reverse the string so we can pop off the end
    total_progress_bar = total_progress_bar.chars().rev().collect();
    // now calculate our progress
    let mut total_progress_percentage: f64;
    let current_float_index: f64 = current_index as f64;
    total_progress_percentage = (stats.dumped_clips - stats.total_clips) as f64;
    total_progress_percentage = current_float_index / total_progress_percentage;
    total_progress_percentage *= -1.0;

    #[allow(clippy::cast_possible_truncation)] // there is not any reasonable scenario where this would be an issue
    #[allow(clippy::cast_sign_loss)] // dealing with positive numbers only anyways.
    let pop_amount: u16 = (79_f64 * total_progress_percentage).floor() as u16;
    // pop off the dots
    for _ in 0..=pop_amount {
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
    // flush to make sure we clear then print
    _ = std::io::stdout().flush();
    easy_draw_text(total_progress_bar, 0, 8);
    _ = std::io::stdout().flush();
}
