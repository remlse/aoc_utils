fn number_dirs_in(parent: &str) -> Vec<u16> {
    let mut num_dirs = std::fs::read_dir(parent)
        .expect("should read project root directory")
        .flatten()
        .filter(|dir_entry| matches!(dir_entry.file_type(), Ok(t) if t.is_dir()))
        .filter_map(|dir| dir.file_name().to_str().map(|s| s.to_owned()))
        .filter_map(|dir_name| dir_name.parse().ok())
        .collect::<Vec<_>>();
    num_dirs.sort();
    num_dirs
}

const ROW_HEIGHT: u16 = 25;

fn main() {
    let years = number_dirs_in(".");

    let mut table = format!(
        "<svg viewBox=\"0 0 500 {}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        ROW_HEIGHT as usize * years.len()
    );

    for year in years {
        let days = number_dirs_in(year.to_string().as_str());

        for day in 1..=25 {
            let x = format!("x=\"{}\"", (day - 1) * 20);
            let y = format!("y=\"{}\"", (year - 15) * ROW_HEIGHT + 15);
            let fill = if days.contains(&day) { "⭐" } else { "🪟" };
            table += &format!("<text {x:<7} {y:<7}>{fill}</text>\n");
        }
    }
    table += "</svg>\n";

    print!("{table}");
}
