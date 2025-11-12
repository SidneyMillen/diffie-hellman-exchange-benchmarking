use rand::rngs::OsRng;
use x25519_dalek::*;

pub struct StaticKeyClient {
    secret: StaticSecret,
    pub pubkey: PublicKey,
}

impl StaticKeyClient {
    pub fn new() -> Self {
        let secret = StaticSecret::random_from_rng(OsRng);
        let pubkey = PublicKey::from(&secret);
        Self { secret, pubkey }
    }
    pub fn calculate_shared_secret(&self, other_pubkey: &PublicKey) -> SharedSecret {
        self.secret.diffie_hellman(other_pubkey)
    }
}

#[derive(Clone, Copy)]
pub struct ClientID(u8);

pub struct TmpKeyClient {
    shared_secrets: Vec<(SharedSecret, ClientID)>,
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
            .push((s.diffie_hellman(other_pubkey), other_id));
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
