#[cfg(test)]
mod tests {
    use std::{fs, process};
    use toy_txns::libs::transactions::TransactionManager;

    pub fn read_lines(filename: &String) -> String {
        fs::read_to_string(filename).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        })
    }

    #[test]
    fn sample_transactions() {
        for input_file in [
            "transactions.csv",
            "transactions2.csv",
            "transactions3.csv",
            "precision.csv",
            "bad_line.csv",
        ]
        .iter()
        {
            let input_file = format!("tests/{}", input_file);
            let output_file = format!("{}.out", input_file);
            let data = read_lines(&output_file);
            let result = TransactionManager::process_input_file(&input_file.to_string())
                .unwrap_or_else(|err| {
                    println!("{}", err);
                    process::exit(1);
                });
            assert_eq!(result, data.trim_end());
        }
    }

    #[test]
    #[should_panic(expected = "CSV deserialize error: record 3")]
    fn invalid_file() {
        let input_file = "bad_file.csv";
        let input_file = format!("tests/{}", input_file);
        TransactionManager::process_input_file(&input_file.to_string()).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }
}
