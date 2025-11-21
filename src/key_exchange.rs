use crate::clients::*;
use x25519_dalek::{PublicKey, SharedSecret};

fn n_party_mutual_key_example(n: usize) {
    let mut secrets: Vec<SharedSecret> = vec![];

    let mut clients: Vec<StaticKeyClient> = vec![];

    for _ in 0..n {
        clients.push(StaticKeyClient::new())
    }

    for i in 0..n {
        let client = clients.get(i).unwrap();

        let mut current_secret: PublicKey = client.pubkey;
        for j in 1..n {
            let mut next_client_idx = i + j;
            if next_client_idx > n - 1 {
                next_client_idx -= n;
            }
            let next_client = clients.get(next_client_idx).unwrap();
            let secret = next_client.calculate_shared_secret(&current_secret);
            current_secret = PublicKey::from(secret.to_bytes());
            if j == n - 1 {
                secrets.push(secret);
            }
        }
    }
    let secret_bytes = secrets.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    assert!(secret_bytes.iter().all(|&secret| secret == secret_bytes[0]));
    println!("successfully negotiated shared secret for 10 parties");
}
