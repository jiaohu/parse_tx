#[cfg(test)]
mod tests {
    use macros::parse_tx;
    use my_types::Transaction;

    #[test]
    fn test_1() {
        let a: Transaction = parse_tx!({
            tx_hash: "01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7",
            fee: 10000,
            action: {
                action: "register",
                params: "0x00"
            },
            inputs: [
            {
                index: 0,
                capacity: 10000
            },
            {
                index: 1,
                capacity: 10000
            }
            ],
            outputs: [
            {
                index: 0,
                capacity: 10000
            },
            {
                index: 1,
                capacity: 10000
            }
        ],
        digest: "01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfae7"
        });
        println!("{:?}", a);
        assert_eq!(a.fee, 10000)
    }
}
