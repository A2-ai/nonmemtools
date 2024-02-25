use std::path::Path;

use extendr_api::prelude::*;
pub mod parsers;

#[derive(Debug, PartialEq, IntoDataFrameRow)]
struct ParameterRow {
    parameter: String,
    estimate: f64,
    stderr: Option<f64>,
    fixed: bool,
}

#[extendr]
// @export
fn parse_ext_file_impl(path: &str) -> Robj {
    let mut all_parameters: Vec<ParameterRow> = Vec::new();
    let ext_file = Path::new(path);
    if !ext_file.exists() {
        return Robj::from(format!("File does not exist: {}", path));
    }
    let ext_result = parsers::ext::parse_ext_file(ext_file);
    if ext_result.is_err() {
        return Robj::from(format!("Error parsing file: {}", ext_result.err().unwrap()));
    }
    let ext_result = ext_result.unwrap();
    let last_index = ext_result.data.len() - 1;
    let (has_stderr, std_errors) = match ext_result.data[last_index].std_errors {
        Some(_) => (true, ext_result.data[last_index].std_errors.clone().unwrap()),
        None => (false, Vec::new()),
    };
    for i in 0..ext_result.names.len() {
        let mut stderr: Option<f64> = None;
        if has_stderr {
            // nonmem sets to 1E10 if the parameter is fixed or stderr not calculated such
            // as for off diagonal elements
            if std_errors[i] != 10000000000.0 {
                stderr = Some(std_errors[i]);
            }
        }
        all_parameters.push(ParameterRow {
            parameter: ext_result.names[i].to_string(),
            estimate: ext_result.data[last_index].estimates[i],
            stderr: stderr,
            fixed: ext_result.data[last_index].fixed[i],
        });
    }
    match all_parameters.into_dataframe() {
        Ok(dataframe) => dataframe.as_robj().clone(),
        Err(err) => Robj::from(format!("Error converting to DataFrame: {}", err)),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod nonmemtools;
    fn parse_ext_file_impl;
}
