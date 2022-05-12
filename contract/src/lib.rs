use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    name: String,
    address: String
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "dnhan_nguyen".to_string(),
            address: "HN".to_string()
        }
    }
}

#[near_bindgen]
impl User {
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_address(&self) -> &String {
        return &self.address;
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
        log!("Change name to {}", self.name);
    }

    pub fn set_address(&mut self, new_address: String) {
        self.address = new_address;
        log!("Change address to {}", self.address);
    }
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_name() {
        let contract = User { name: "Nhan".to_string(), address: "Japan".to_string() };
        assert_eq!(&"Nhan".to_string(), contract.get_name());
    }

    #[test]
    fn get_address() {
        let contract = User { name: "Nhan".to_string(), address: "Japan".to_string() };
        assert_eq!(&"Japan".to_string(), contract.get_address());
    }

    #[test]
    fn set_name() {
        let mut contract = User { name: "Nhan".to_string(), address: "Japan".to_string() };
        contract.set_name("Test Name".to_string());
        assert_eq!("Test Name", contract.get_name());
    }

    #[test]
    fn set_address() {
        let mut contract = User { name: "Nhan".to_string(), address: "Japan".to_string() };
        contract.set_address("Test address".to_string());
        assert_eq!("Test address", contract.get_address());
    }
}
