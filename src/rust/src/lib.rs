use extendr_api::prelude::*;
use extendr_api::robj::Robj;
use std::path::Path;
pub mod parsers;
use itertools::izip;
#[derive(Debug, PartialEq, IntoDataFrameRow)]
struct ParameterRow {
    parameter: String,
    estimate: f64,
    stderr: Option<f64>,
    fixed: bool,
}

fn collect_results(estimate: f64, stderr: Option<f64>, fixed: bool) -> Vec<f64> {
    let mut result = Vec::new();
    result.push(estimate);
    if let Some(stderr) = stderr {
        result.push(stderr);
    }
    if fixed {
        result.push(1.0);
    } else {
        result.push(0.0);
    }
    result
}

#[extendr]
// @export
fn parse_ext_file_wide_impl(path: &str) -> Robj {
    let ext_file = Path::new(path);
    if !ext_file.exists() {
        return Robj::from(format!("File does not exist: {}", path));
    }
    let ext_result = parsers::ext::parse_ext_file(ext_file);
    if ext_result.is_err() {
        return Robj::from(format!("Error parsing file: {}", ext_result.err().unwrap()));
    }

    let ext_result = ext_result.unwrap();
    // let mut names = vec![String::from("output")];
    // // The extend method in Rust does not return the extended vector,
    // // but instead it modifies the vector in-place. Therefore, you cannot chain it with other
    // // methods
    // names.extend(ext_result.names.iter().cloned());
    let names = ext_result.names.clone();
    let mut output_names: Vec<String> = Vec::new();
    let mut output_method: Vec<String> = Vec::new();
    let mut results: Vec<(&str, Vec<f64>)> = names
        .iter()
        .map(|name| (name.as_str(), Vec::new()))
        .collect();

    ext_result.data.iter().for_each(|table| {
        if table.std_errors.is_some() {
            let nms = vec![
                "estimate".to_string(),
                "stderr".to_string(),
                "fixed".to_string(),
            ];
            output_names.extend(
                nms
                .iter()
                .cloned(),
            );
            output_method.extend(nms.iter().map(|_| table.title.clone()).collect::<Vec<_>>());
        } else {
           let nms =  vec!["estimate".to_string(), "fixed".to_string()];
            output_names.extend(
               nms 
                    .iter()
                    .cloned(),
            );
            output_method.extend(nms.iter().map(|_| table.title.clone()).collect::<Vec<_>>());
        }
        ext_result.names.iter().enumerate().for_each(|(i, name)| {
            let std_errors = match &table.std_errors {
                Some(std_errors) => Some(std_errors[i]),
                None => None,
            };
            let result = collect_results(table.estimates[i], std_errors, table.fixed[i]);
            results[i].1.extend(result);
        });
    });
    let mut output: Vec<(&str, Robj)> = results
        .iter()
        .map(|(name, values)| (*name, values.clone().into()))
        .collect();
    output.push(("output", output_names.into()));
    output.push(("method", output_method.into()));
    let df = List::from_pairs(output);
    let classes = vec!["tbl_df", "tbl", "data.frame"];
    df.set_attrib("class", classes).unwrap();
    //df.set_attrib("row.names", vec!["1"].into_iter().collect::<Vec<_>>()).unwrap();
    df.set_attrib(
        "row.names",
        Strings::from_values((0..results[0].1.len()).map(|i| format!("{}", i + 1))),
    )
    .unwrap();
    df.into()
}

#[extendr]
// @export
fn parse_ext_file_long_impl(path: &str) -> Robj {
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
        Some(_) => (
            true,
            ext_result.data[last_index].std_errors.clone().unwrap(),
        ),
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
    fn parse_ext_file_long_impl;
    fn parse_ext_file_wide_impl;
}
