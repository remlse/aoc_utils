use std::{fs::File, io::Write};

use anyhow::{anyhow, Context};

static MARKER: &str = "<!-- generate_readme_table_marker -->\n";

static HEADER: &str = "<pre>
        1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
     ┌────────────────────────────────────────────────────────────────────────────┐
";
static FOOTER: &str =
    "     └────────────────────────────────────────────────────────────────────────────┘
</pre>
";

fn number_dirs_in(parent: &str) -> anyhow::Result<Vec<u8>> {
    let mut num_dirs = std::fs::read_dir(parent)
        .context("failed to read project root directory")?
        .flatten()
        .filter(|dir_entry| matches!(dir_entry.file_type(), Ok(t) if t.is_dir()))
        .filter_map(|dir| dir.file_name().to_str().map(|s| s.to_owned()))
        .filter_map(|dir_name| dir_name.parse::<u8>().ok())
        .collect::<Vec<_>>();
    num_dirs.sort();
    Ok(num_dirs)
}

fn table() -> anyhow::Result<String> {
    let mut table = String::from(HEADER);

    let years = number_dirs_in(".").context("failed to read years")?;

    for year in years {
        table += &format!("20{year:02} │ <a\n");

        let days = number_dirs_in(year.to_string().as_str()).context("failed to read days")?;

        for day in 1..=25 {
            let href = format!("\"https://adventofcode.com/20{year:02}/day/{day}\"");
            let fill = if days.contains(&day) {
                "▓▓"
            } else {
                "░░"
            };
            table += &format!("href={href:<38}>{fill}</a");

            table += match day {
                25 => "\n",
                _ => "> <a\n\n",
            };
        }
        table += "> │\n";
    }
    table += FOOTER;

    Ok(table)
}

fn main() -> anyhow::Result<()> {
    let readme = std::fs::read_to_string("README.md").context("failed to read README.md")?;
    let mut iter = readme.split(MARKER);
    let pre = iter
        .next()
        .ok_or(anyhow!("failed to parse stuff before table"))?;
    let post = iter
        .nth(1)
        .ok_or(anyhow!("failed to parse stuff after table"))?;

    let table = table().context("failed to construct table")?;
    let new_readme = format!("{pre}{MARKER}{table}{MARKER}{post}");

    let mut out = File::create("README.md").context("failed to open README for writing")?;
    out.write_all(new_readme.as_bytes())
        .context("failed to write new README")
}
