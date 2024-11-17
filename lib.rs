#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    #[test]
    fn test_initialization() {
        let caller = Principal::anonymous();
        WALLET.lock().unwrap().balances.insert(caller, 1000);
        assert_eq!(WALLET.lock().unwrap().balances.get(&caller), Some(&1000));
    }

    #[test]
    fn test_transfer() {
        let caller = Principal::anonymous();
        let recipient = Principal::anonymous(); // Replace with actual recipient principal
        WALLET.lock().unwrap().balances.insert(caller, 1000);
        transfer(recipient, 500).unwrap();
        assert_eq!(WALLET.lock().unwrap().balances.get(&caller), Some(&500));
        assert_eq!(WALLET.lock().unwrap().balances.get(&recipient), Some(&500));
    }
}
