use soroban_sdk::{Address, Env, String, Symbol, contract, contracterror, contractimpl, contracttype, symbol_short};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstitutionData {
    pub name: String,
    pub license_id: String,
    pub metadata: String,
    pub is_verified: bool,
}

#[contracttype]
pub enum DataKey {
    Inst(Address),
    Admin, // To manage the 'verifier' role
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyRegistered = 1,
    NotFound = 2,
    NotAuthorized = 3,
}

#[contract]
pub struct HealthcareRegistry;

#[contractimpl]
impl HealthcareRegistry {
    // Set an admin/verifier during initialization
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn register_institution(env: Env, wallet: Address, name: String, license_id: String, metadata: String) {
        wallet.require_auth();

        let key = DataKey::Inst(wallet.clone());
        if env.storage().persistent().has(&key) {
            panic!("Already registered");
        }

        let data = InstitutionData {
            name,
            license_id,
            metadata,
            is_verified: false,
        };

        env.storage().persistent().set(&key, &data);
        
        // Event emission
        env.events().publish((symbol_short!("reg"), wallet), symbol_short!("success"));
    }

    pub fn get_institution(env: Env, wallet: Address) -> InstitutionData {
        let key = DataKey::Inst(wallet);
        env.storage().persistent().get(&key).expect("Institution not found")
    }

    pub fn update_institution(env: Env, wallet: Address, metadata: String) {
        wallet.require_auth();
        
        let key = DataKey::Inst(wallet.clone());
        let mut data: InstitutionData = env.storage().persistent().get(&key).expect("Not found");
        
        data.metadata = metadata;
        env.storage().persistent().set(&key, &data);
    }

    pub fn verify_institution(env: Env, verifier: Address, wallet: Address) {
        verifier.require_auth();
        
        // Access Control: Check if caller is the admin
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if verifier != admin {
            panic!("Not authorized to verify");
        }

        let key = DataKey::Inst(wallet.clone());
        let mut data: InstitutionData = env.storage().persistent().get(&key).expect("Not found");
        
        data.is_verified = true;
        env.storage().persistent().set(&key, &data);
    }
}

mod test;