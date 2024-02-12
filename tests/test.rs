#[cfg(test)]
mod tests {
    use crate::MempoolTransaction;
    use std::fs;
    use std::io::{BufRead, BufReader, Write};

    #[test]
    fn test_parse_mempool_csv() {
        // Create sample CSV content
        let csv_content = "txid1,100,200,parent1\n\
                           txid2,150,250,\n\
                           txid3,200,300,parent2\n";

        // Write the sample CSV content to a temporary file
        let temp_dir = tempdir::TempDir::new("test").expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_mempool.csv");
        fs::write(&file_path, csv_content).expect("Failed to write to file");

        // Call the function to parse the CSV
        let transactions = MempoolTransaction::parse_mempool_csv();

        // Assert that the parsed transactions match the expected ones
        assert_eq!(transactions.len(), 3); // Ensure all transactions are parsed
        assert_eq!(transactions[0].txid, "txid1"); // Ensure transaction fields are correctly parsed
        assert_eq!(transactions[1].fee, 150);
        assert_eq!(transactions[2].parents, "parent2");
    }

    #[test]
    fn test_write_output_file() {
        // Set up a sample output file path
        let output_file = "test_output.txt";

        // Call the function to write the output file
        MempoolTransaction::write_output_file(output_file);

        // Read the contents of the output file and assert its content
        let content = fs::read_to_string(output_file).expect("Failed to read output file");
        let expected_content = "txid1\nparent1\ntxid2\ntxid3\nparent2\n";
        assert_eq!(content, expected_content); // Ensure transactions are written correctly
    }
}
