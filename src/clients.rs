use rand::rngs::OsRng;
use x25519_dalek::*;

pub struct StaticKeyClient {
    secret: StaticSecret,
    pub client_id: ClientID,
    pub pubkey: PublicKey,
}

impl StaticKeyClient {
    pub fn new<T: Into<ClientID>>(id: T) -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let pubkey = PublicKey::from(&secret);
        Self {
            secret,
            pubkey,
            client_id: id.into(),
        }
    }
    pub fn calculate_shared_secret(&self, other_pubkey: &PublicKey) -> SharedSecret {
        self.secret.diffie_hellman(other_pubkey)
    }
}

#[derive(Clone, Copy)]
pub struct ClientID(pub u8);

impl From<u8> for ClientID {
    fn from(id: u8) -> Self {
        ClientID(id)
    }
}

pub struct TmpKeyClient {
    shared_secrets: Vec<(ClientID, SharedSecret)>,
    pub id: ClientID,
}

impl TmpKeyClient {
    pub fn new(id: u8) -> Self {
        Self {
            shared_secrets: vec![],
            id: ClientID(id),
        }
    }
    pub fn add_shared_secret(
        &mut self,
        s: EphemeralSecret,
        other_pubkey: &PublicKey,
        other_id: ClientID,
    ) {
        self.shared_secrets
            .push((other_id, s.diffie_hellman(other_pubkey)));
    }
}
pub fn tmp_client_key_exchange(client: &mut TmpKeyClient, other_client: &mut TmpKeyClient) {
    let secret1 = EphemeralSecret::random_from_rng(OsRng);
    let pubkey1 = PublicKey::from(&secret1);

    let secret2 = EphemeralSecret::random_from_rng(OsRng);
    let pubkey2 = PublicKey::from(&secret2);

    client.add_shared_secret(secret1, &pubkey2, other_client.id);
    other_client.add_shared_secret(secret2, &pubkey1, client.id);
}

//this is useless in any practical setting. only using it to benchmark
#[allow(dead_code)]
pub fn tmp_client_key_generation() {
    let secret1 = EphemeralSecret::random_from_rng(OsRng);
    let _pubkey1 = PublicKey::from(&secret1);

    let secret2 = EphemeralSecret::random_from_rng(OsRng);
    let _pubkey2 = PublicKey::from(&secret2);
}
