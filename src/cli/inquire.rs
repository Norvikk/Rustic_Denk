use inquire::{InquireError, Select};

pub fn process(processes: Vec<&str>) -> anyhow::Result<usize> {
    // creates the CLI using crate inquire
    let selection: Result<&str, InquireError> =
        Select::new("Select process", processes.clone()).prompt();

    let selection_result = selection?;

    // returns the index of the users selection
    Ok(processes
        .iter()
        .position(|&r| r == selection_result)
        .unwrap())
}
