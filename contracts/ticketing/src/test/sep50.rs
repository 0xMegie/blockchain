//! SEP-50 read surface: `owner_of`, `balance` and `token_uri` (Issue #110).
//!
//! `balance` reads a per-owner counter that every ownership-changing path
//! updates: minting, direct and batch transfers, gift claims, resale purchases
//! and lottery allocation. Each test below drives one of those paths and
//! checks the counts on both sides.

use super::*;

fn gift_secret(env: &Env) -> (Bytes, BytesN<32>) {
    let secret = Bytes::from_slice(env, b"gift-secret");
    let secret_hash: BytesN<32> = env.crypto().sha256(&secret).to_bytes();
    (secret, secret_hash)
}

#[test]
fn owner_of_follows_the_ticket_owner() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    assert_eq!(client.owner_of(&ticket_id), owner);
    client.transfer_ticket(&owner, &ticket_id, &recipient);
    assert_eq!(client.owner_of(&ticket_id), recipient);
}

#[test]
fn owner_of_unknown_ticket_reports_not_found() {
    let (_env, client, _token, _token_asset, _admin, _organizer) = setup();

    assert_eq!(client.try_owner_of(&99), Err(Ok(Error::TicketNotFound)));
}

#[test]
fn balance_is_zero_for_an_address_with_no_tickets() {
    let (env, client, _token, _token_asset, _admin, _organizer) = setup();
    let stranger = Address::generate(&env);

    assert_eq!(client.balance(&stranger), 0);
}

#[test]
fn balance_counts_issued_tickets() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);
    issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);

    assert_eq!(client.balance(&buyer), 2);
}

#[test]
fn balance_moves_with_a_direct_transfer() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &sender, 1_000);

    client.transfer_ticket(&sender, &ticket_id, &recipient);
    assert_eq!(client.balance(&sender), 0);
    assert_eq!(client.balance(&recipient), 1);
}

#[test]
fn balance_moves_with_a_batch_transfer() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let first = issue_sample_ticket(&env, &client, &organizer, 1, &sender, 1_000);
    let second = issue_sample_ticket(&env, &client, &organizer, 1, &sender, 1_000);
    let mut ticket_ids = Vec::new(&env);
    ticket_ids.push_back(first);
    ticket_ids.push_back(second);

    client.transfer_batch(&sender, &ticket_ids, &recipient);
    assert_eq!(client.balance(&sender), 0);
    assert_eq!(client.balance(&recipient), 2);
}

#[test]
fn balance_moves_with_a_gift_claim() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);
    let (secret, secret_hash) = gift_secret(&env);

    client.create_gift_claim(&owner, &ticket_id, &secret_hash, &5_000u64);
    client.claim_gift(&recipient, &ticket_id, &secret);
    assert_eq!(client.balance(&owner), 0);
    assert_eq!(client.balance(&recipient), 1);
}

#[test]
fn balance_moves_with_a_resale_purchase() {
    let (env, client, token, token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &seller, 1_000);
    client.list_for_resale(&seller, &ticket_id, &1_000i128);
    mint_tokens(&token_asset, &buyer, 5_000);

    client.buy_resale(&buyer, &ticket_id);
    assert_eq!(client.balance(&seller), 0);
    assert_eq!(client.balance(&buyer), 1);
    assert_eq!(token.balance(&buyer), 4_000);
}

#[test]
fn balance_still_counts_checked_in_and_revoked_tickets() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let used = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);
    let revoked = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    client.check_in(&organizer, &used);
    client.revoke_ticket(&organizer, &revoked);
    assert_eq!(client.balance(&owner), 2);
}

#[test]
fn listing_for_resale_does_not_change_the_balance() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    client.list_for_resale(&owner, &ticket_id, &1_000i128);
    assert_eq!(client.balance(&owner), 1);
}

#[test]
fn lottery_winners_are_counted() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let mut entrants = Vec::new(&env);
    let addresses = [
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    ];
    for address in addresses.iter() {
        entrants.push_back(address.clone());
    }

    client.allocate_lottery(
        &organizer,
        &1,
        &entrants,
        &2,
        &String::from_str(&env, "GA"),
        &0,
    );

    let total: u64 = addresses.iter().map(|a| client.balance(a)).sum();
    assert_eq!(total, 2);
}

#[test]
fn token_uri_returns_the_metadata_set_at_issuance() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let uri = String::from_str(&env, "ipfs://bafybeigdyrzt/sep50.json");
    let ticket_id = client.issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(uri.clone()),
    );

    assert_eq!(client.token_uri(&ticket_id), uri);
}
