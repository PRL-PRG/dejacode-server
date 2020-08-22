use structopt::StructOpt;

use select::cachedb::CachedDatabase;

use dcd::DCD;

use examples::with_elapsed_seconds;
use examples::Configuration;
use examples::io::*;

use examples::canned_queries::Queries;

fn main() {
    let configuration = Configuration::from_args();

    eprintln!("Loading dataset at `{}`", configuration.dataset_path_as_string());
    let (dcd, loading_time) = with_elapsed_seconds!(
        DCD::new(configuration.dataset_path_as_string())
    );

    let database = CachedDatabase::from(&dcd, configuration.skip_cache);

    let queries =
        if configuration.queries.len() != 0 { configuration.queries.clone() } else { Queries::all() };

    eprintln!("Starting to execute {} queries", queries.len());
    for query in queries.iter() {
        let parameters = Queries::default_parameters(query);
        eprintln!("Executing query {} with parameters: {:?}", query, parameters);
        let (projects, query_execution_time) = with_elapsed_seconds!(
            match Queries::run(&database, query, parameters) {
                Some(projects) => projects,
                None => { eprintln!("No such query {}!", query); continue; }
            }
        );

        eprintln!("Writing results to `{}`", configuration.output_path_for_as_string(query.to_string()));
        let (_, writing_to_output_time) = with_elapsed_seconds!(
            write_to_output(&configuration, query.to_string(), &projects)
        );

        eprintln!("Elapsed time...");
        eprintln!("    {}s loading",           loading_time);
        eprintln!("    {}s query execution",   query_execution_time);
        eprintln!("    {}s writing to output", writing_to_output_time);

        eprintln!("Logging elapsed time to `{}`", configuration.timing_log_as_string());
        log_timing(&configuration, query, loading_time, query_execution_time, writing_to_output_time);
    }
    eprintln!("Done executing {} queries", queries.len());
}