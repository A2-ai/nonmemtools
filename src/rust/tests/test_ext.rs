use std::fs;
use std::path::Path;
use nonmemtools::parsers::ext;
use ext::{ExtEstimationResults, ExtTableResultsOutput, TerminationStatus};
// Add this line to implement the Debug trait


#[test]
fn test_single_table() {
    let single_table_ext = Path::new("tests/fixtures/single-table-01.ext");
    let start_time = std::time::Instant::now();
    let output = ext::parse_ext_file(single_table_ext);
    dbg!(output);
    println!("Time elapsed in parse_ext_file() is: {:?}", start_time.elapsed());
}

#[test]
fn test_multiple_table() {
    let table = Path::new("tests/fixtures/two-table-01.ext");
   
    let start_time = std::time::Instant::now();
    let output = ext::parse_ext_file(table);
    // help codegenning via https://chat.openai.com/share/094da42f-13d7-4d6e-9ebb-ac334a8dd7d6
    let expected_output = ExtTableResultsOutput {
        names: vec![
            "THETA1".to_string(), "THETA2".to_string(), "THETA3".to_string(),
            "THETA4".to_string(), "THETA5".to_string(), "THETA6".to_string(),
            "THETA7".to_string(), "THETA8".to_string(), "THETA9".to_string(),
            "THETA10".to_string(), "THETA11".to_string(), "THETA12".to_string(),
            "THETA13".to_string(), "SIGMA(1,1)".to_string(), "OMEGA(1,1)".to_string(),
            "OMEGA(2,1)".to_string(), "OMEGA(2,2)".to_string(),
        ],
        data: vec![
            ExtEstimationResults {
                title: "TABLE NO.     1: Stochastic Approximation Expectation-Maximization: Goal Function=FINAL VALUE OF LIKELIHOOD FUNCTION: Problem=1 Subproblem=0 Superproblem1=0 Iteration1=0 Superproblem2=0 Iteration2=0\n".to_string(),
                estimates: vec![
                    2.33804, 28.4923, -0.743265, 0.00638923, -3.00269e-5, 0.011062,
                    -0.0227752, 0.0431895, -0.0739587, 0.137796, -0.178194, 0.548269,
                    -0.452265, 0.0449295, 0.0295071, 0.0, 0.681026,
                ],
                std_errors: Some(vec![
                    0.0352914, 1.91624, 0.00240809, 1.74227e-5, 1.94094e-5, 0.00764889,
                    0.031025, 0.0282091, 0.0756834, 0.0224119, 0.086865, 0.162997,
                    0.320617, 0.000486596, 0.00182747, 0.0, 0.054566,
                ]),
                eigenvalues: Some(vec![
                    0.136875, 0.212361, 0.2617, 0.325154, 0.344213, 0.457476,
                    0.576399, 0.893848, 1.00054, 1.12621, 1.1382, 1.28628,
                    1.69526, 1.72286, 1.9242, 2.89842, 0.0,
                ]),
                condition_number: 0.136875,
                fixed: vec![
                    false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, false,
                ],
                termination_status: TerminationStatus { status: 0, codes: vec![] },
                partial_derivative: Some(vec![
                    0.159549, -0.000603369, 258.212, -48716.0, 7.98249, -0.157343,
                    0.163215, 0.0348247, -0.0200301, 0.00440389, -0.00517219,
                    0.000727935, 0.00151352, 3.45295, -1.84856, 0.0, 0.00399278,
                ]),
            },
            ExtEstimationResults {
                title: "TABLE NO.     2: Objective Function Evaluation by Importance Sampling: Goal Function=FINAL VALUE OF OBJECTIVE FUNCTION: Problem=1 Subproblem=0 Superproblem1=0 Iteration1=0 Superproblem2=0 Iteration2=0\n".to_string(),
                estimates: vec![
                    2.23804, 28.4923, -0.743265, 0.00638923, -3.00269e-5, 0.011062,
                    -0.0217752, 0.0431895, -0.0739587, 0.137796, -0.178194, 0.548269,
                    -0.452265, 0.0449295, 0.0295071, 0.0, 0.671026,
                ],
                std_errors: Some(vec![
                    0.0371291, 1.65889, 0.000171994, 4.09255e-6, 9.23832e-6, 0.00476713,
                    0.0241884, 0.0222926, 0.0884264, 0.0237098, 0.0924824, 0.128321,
                    0.10451, 0.00608201, 0.00585466, 0.0, 0.0442403,
                ]),
                eigenvalues: Some(vec![
                    0.00409247, 0.0259299, 0.18685, 0.236312, 0.284972, 0.323288,
                    0.515171, 0.564798, 0.653519, 1.02376, 1.05999, 1.41487,
                    1.62169, 2.08645, 2.82429, 3.17401, 0.0,
                ]),
                condition_number: 0.00409247,
                fixed: vec![
                    false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, false,
                ],
                termination_status: TerminationStatus { status: 1, codes: vec![] },
                partial_derivative: Some(vec![
                    2.39234, -0.006921, 11.5255, 297.897, 2910.21, -9.55619,
                    3.2947, 3.31487, 0.771798, 0.616179, -0.0619476, -0.12482,
                    -0.10152, -20.8626, -42.9173, 0.0, -0.147185,
                ]),
            },
        ],
    };

    assert_eq!(output.unwrap(), expected_output);
    println!("Time elapsed in parse_ext_file() is: {:?}", start_time.elapsed());
}