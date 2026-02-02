//! CSV output utilities for exporting algorithm results.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Writes a 2D dataset to a CSV file.
pub fn write_csv<P: AsRef<Path>>(path: P, headers: &[&str], data: &[Vec<f64>]) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", headers.join(","))?;

    for row in data {
        let line: String = row.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_csv() {
        let path = "/tmp/test_output.csv";
        let headers = &["x", "y"];
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

        write_csv(path, headers, &data).unwrap();

        let contents = fs::read_to_string(path).unwrap();
        assert!(contents.contains("x,y"));
        assert!(contents.contains("1,2"));
        assert!(contents.contains("3,4"));

        fs::remove_file(path).unwrap();
    }
}
