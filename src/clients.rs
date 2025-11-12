use rand::rngs::OsRng;
use x25519_dalek::*;

pub struct StaticKeyClient {
    secret: StaticSecret,
    pub pubkey: PublicKey,
}

impl StaticKeyClient {
    pub fn new() -> Self {
        let csprng = OsRng;
        let secret = StaticSecret::random_from_rng(csprng);
        let pubkey = PublicKey::from(&secret);
        StaticKeyClient { secret, pubkey }
    }
    pub fn calculate_shared_secret(&self, other_pubkey: &PublicKey) -> SharedSecret {
        self.secret.diffie_hellman(other_pubkey)
    }
}
