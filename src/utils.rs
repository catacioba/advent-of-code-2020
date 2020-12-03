pub mod utils {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufRead;
    use std::io;

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>,
    {
        let file = File::open(filename).unwrap();
        Ok(io::BufReader::new(file).lines())
    }

    pub fn convert_lines_to_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i64> {
        lines
            .map(|line| line.unwrap().parse::<i64>().unwrap())
            .collect()
    }
}