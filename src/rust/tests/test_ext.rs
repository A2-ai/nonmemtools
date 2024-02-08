use std::fs;
use std::path::Path;
use nonmemtools::parsers::ext;

// Add this line to implement the Debug trait


#[test]
fn test_single_table() {
    let single_table_ext = Path::new("tests/fixtures/single-table-01.ext");
    let start_time = std::time::Instant::now();
    let output = ext::parse_ext_file(single_table_ext);
    dbg!(output);
    println!("Time elapsed in parse_ext_file() is: {:?}", start_time.elapsed());
}