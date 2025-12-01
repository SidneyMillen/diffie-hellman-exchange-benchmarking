#![feature(test)]
extern crate test;

mod clients;
use clients::*;
use x25519_dalek::{PublicKey, SharedSecret};

fn main() {
    ten_party_static_key_example();
    ten_party_ephemeral_key_example();
    n_party_mutual_key_example(10);
}

fn ten_party_static_key_example() {
    let mut clients: Vec<StaticKeyClient> = vec![];

    for _ in 0..10 {
        clients.push(StaticKeyClient::new())
    }

    // iterate through each pair of clients, calculating shared secrets but not regenerating keypairs (10 total keypairs generated)
    for client in &clients {
        for other_client in &clients {
            if other_client.pubkey == client.pubkey {
                break;
            }
            //generate a shared secret for each pair of clients. each client needs to compute the secret from the other client's pubkey
            client.calculate_shared_secret(&other_client.pubkey);
        }
    }
}

fn ten_party_ephemeral_key_example() {
    let mut clients: Vec<TmpKeyClient> = vec![];

    for i in 0..10 {
        clients.push(TmpKeyClient::new(i));
    }

    //we have to iter over all pairs of clients since each for each exchange we need to generate a new ephemeral key on both clients
    //for 10 clients that is 9+8+7+...+1 iterations, or 45 total keypairs generated
    for i in 0..clients.len() {
        for j in (i + 1)..clients.len() {
            // Split the slice to get two mutable references
            let (left, right) = clients.split_at_mut(j);
            let client = &mut left[i];
            let other_client = &mut right[0];

            tmp_client_key_exchange(client, other_client);
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

    #[bench]
    fn ten_party_static_keypair_generation_benchmark(b: &mut Bencher) {
        b.iter(|| {
            let mut clients: Vec<StaticKeyClient> = vec![];
            for _ in 0..10 {
                clients.push(StaticKeyClient::new())
            }
        });
    }
    #[bench]
    fn ten_party_static_key_benchmark(b: &mut Bencher) {
        b.iter(|| {
            let mut clients: Vec<StaticKeyClient> = vec![];
            for _ in 0..10 {
                clients.push(StaticKeyClient::new())
            }
            // iterate through each pair of clients, calculating shared secrets but not regenerating keypairs (10 total keypairs generated)
            for client in &clients {
                for other_client in &clients {
                    if other_client.pubkey == client.pubkey {
                        break;
                    }
                    //generate a shared secret for each pair of clients. each client needs to compute the secret from the other client's pubkey
                    client.calculate_shared_secret(&other_client.pubkey);
                }
            }
        });
    }
    #[bench]
    fn ten_party_ephemeral_keypair_generation_benchmark(b: &mut Bencher) {
        b.iter(|| {
            let mut clients: Vec<TmpKeyClient> = vec![];

            for i in 0..10 {
                clients.push(TmpKeyClient::new(i))
            }
            //for 10 clients that is 9+8+7+...+1 iterations, or 45 exchanges for 90 keypairs generated
            for i in 0..clients.len() {
                for j in (i + 1)..clients.len() {
                    // do the same iteration as below to minimize code differences
                    // let (left, right) = clients.split_at_mut(j);
                    // let client = &mut left[i];
                    // let other_client = &mut right[0];

                    tmp_client_key_generation();
                }
            }
        });
    }
    #[bench]
    fn ten_party_ephemeral_key_benchmark(b: &mut Bencher) {
        b.iter(|| {
            let mut clients: Vec<TmpKeyClient> = vec![];

            for i in 0..10 {
                clients.push(TmpKeyClient::new(i))
            }
            //we have to iter over all pairs of clients since each for each exchange we need to generate a new ephemeral key on both clients
            //for 10 clients that is 9+8+7+...+1 iterations, or 45 exchanges for 90 keypairs generated
            for i in 0..clients.len() {
                for j in (i + 1)..clients.len() {
                    // Split the slice to get two mutable references
                    let (left, right) = clients.split_at_mut(j);
                    let client = &mut left[i];
                    let other_client = &mut right[0];

                    tmp_client_key_exchange(client, other_client);
                }
            }
        });
    }
    #[bench]
    fn n_party_mutual_secret_static_key_benchmark(b: &mut Bencher) {
        b.iter(|| {
            let n = 10;
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
                }
            }
        });
    }
}
