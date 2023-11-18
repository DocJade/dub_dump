use crate::Statistics;

// reference
// pub struct Statistics {
//     total_clips:usize,    // total clips ever imported this session
//     dumped_clips:usize,   // total deleted clips
//     cut_ratio:f64,      // ratio of kept to thrown away, throwing away none is 0, all is 1 (cut/total)
//     old_run_time:f64,   // original runtime of all the files
//     new_run_time:f64,   // current run time of all files
//     time_saved:f64,     // old - new
// }

// update stats by ingesting update data
#[must_use]
#[allow(clippy::pedantic)] // dont care about the FP precision lost
pub fn update_statistics(
    old_stats: Statistics,
    total_change: usize,
    dumped_change: usize,
    old_runtime_change: f64,
    new_runtime_change: f64,
) -> Statistics {
    let mut new_stats = old_stats; // copy
                                   // now update the stats based on incoming data
    new_stats.total_clips += total_change;
    new_stats.dumped_clips += dumped_change;
    new_stats.old_run_time += old_runtime_change;
    new_stats.new_run_time += new_runtime_change;

    // calculate the new cut ratio and time saved
    // precision errors? i hardly knew her
    new_stats.cut_ratio = new_stats.dumped_clips as f64 / new_stats.total_clips as f64;
    new_stats.time_saved =
        new_stats.old_run_time - (new_stats.old_run_time - new_stats.new_run_time);

    new_stats
}
