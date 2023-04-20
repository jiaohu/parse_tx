use proc_macro::TokenStream;
use std::borrow::Borrow;
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use regex::Regex;
use syn::LitStr;


#[proc_macro]
pub fn parse_tx(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let lit = LitStr::new(&input_str[1..input_str.len() - 1], proc_macro2::Span::call_site());
    let binding = lit.value().trim().replace("\n", "");
    let origin = binding.as_str();


    // 1. parse tx_hash
    let mut tx_hash = vec![];
    let mut re = Regex::new(r#"tx_hash\s*:\s*"([[:xdigit:]]+)""#).unwrap();
    if let Some(a) = re.captures(origin) {
        tx_hash = hex::decode(a.get(1).unwrap().as_str()).unwrap();
    }


    // 2. parse fee
    let mut fee: u64 = 0;
    re = Regex::new(r#"fee\s*:\s*([[:digit:]]+)"#).unwrap();
    if let Some(a) = re.captures(origin) {
        fee = a.get(1).unwrap().as_str().parse::<u64>().unwrap();
    }

    // 3. parse action
    let mut action_str = "";
    let mut params_str = "";
    re = Regex::new(r#"\{\s*action\s*:\s*"(?P<action>[[:ascii:]]*)",\s*params\s*:\s*"(?P<params>[[:ascii:]]*)"\s*\}"#).unwrap();
    if let Some(a) = re.captures(origin) {
        action_str = a.name("action").unwrap().as_str();
        params_str = a.name("params").unwrap().as_str();
    }

    let parse_put_data = |mut x: Vec<TokenStream2>, r: &Regex, o_str: &str| -> Vec<TokenStream2> {
        if let Some(captures) = r.captures(o_str) {
            let items_str = captures.name("items").unwrap().as_str().trim();
            let items_re = Regex::new(r#"\s*index\s*:\s*(?P<index>\d*),\s*capacity\s*:\s*(?P<capacity>\d*)"#).unwrap();
            for caps in items_re.captures_iter(items_str) {
                let index = caps["index"].parse::<u64>().unwrap();
                let capacity = caps["capacity"].parse::<u64>().unwrap();
                x.push(quote! {
                my_types::PutData {
                    index: #index,
                    capacity: #capacity
                }
            });
            }
        }
        x
    };
    // 4. parse inputs
    let mut inputs: Vec<TokenStream2> = vec![];
    re = Regex::new(r#"inputs\s*:\s*\[(?P<items>.+)\],\s*outputs"#).unwrap();
    inputs = parse_put_data(inputs, re.borrow(), origin);


    // 5. parse outputs
    let mut outputs: Vec<TokenStream2> = vec![];
    re = Regex::new(r#"outputs\s*:\s*\[(?P<items>.+)\],"#).unwrap();
    outputs = parse_put_data(outputs, re.borrow(), origin);

    // 6. parse digest
    let mut digest = vec![];
    re = Regex::new(r#"digest\s*:\s*"([[:xdigit:]]+)""#).unwrap();
    if let Some(a) = re.captures(origin) {
        digest = hex::decode(a.get(1).unwrap().as_str()).unwrap();
    }

    let tx_struct = quote! {
        my_types::Transaction {
            tx_hash: vec![ #( #tx_hash ),*],
            fee: #fee,
            action: my_types::Action {
                action: String::from(#action_str),
                params: String::from(#params_str)
            },
            inputs: vec![ #( #inputs ),*],
            outputs: vec![ #( #outputs ),*],
            digest: vec![ #( #digest ),*],
        }
    };

    tx_struct.into()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use my_types::{Action, PutData, Transaction};
    use super::*;


    #[test]
    fn test_1() {
        let a = serde_json::to_string(&Transaction {
            tx_hash: hex::encode("hello").into_bytes(),
            fee: 0,
            action: Action {
                action: "123".to_string(),
                params: "234".to_string(),
            },
            inputs: vec![PutData { index: 0, capacity: 0 }],
            outputs: vec![],
            digest: hex::encode("world").into_bytes(),
        }).unwrap();
        println!("{}", a)
    }

    #[test]
    fn test_match() {
        let text = "tx_hash :    \"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7\", fee :    10000, action : { action : \"register\", params : \"0x00\" }, inputs :    [{ index : 0, capacity : 10000 }, { index : 1, capacity : 10000 }],    outputs :    [{ index : 0, capacity : 10000 }, { index : 1, capacity : 10000 }], digest    : \"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7\"";
        let re = Regex::new(r#"tx_hash\s*:\s*"([[:xdigit:]]+)""#).unwrap();
        if let Some(cap) = re.captures(text) {
            println!("{}", cap.get(1).unwrap().as_str())
        }
    }

    #[test]
    fn test_match_inputs() {
        let text = "tx_hash :    \"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7\", fee :    10000, action : { action : \"register\", params : \"0x00\" }, inputs :    [{ index : 0, capacity : 10000 }, { index : 1, capacity : 10000 }],    outputs :    [{ index : 0, capacity : 10000 }, { index : 1, capacity : 10000 }], digest    : \"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7\"";
        let re = Regex::new(r#"inputs\s*:\s*\[(?P<items>.+)\],\s*outputs"#).unwrap();
        if let Some(captures) = re.captures(text) {
            let items_str = captures.name("items").unwrap().as_str().trim();
            let items_re = Regex::new(r#"\s*index\s*:\s*(?P<index>\d*),\s*capacity\s*:\s*(?P<capacity>\d*)"#).unwrap();
            for caps in items_re.captures_iter(items_str) {
                println!("Movie: {:?}, Released: {:?}",
                         &caps["index"], &caps["capacity"]);
            }
        }
    }

    #[test]
    fn test_map() {
        let text = "tx_hash :    \"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7\", fee :    10000, action : { action : \"register\", params : \"0x00\" }, inputs :    [{ index : 0, capacity : 10000 }, { index : 1, capacity : 10000 }],    outputs :    [{ index : 0, capacity : 10000 }, { index : 1, capacity : 10000 }], digest    : \"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7\"";
        let mut map: HashMap<String, String> = HashMap::new();
        let re = regex::Regex::new(r#"(\w+)\s*:\s*([[:ascii:]]+),"#).unwrap();
        for capture in re.captures_iter(text) {
            let key = capture.get(1).unwrap().as_str();
            let value = capture.get(2).unwrap().as_str();
            println!("{} : {}", key, value)
        }
    }
}