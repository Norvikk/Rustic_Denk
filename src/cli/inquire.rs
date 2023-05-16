use inquire::{InquireError, Select};

pub fn process(processes: Vec<&str>) -> usize{
    // creates the CLI using crate inquire
    let selection: Result<&str, InquireError> =
        Select::new("Select process", processes.clone()).prompt();

    let selection_result = selection.unwrap();

    // returns the index of the users selection
    processes
        .iter()
        .position(|&r| r == selection_result)
        .unwrap()
}
