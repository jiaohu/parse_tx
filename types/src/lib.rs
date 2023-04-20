use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_hash: Vec<u8>,
    pub fee: u64,
    pub action: Action,
    pub inputs: Vec<PutData>,
    pub outputs: Vec<PutData>,
    pub digest: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub action: String,
    pub params: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutData {
    pub index: u64,
    pub capacity: u64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }
        let j = r#"{"int":1,"seq":["a","b"]}"#;
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, serde_json::from_str(j).unwrap());
    }
}