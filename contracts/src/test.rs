#[cfg(test)]
mod test {
    use crate::{HealthcareRegistry, HealthcareRegistryClient};

    use super::*; // This is now used correctly by the client
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup_test(env: &Env) -> (HealthcareRegistryClient<'static>, Address, Address) {
        // Updated from register_contract to register
        let contract_id = env.register(HealthcareRegistry, ()); 
        let client = HealthcareRegistryClient::new(env, &contract_id);
        
        let admin = Address::generate(env);
        let institution = Address::generate(env);
        
        client.init(&admin);
        
        (client, admin, institution)
    }

    #[test]
    fn test_register_and_get() {
        let env = Env::default();
        let (client, _, inst_addr) = setup_test(&env);

        let name = String::from_str(&env, "General Hospital");
        let license = String::from_str(&env, "LIC-123");
        let meta = String::from_str(&env, "{}");

        env.mock_all_auths();
        client.register_institution(&inst_addr, &name, &license, &meta);

        let data = client.get_institution(&inst_addr);
        assert_eq!(data.name, name);
    }

    #[test]
    #[should_panic(expected = "Already registered")]
    fn test_duplicate_registration_fails() {
        let env = Env::default();
        let (client, _, inst_addr) = setup_test(&env);
        env.mock_all_auths();

        let name = String::from_str(&env, "Clinic A");
        client.register_institution(&inst_addr, &name, &name, &name);
        client.register_institution(&inst_addr, &name, &name, &name);
    }

    #[test]
    fn test_verification_by_admin() {
        let env = Env::default();
        let (client, admin, inst_addr) = setup_test(&env);
        env.mock_all_auths();

        let name = String::from_str(&env, "Clinic A");
        client.register_institution(&inst_addr, &name, &name, &name);

        client.verify_institution(&admin, &inst_addr);

        let data = client.get_institution(&inst_addr);
        assert_eq!(data.is_verified, true);
    }

    #[test]
    #[should_panic(expected = "Not authorized to verify")]
    fn test_unauthorized_verification_fails() {
        let env = Env::default();
        let (client, _, inst_addr) = setup_test(&env);
        let fake_admin = Address::generate(&env);
        env.mock_all_auths();

        let name = String::from_str(&env, "Clinic A");
        client.register_institution(&inst_addr, &name, &name, &name);

        client.verify_institution(&fake_admin, &inst_addr);
    }

    #[test]
    fn test_update_metadata() {
        let env = Env::default();
        let (client, _, inst_addr) = setup_test(&env);
        env.mock_all_auths();

        client.register_institution(&inst_addr, &String::from_str(&env, "H"), &String::from_str(&env, "1"), &String::from_str(&env, "old"));
        
        let new_meta = String::from_str(&env, "new_metadata");
        client.update_institution(&inst_addr, &new_meta);

        let data = client.get_institution(&inst_addr);
        assert_eq!(data.metadata, new_meta);
    }
}