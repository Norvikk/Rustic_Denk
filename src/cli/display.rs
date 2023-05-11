use colored::Colorize;

pub fn developer() {
    println!(
        "{} {} {} {} {}            {}\n",
        "Denk_Algo".bright_red(),
        "|".bold(),
        "V. 1.0".red(),
        "|".bold(),
        "Norvik".red(),
        "BUILD: optimized - API".on_white().black().bold(),
    );
}
