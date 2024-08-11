#[cfg(test)]
use super::*;

mod tests {
    #[allow(unused_imports)]
    use soroban_sdk::Env;

    #[test]
    fn add() {
        let env = Env::default();
        let contract_id = env.register_contract(None, super::Calculator);
        let client = super::CalculatorClient::new(&env, &contract_id);
        let sum = client.add(&1, &2);
        assert_eq!(sum, 3);
    }
}