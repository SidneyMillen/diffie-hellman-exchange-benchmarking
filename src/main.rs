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
    use itertools::Itertools;
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
    #[bench]
    fn ten_party_ephemeral_key_benchmark(b: &mut test::Bencher) {
        let mut clients: Vec<TmpKeyClient> = vec![];

        for i in 1..10 {
            clients.push(TmpKeyClient::new(i))
        }

        b.iter(|| {
            //we have to iter over all pairs of clients since each for each exchange we need to generate a new ephemeral key on both clients
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
}
