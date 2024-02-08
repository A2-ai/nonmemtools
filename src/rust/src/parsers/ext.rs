use std::fs::File;
use std::path::Path;
use std::{
    error,
    io::{self, BufRead, BufReader},
};

fn parse_title(line: &str) -> Vec<String> {
    let mut result = line
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    result.remove(0);
    result.remove(result.len() - 1);
    result
}

fn parse_to_int(line: &str) -> Vec<i32> {
    // need to cast first to float since oculd have various scientific style notations
    // see the test cases for example
    let mut result = line
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap_or(-999_999_999.0) as i32)
        .collect::<Vec<i32>>();
    result.remove(0);
    result.remove(result.len() - 1);
    result
}

fn parse_float(line: &str) -> Vec<f64> {
    // need to cast first to float since oculd have various scientific style notations
    // see the test cases for example
    let mut result = line
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap_or(-999_999_999.0))
        .collect::<Vec<f64>>();
    // remove the first and last values since they are the iteration number and objective function
    result.remove(0);
    result.remove(result.len() - 1);
    result
}

#[derive(Debug, PartialEq)]
pub struct TerminationStatus {
    status: i32,
    codes: Vec<i32>,
}

pub fn parse_error_codes(line: &str) -> TerminationStatus {
    // need to cast first to float since oculd have various scientific style notations
    // see the test cases for example
    let parsed = parse_to_int(line);
    TerminationStatus {
        status: parsed[0],
        codes: parsed[1..]
            .iter()
            .filter(|&x| x.is_positive())
            .copied()
            .collect(),
    }
}

// notes originally in https://github.com/metrumresearchgroup/bbi/blob/835953efed10fff2d7024ac1c3dd88949b268c05/parsers/nmparser/read_ext.go

//ParseParamsExt returns the ExtData in the structure of final parameter estimates
// the parameter names correspond to the names per the ext file (THETA1, THETA2, etc)
// per nonmem 7.4 the following information will be grabbed
// 1) Theburn-in iterations of the MCMCBayesian analysis are given negative values,starting at –NBURN, the number of burn-in iterations requested by the user. These are followed by positive iterations of the stationary phase.
// 2) The stochastic iterations of the SAEM analysis are given negative values. These are followed by positive iterations of the accumulation phase.
// 3) Iteration -1000000000 (negative one billion) indicates that this line contains the final result (thetas, omegas, and sigmas, and objective function) of the particular analysis. For BAYES analysis, this is the mean of the non-negative iterations (stationary samples) listed before it.
// 4) Iteration -1000000001 indicates that this line contains the standard errors of the final population parameters. For BAYES, it is the sample standard deviation of the stationary samples.
// 5) Iteration -1000000002 indicates that this line contains the eigenvalues of the correlation matrix of the variances of the final parameters.
// 6) Iteration -1000000003 indicates that this line contains the condition number , lowest, highest, Eigen values of the correlation matrix of the variances of the final parameters.
// 7) Iteration -1000000004 indicates this line contains the OMEGA and SIGMA elements in
// standard deviation/correlation format
// 8) Iteration-1000000005 indicates this linecontainsthestandarderrorstotheOMEGAand
// SIGMA elements in standard deviation/correlation format
// 9) Iteration -1000000006 indicates 1 if parameter was fixed in estimation, 0 otherwise.
// 10) Iteration -1000000007 lists termination status (first item) followed by termination codes.
// See I.54 $EST: Additional Output Files Produced under root.xml (NM72) for interpreting the codes.
// nm741.doc 174 of 284
// NONMEM Users Guide: Introduction to NONMEM 7.4.1
// 11) Iteration -1000000008 lists the partial derivative of the likelihood (-1/2 OFV) with respect to each estimated parameter. This may be useful for using tests like the Lagrange multiplier test.
// 12) Additional special iteration number lines may be added in future versions of NONMEM.

// additional notes from intro7 nm75 pdf
// In NM73, termination_textmsgs catalogs termination text messages by number, which can be
// mapped to ..\source\textmsgs.f90.
// In nm73, termination status catalogs the error status:
// For traditional analyses, an error number is listed. If negative, the analysis was user-interrupted
// For EM/Bayes analysis, error numbers map as follows (changed for nm75):
// 0,8: optimization was completed
// 1,9: optimization not completed (ran out of iterations)
// 2,10: optimization was not tested for convergence
// 3,11: optimization was not tested for convergence and was user interrupted
// 4,12: optimization was not completed and was user interrupted
// 16,24: objective function is infinite or all individual objective fuctions are zero. problem ended
// 32,40: All individual objective fuctions are zero. problem ended
// 8,9,10,11,12,24,40: reduced stochastic/sationary portion was not completed prior to user
// interrupt

#[derive(Debug, PartialEq)]
pub struct ExtTableResultsOutput {
    pub title: String,
    pub names: Vec<String>,
    pub estimates: Vec<f64>,
    // TODO: determine if this is always present in the ext file??
    pub std_errors: Option<Vec<f64>>,
    pub eigenvalues: Option<Vec<f64>>,
    pub condition_number: f64,
    pub fixed: Vec<bool>,
    pub termination_status: TerminationStatus,
    pub partial_derivative: Option<Vec<f64>>,
}

pub fn parse_ext_file(file: &Path) -> Result<ExtTableResultsOutput, Box<dyn error::Error>> {
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    let mut output = ExtTableResultsOutput {
        title: String::new(),
        names: Vec::new(),
        estimates: Vec::new(),
        std_errors: None,
        eigenvalues: None,
        condition_number: 0.0,
        fixed: Vec::new(),
        termination_status: TerminationStatus {
            status: 0,
            codes: Vec::new(),
        },
        partial_derivative: None,
    };
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        if line.starts_with(" ITER") {
            output.names = parse_title(&line)
        } else if line.starts_with("TABLE") {
            output.title = line.clone()
        } else if line.starts_with("  -1000000000") {
            output.estimates = parse_float(&line);
        } else if line.starts_with("  -1000000001") {
            let std_errors = parse_float(&line);
            output.std_errors = Some(std_errors);
        } else if line.starts_with("  -1000000002") {
            output.eigenvalues = Some(parse_float(&line));
        } else if line.starts_with("  -1000000003") {
            output.condition_number = parse_float(&line)[1];
        } else if line.starts_with("  -1000000004") {
            continue;
        } else if line.starts_with("  -1000000005") {
            continue
        } else if line.starts_with("  -1000000006") {
            output.fixed = parse_to_int(&line).iter().map(|&x| x == 1).collect();
        } else if line.starts_with("  -1000000007") {
            let error_status = parse_error_codes(&line);
            output.termination_status = error_status;
        } else if line.starts_with("  -1000000008") {
            output.partial_derivative = Some(parse_float(&line));
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    const LINE: &str = "ITERATION    THETA1       THETA2       THETA3       SIGMA(1,1)   SIGMA(2,1)   SIGMA(2,2)   OMEGA(1,1)   OMEGA(2,1)   OMEGA(2,2)   OMEGA(3,1)   OMEGA(3,2)   OMEGA(3,3)   OBJ";

    #[test]
    fn test_parse_title() {
        let expected: Vec<&str> = vec![
            "THETA1",
            "THETA2",
            "THETA3",
            "SIGMA(1,1)",
            "SIGMA(2,1)",
            "SIGMA(2,2)",
            "OMEGA(1,1)",
            "OMEGA(2,1)",
            "OMEGA(2,2)",
            "OMEGA(3,1)",
            "OMEGA(3,2)",
            "OMEGA(3,3)",
        ];
        assert_eq!(parse_title(LINE), expected)
    }

    #[test]
    fn test_parse_int() {
        let expected: Vec<i32> = vec![0, 37, 38, 0];
        let line =
            "-1000000007  0.00000E+00  3.70000E+01  3.80000E+01  0.00000E+00    0.0000000000000000";
        assert_eq!(parse_to_int(line), expected)
    }

    #[test]
    fn test_parse_error_codes() {
        let line =
            "-1000000007  0.00000E+00  3.70000E+01  3.80000E+01  0.00000E+00    0.0000000000000000";
        let expected: TerminationStatus = TerminationStatus {
            status: 0,
            codes: vec![37, 38],
        };
        assert_eq!(parse_error_codes(line), expected)
    }
}
