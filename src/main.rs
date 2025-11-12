#![feature(test)]
extern crate test;

mod clients;
use clients::*;

fn main() {
    let alice = StaticKeyClient::new();
    let bob = StaticKeyClient::new();
    let cindy = StaticKeyClient::new();

    let ab_secret = alice.calculate_shared_secret(&bob.pubkey);
    let ba_secret = bob.calculate_shared_secret(&alice.pubkey);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn ten_party_static_key_benchmark(b: &mut test::Bencher) {
        let mut clients: Vec<StaticKeyClient> = vec![];

        for _ in 1..10 {
            clients.push(StaticKeyClient::new())
        }

        b.iter(|| {
            // iterate through each pair of clients.
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
}
