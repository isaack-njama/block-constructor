use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader, Write};

pub struct MempoolTransaction {
    txid: String,       // Transaction ID
    fee: i32,           // Transaction fee
    weight: i32,        // Transaction weight
    parents: String,    // Parent transactions
}

impl MempoolTransaction {
    // Constructor method to create a new MempoolTransaction
    fn new(txid: String, fee: &str, weight: &str, parents: String) -> MempoolTransaction {
        // Parsing fee and weight into integers, defaults to 0 if parsing fails
        let fee = fee.parse::<i32>().unwrap_or(0);
        let weight = weight.parse::<i32>().unwrap_or(0);
        MempoolTransaction {
            txid,
            fee,
            weight,
            parents,
        }
    }

    // Parse the mempool CSV file and return a vector of MempoolTransactions
    fn parse_mempool_csv() -> Vec<MempoolTransaction> {
        // Open and read the mempool CSV file
        let file = fs::File::open("mempool.csv").expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut transactions = Vec::new();

        // Parse each line of the CSV file to extract transaction information
        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 4 {
                let txid = parts[0].to_string();
                let fee = parts[1];
                let weight = parts[2];
                let parents = parts[3].to_string();
                let transaction = MempoolTransaction::new(txid, fee, weight, parents);
                transactions.push(transaction);
            }
        }
        transactions
    }

    // Write output file with transaction information, including parent transactions
    fn write_output_file(output_file: &str) {
        let transactions = MempoolTransaction::parse_mempool_csv();

        // Create output file and handle errors if failed to create
        let mut block_data_file =
            fs::File::create(output_file).expect("Failed to create output file");

        // Keep track of written transactions to avoid duplicates
        let mut written_transactions: HashSet<String> = HashSet::new();

        // Iterate through transactions to write them to the output file
        for transaction in &transactions {
            // Write parent transactions first, if any
            if !transaction.parents.is_empty() {
                let parent_txids: Vec<&str> = transaction.parents.split(',').collect();
                for parent_txid in &parent_txids {
                    if !written_transactions.contains(&parent_txid.to_string()) {
                        if let Some(parent_tx) =
                            transactions.iter().find(|&tx| tx.txid == *parent_txid)
                        {
                            // Write parent transaction to the file
                            writeln!(block_data_file, "{}", parent_tx).expect("Write failed");
                            written_transactions.insert(parent_txid.to_string());
                        }
                    }
                }
            }
            // Write the current transaction if it hasn't been written already
            if !written_transactions.contains(&transaction.txid) {
                // Write current transaction to the file
                writeln!(block_data_file, "{}", transaction).expect("Write failed");
                written_transactions.insert(transaction.txid.clone()); // Use clone() to create a new String
            }
        }
    }
}

impl std::fmt::Display for MempoolTransaction {
    // Implement Display trait to format MempoolTransaction for output
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display transaction ID
        write!(f, "{}", self.txid)
    }
}

fn main() {
    // Output file for writing transactions
    let output_file = "transactions.txt";
    // Call write_output_file to generate the output file
    MempoolTransaction::write_output_file(output_file);
}

// #[cfg(test)]
// #[path ="../tests/test.rs"]
// mod test;

#[cfg(test)]
mod tests {
    use super::MempoolTransaction;
    use std::fs;

    #[test]
    fn test_parse_mempool_csv() {
        // Create sample CSV content
        let csv_content = "2e3da8fbc1eaca8ed9b7c2db9e6545d8ccac3c67deadee95db050e41c1eedfc0,452,1620,\n\
        79c51c9d4124c5cbb37a85263748dcf44e182dff83561fa3087f0e9e43f41c33,682,1136,6eb38fad135e38a93cb47a15a5f953cbc0563fd84bf1abdec578c2af302e10bf\n\
        b0ef627c8dc2a706475d33d7712209ec779f7a8302aaeab86c64cf00316a3df8,226,900,\n";

        // Write the sample CSV content to a temporary file
        let temp_dir = tempdir::TempDir::new("test").expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("test_mempool.csv");
        fs::write(&file_path, csv_content).expect("Failed to write to file");

        // Call the function to parse the CSV
        let transactions = MempoolTransaction::parse_mempool_csv();

        // Assert that the parsed transactions match the expected ones
        assert_eq!(transactions.len(), 3); // Ensure all transactions are parsed
        assert_eq!(transactions[0].txid, "2e3da8fbc1eaca8ed9b7c2db9e6545d8ccac3c67deadee95db050e41c1eedfc0"); // Ensure transaction fields are correctly parsed
        assert_eq!(transactions[1].parents, "6eb38fad135e38a93cb47a15a5f953cbc0563fd84bf1abdec578c2af302e10bf");
        assert_eq!(transactions[2].fee, 226);
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
