use std::cmp::Ordering;

use structopt::StructOpt;
//use itertools::Itertools;

use select::selectors::sort_and_sample;
use select::meta::ProjectMeta;

use dcd::DCD;
use dcd::Project;
//use dcd::Database;

use examples::sort_by_numbers;
use examples::with_elapsed_seconds;
use examples::top;
use examples::Configuration;
use examples::Direction;
use examples::io::*;


fn main() {
    let configuration = Configuration::from_args();

    eprintln!("Loading dataset at `{}`", configuration.dataset_path_as_string());
    let (database, loading_time) = with_elapsed_seconds!(
        DCD::new(configuration.dataset_path_as_string())
    );

    eprintln!("Executing query");
    let (projects, query_execution_time) = with_elapsed_seconds!({

        let how_sort = sort_by_numbers!(Direction::Descending, |p: &Project| p.get_buggy_issue_count_or_zero());
        let how_sample = top!(50);

        sort_and_sample(&database, how_sort, how_sample)
    });

    eprintln!("Writing results to `{}`", configuration.output_path_as_string());
    let (_, writing_to_output_time) = with_elapsed_seconds!(
        write_to_output(&configuration, &projects)
    );

    eprintln!("Elapsed time...");
    eprintln!("    {}s loading",           loading_time);
    eprintln!("    {}s query execution",   query_execution_time);
    eprintln!("    {}s writing to output", writing_to_output_time);
}