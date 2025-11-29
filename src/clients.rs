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
