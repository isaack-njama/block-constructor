use std::fs;
use std::io::{BufRead, BufReader, Write};

pub struct MempoolTransaction {
    txid: String,
    fee: i32,
    weight: i32,
    parents: String,
}

impl MempoolTransaction {
    fn new(txid: String, fee: &str, weight: &str, parents: String) -> MempoolTransaction {
        let fee = fee.parse::<i32>().unwrap_or(0);
        let weight = weight.parse::<i32>().unwrap_or(0);
        MempoolTransaction {
            txid,
            fee,
            weight,
            parents,
        }
    }

    fn parse_mempool_csv() -> Vec<MempoolTransaction> {
        let file = fs::File::open("mempool.csv").expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut transactions = Vec::new();

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

    fn write_output_file(output_file: &str) {
        let transactions = MempoolTransaction::parse_mempool_csv();

        let mut block_data_file =
            fs::File::create(output_file).expect("Failed to create output file");

        for transaction in &transactions {
            if !transaction.parents.is_empty() {
                let parent_txids: Vec<&str> = transaction.parents.split(',').collect();
                for parent_txid in parent_txids {
                    if let Some(parent_tx) = transactions
                        .iter()
                        .find(|&tx| tx.txid == *parent_txid)
                    {
                        writeln!(block_data_file, "{}", parent_tx).expect("Write failed");
                    }
                }
            }
            writeln!(block_data_file, "{}", transaction).expect("Write failed");
        }
    }
}

impl std::fmt::Display for MempoolTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.txid)
    }
}

fn main() {
    let output_file = "transactions.txt";
    MempoolTransaction::write_output_file(output_file);
}

#[cfg(test)]
#[path ="../tests/test.rs"]
mod test;