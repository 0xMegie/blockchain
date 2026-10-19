//! Soulbound (non-transferable) events (Issue #108).
//!
//! When an organizer marks an event non-transferable, its tickets cannot be
//! moved: direct transfers, batch transfers, gift claims and resale listings
//! all reject with `NotTransferable`. The flag can only be changed before any
//! ticket is issued, so no buyer ends up holding a ticket on different terms.

use super::*;

fn soulbound_ticket(
    env: &Env,
    client: &TicketingContractClient,
    organizer: &Address,
) -> (Address, u64) {
    make_event(env, client, organizer, 1);
    client.set_event_transferable(organizer, &1, &false);
    let owner = Address::generate(env);
    let ticket_id = issue_sample_ticket(env, client, organizer, 1, &owner, 1_000);
    (owner, ticket_id)
}

#[test]
fn events_are_transferable_by_default() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);

    assert!(client.get_event(&1).transferable);
}

#[test]
fn organizer_can_mark_an_event_soulbound() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);

    client.set_event_transferable(&organizer, &1, &false);
    assert!(!client.get_event(&1).transferable);
}

#[test]
fn transfer_ticket_rejects_a_soulbound_ticket() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    let (owner, ticket_id) = soulbound_ticket(&env, &client, &organizer);
    let recipient = Address::generate(&env);

    let result = client.try_transfer_ticket(&owner, &ticket_id, &recipient);
    assert_eq!(result, Err(Ok(Error::NotTransferable)));
    assert_eq!(client.get_ticket(&ticket_id).owner, owner);
}

#[test]
fn transfer_batch_rejects_a_soulbound_ticket() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    let (owner, ticket_id) = soulbound_ticket(&env, &client, &organizer);
    let recipient = Address::generate(&env);
    let mut ticket_ids = Vec::new(&env);
    ticket_ids.push_back(ticket_id);

    let result = client.try_transfer_batch(&owner, &ticket_ids, &recipient);
    assert_eq!(result, Err(Ok(Error::NotTransferable)));
    assert_eq!(client.get_ticket(&ticket_id).owner, owner);
}

#[test]
fn gift_claim_rejects_a_soulbound_ticket() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    let (owner, ticket_id) = soulbound_ticket(&env, &client, &organizer);
    let secret = Bytes::from_slice(&env, b"gift-secret");
    let secret_hash: BytesN<32> = env.crypto().sha256(&secret).to_bytes();

    let result = client.try_create_gift_claim(&owner, &ticket_id, &secret_hash, &5_000u64);
    assert_eq!(result, Err(Ok(Error::NotTransferable)));
}

#[test]
fn resale_listing_rejects_a_soulbound_ticket() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    let (owner, ticket_id) = soulbound_ticket(&env, &client, &organizer);

    let result = client.try_list_for_resale(&owner, &ticket_id, &1_000i128);
    assert_eq!(result, Err(Ok(Error::NotTransferable)));
    assert_eq!(client.get_ticket(&ticket_id).status, TicketStatus::Valid);
}

#[test]
fn set_event_transferable_is_organizer_only() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let stranger = Address::generate(&env);

    let result = client.try_set_event_transferable(&stranger, &1, &false);
    assert_eq!(result, Err(Ok(Error::NotOrganizer)));
}

#[test]
fn set_event_transferable_is_locked_once_tickets_are_issued() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);

    let result = client.try_set_event_transferable(&organizer, &1, &false);
    assert_eq!(result, Err(Ok(Error::TicketsAlreadyIssued)));
    assert!(client.get_event(&1).transferable);
}

#[test]
fn transferable_event_still_allows_transfers() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    client.transfer_ticket(&owner, &ticket_id, &recipient);
    assert_eq!(client.get_ticket(&ticket_id).owner, recipient);
}

#[test]
fn flag_can_be_restored_before_issuance() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_transferable(&organizer, &1, &false);
    client.set_event_transferable(&organizer, &1, &true);
    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    client.transfer_ticket(&owner, &ticket_id, &recipient);
    assert_eq!(client.get_ticket(&ticket_id).owner, recipient);
}
