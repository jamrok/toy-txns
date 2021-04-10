#[cfg(test)]
mod tests {
    use toy_txns::libs::transactions::TransactionManager;

    #[test]
    #[should_panic(expected = "not_a_file: No such file or directory")]
    fn missing_file() {
        TransactionManager::process_input_file(&"not_a_file".to_string()).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }
}
