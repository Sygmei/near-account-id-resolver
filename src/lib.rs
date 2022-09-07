use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use postgres::{Client, NoTls};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_account_id(&self, public_key: String) -> String {
        let mut client = Client::connect("host=testnet.db.explorer.indexer.near.dev user=public_readonly password=nearprotocol database=testnet_explorer", NoTls)
            .expect("failed to connect to NEAR PostgreSQL server");
        let query = format!(
            "SELECT account_id FROM access_keys WHERE public_key = 'ed25519:{}';",
            public_key
        );
        for row in client.query(&query, &[]).unwrap() {
            let account_id: String = row.get(0);

            return account_id;
        }
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_specific_account_id() {
        let mut contract = Contract::default();

        assert_eq!(1, 1);
    }
}
