#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::{fs, io::BufWriter};
#[macro_use]
extern crate prettytable;
use prettytable::Table;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TimingResult {
    timing_type: String,
    timing_framework: String,
    final_timing: TraceFileTimings,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TraceFileTimings {
    total_dur: i64,
    click_dur: i64,
    render_during_click: i64,
    render_after_click: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TraceDataObj {
    #[serde(alias = "type")]
    the_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TraceArgs {
    data: Option<TraceDataObj>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TraceData {
    args: Option<TraceArgs>,
    cat: Option<String>,
    name: Option<String>,
    ph: Option<String>,
    pid: Option<i64>,
    tid: Option<i64>,
    ts: Option<i64>,
    dur: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Trace {
    #[serde(alias = "traceEvents")]
    trace_events: Vec<TraceData>,
}

fn main() {
    let start = Instant::now();
    let framework_directories = fs::read_dir("../traces/".to_owned()).unwrap();

    let trace_timing_results_per_framework: Vec<TimingResult> = framework_directories
        .flat_map(|framework_dir_entry| {
            let framework_directory_buf =
                framework_dir_entry.expect("no framework directory").path();
            let framework = framework_directory_buf
                .to_str()
                .unwrap()
                .split('/')
                .collect::<Vec<&str>>()
                .pop()
                .unwrap();
            let metric_directories = fs::read_dir(framework_directory_buf.clone()).unwrap();

            let timing_results_per_metric: Vec<TimingResult> = metric_directories
                .map(|metric_dir_entry| {
                    let metric_dir_buf = metric_dir_entry.expect("no metric directory").path();
                    let metric = metric_dir_buf
                        .to_str()
                        .unwrap()
                        .split('/')
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap();
                    let file_paths = fs::read_dir(metric_dir_buf.clone()).unwrap();

                    let trace_file_timings_per_file: Vec<TraceFileTimings> = file_paths
                        .map(|path| {
                            let path_entry = path.unwrap();
                            let path_buf = path_entry.path();
                            calc_event_trace(get_trace_file(path_buf.to_str().unwrap()))
                        })
                        .collect();

                    get_trace_timing_result(
                        trace_file_timings_per_file,
                        metric.to_owned(),
                        framework.to_owned(),
                    )
                })
                .collect();
            timing_results_per_metric
        })
        .collect();

    make_tables(&trace_timing_results_per_framework);
    create_json_file(&trace_timing_results_per_framework);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn create_json_file(trace_timing_results: &Vec<TimingResult>) {
    let writer = BufWriter::new(File::create("trace_results.json").unwrap());
    serde_json::to_writer_pretty(writer, &trace_timing_results).unwrap();
}

fn make_tables(trace_timing_results: &Vec<TimingResult>) {
    let header_row = row![
        "Framework",
        "Metric",
        "Click Duration",
        "Render During Click",
        "Render After Click",
        "Total Duration"
    ];

    let mut create_k_table = Table::new();
    let mut create_ten_k_table = Table::new();
    let mut clear_k_table = Table::new();
    let mut clear_ten_k_table = Table::new();

    create_k_table.add_row(header_row.clone());
    create_ten_k_table.add_row(header_row.clone());
    clear_k_table.add_row(header_row.clone());
    clear_ten_k_table.add_row(header_row.clone());

    for result in trace_timing_results {
        match result.timing_type.as_str() {
            "k" => {
                create_k_table.add_row(row![
                    result.timing_framework,
                    result.timing_type,
                    result.final_timing.click_dur.to_string(),
                    result.final_timing.render_during_click.to_string(),
                    result.final_timing.render_after_click.to_string(),
                    result.final_timing.total_dur.to_string(),
                ]);
            }
            "ten-k" => {
                create_ten_k_table.add_row(row![
                    result.timing_framework,
                    result.timing_type,
                    result.final_timing.click_dur.to_string(),
                    result.final_timing.render_during_click.to_string(),
                    result.final_timing.render_after_click.to_string(),
                    result.final_timing.total_dur.to_string(),
                ]);
            }
            "clear-k" => {
                clear_k_table.add_row(row![
                    result.timing_framework,
                    result.timing_type,
                    result.final_timing.click_dur.to_string(),
                    result.final_timing.render_during_click.to_string(),
                    result.final_timing.render_after_click.to_string(),
                    result.final_timing.total_dur.to_string(),
                ]);
            }
            "clear-ten-k" => {
                clear_ten_k_table.add_row(row![
                    result.timing_framework,
                    result.timing_type,
                    result.final_timing.click_dur.to_string(),
                    result.final_timing.render_during_click.to_string(),
                    result.final_timing.render_after_click.to_string(),
                    result.final_timing.total_dur.to_string(),
                ]);
            }
            _ => (),
        }
    }
    create_k_table.printstd();
    create_ten_k_table.printstd();
    clear_k_table.printstd();
    clear_ten_k_table.printstd();
}

fn get_trace_timing_result(
    mut timings: Vec<TraceFileTimings>,
    timing_type: String,
    timing_framework: String,
) -> TimingResult {
    timings.sort_by(|a, b| a.total_dur.cmp(&b.total_dur));
    timings.truncate(10);

    let k_trace_timing_total = timings.iter().fold(
        TraceFileTimings {
            total_dur: 0,
            click_dur: 0,
            render_during_click: 0,
            render_after_click: 0,
        },
        |acc, x| TraceFileTimings {
            total_dur: acc.total_dur + x.total_dur,
            click_dur: acc.click_dur + x.click_dur,
            render_during_click: acc.render_during_click + x.render_during_click,
            render_after_click: acc.render_after_click + x.render_after_click,
        },
    );

    let divisor = timings.len() as i64;

    let final_timing = TraceFileTimings {
        total_dur: k_trace_timing_total.total_dur / divisor,
        click_dur: k_trace_timing_total.click_dur / divisor,
        render_during_click: k_trace_timing_total.render_during_click / divisor,
        render_after_click: k_trace_timing_total.render_after_click / divisor,
    };

    TimingResult {
        timing_type,
        timing_framework,
        final_timing,
    }
}

fn get_trace_file(path: &str) -> Trace {
    let json_file_path = Path::new(path);
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized: Trace = serde_json::from_reader(json_file).expect("error while reading json");
    deserialized
}

fn calc_event_trace(trace: Trace) -> TraceFileTimings {
    fn is_render_event(ev: &str) -> bool {
        ev == "Layout"
            || ev == "UpdateLayoutTree"
            || ev == "UpdateLayerTree"
            || ev == "Paint"
            || ev == "CompositeLayers"
    }

    let entries: Vec<TraceData> = trace
        .trace_events
        .iter()
        .filter(|item| {
            if let Some(x) = item.args.clone() {
                if let Some(y) = x.data {
                    if let Some(t) = y.the_type {
                        if t == "click" {
                            return true;
                        }
                    }
                }
            }
            if let Some(n) = item.name.clone() {
                if is_render_event(&n) {
                    return true;
                }
            }
            false
        })
        .map(|item| item.to_owned())
        .collect();

    let mut click_iter = entries.iter().filter(|item| {
        if let Some(x) = item.args.clone() {
            if let Some(y) = x.data {
                if let Some(t) = y.the_type {
                    if t == "click" {
                        return true;
                    }
                }
            }
        }

        false
    });

    // Right now we always want the last click event
    // because of creating or creating and then clearing.

    let click = match click_iter.next_back() {
        Some(c) => c,
        None => panic!("no click found "),
    };

    let click_start_time = click.ts.unwrap();
    let click_time_end = click_start_time + click.dur.unwrap();

    let entries_during_click: Vec<&TraceData> = entries
        .iter()
        .filter(|item| {
            if let Some(n) = item.name.clone() {
                if is_render_event(&n)
                    && item.ts.unwrap() >= click_start_time
                    && item.ts.unwrap() <= click_time_end
                {
                    return true;
                }
            }
            false
        })
        .collect();

    let render_during_click = entries_during_click
        .iter()
        .fold(0, |acc, x| acc + x.dur.unwrap());

    let entries_after_click: Vec<&TraceData> = entries
        .iter()
        .filter(|item| {
            if let Some(n) = item.name.clone() {
                if is_render_event(&n) && item.ts.unwrap() > click_time_end {
                    return true;
                }
            }
            false
        })
        .collect();

    let render_after_click = entries_after_click
        .iter()
        .fold(0, |acc, x| acc + x.dur.unwrap());

    let click_dur = click.dur.unwrap();
    let total_dur = click_dur + render_after_click;

    TraceFileTimings {
        total_dur,
        click_dur,
        render_during_click,
        render_after_click,
    }
}

#[cfg(test)]
#[test]
pub fn calc_event_trace_is_correct() {
    //  ev == "Layout"
    //             || ev == "UpdateLayoutTree"
    //             || ev == "UpdateLayerTree"
    //             || ev == "Paint"
    //             || ev == "CompositeLayers"
    let args = Some(TraceArgs {
        data: Some(TraceDataObj {
            the_type: Some("click".to_owned()),
        }),
    });
    let click_data = TraceData {
        cat: None,
        args: args.clone(),
        name: Some("Event".to_owned()),
        ph: None,
        pid: None,
        tid: None,
        ts: Some(100),
        dur: Some(50),
    };

    let layer_data_during = TraceData {
        cat: None,
        args: None,
        name: Some("UpdateLayerTree".to_owned()),
        ph: None,
        pid: None,
        tid: None,
        ts: Some(100),
        dur: Some(25),
    };

    let layout_data_after = TraceData {
        cat: None,
        args: None,
        name: Some("UpdateLayoutTree".to_owned()),
        ph: None,
        pid: None,
        tid: None,
        ts: Some(151),
        dur: Some(50),
    };

    let layout_data_during = TraceData {
        cat: None,
        args: None,
        name: Some("Layout".to_owned()),
        ph: None,
        pid: None,
        tid: None,
        ts: Some(100),
        dur: Some(25),
    };

    let paint_data_after = TraceData {
        cat: None,
        args: None,
        name: Some("Paint".to_owned()),
        ph: None,
        pid: None,
        tid: None,
        ts: Some(151),
        dur: Some(50),
    };

    let trace = Trace {
        trace_events: vec![
            click_data.clone(),
            layout_data_during.clone(),
            paint_data_after.clone(),
            layer_data_during.clone(),
            layout_data_after.clone(),
        ],
    };
    let calc = calc_event_trace(trace.clone());
    assert_eq!(calc.total_dur, 150);
    assert_eq!(calc.click_dur, 50);
    assert_eq!(calc.render_during_click, 50);
    assert_eq!(calc.render_after_click, 100);

    let more_click_data = TraceData {
        cat: None,
        args: args.clone(),
        name: Some("Event".to_owned()),
        ph: None,
        pid: None,
        tid: None,
        ts: Some(175),
        dur: Some(75),
    };

    let trace = Trace {
        trace_events: vec![
            click_data.clone(),
            layout_data_during.clone(),
            paint_data_after.clone(),
            layer_data_during.clone(),
            layout_data_after.clone(),
            more_click_data.clone(),
        ],
    };

    let calc = calc_event_trace(trace.clone());
    assert_eq!(calc.total_dur, 75);
    assert_eq!(calc.click_dur, 75);
    assert_eq!(calc.render_during_click, 0);
    assert_eq!(calc.render_after_click, 0);
    // Maybe all I really need to do is calulate the time from start click start to last composite layer?
    // Monitor other frameworks to try it out. Maybe other frameworks are doing alot of work in the browser if
    // There is multiple browser events layered on top of each other?
    // But finding the final composite layer might be a better method.
}
