//! `TicketTransferred` event on direct transfers (issue #115).
//!
//! A direct transfer changes a ticket's owner, so indexers need an event that
//! carries both sides of the move. These tests compare the exact XDR event the
//! contract publishes.

use super::*;
use soroban_sdk::Event as _;

#[test]
fn transfer_ticket_publishes_ticket_transferred_with_from_and_to() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    client.transfer_ticket(&owner, &ticket_id, &recipient);

    let expected = TicketTransferred {
        ticket_id,
        from: owner.clone(),
        to: recipient.clone(),
    }
    .to_xdr(&env, &client.address);
    let published = env
        .events()
        .all()
        .events()
        .iter()
        .filter(|event| **event == expected)
        .count();
    assert_eq!(published, 1);
}

#[test]
fn rejected_transfer_publishes_no_ticket_transferred_event() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    let stranger = Address::generate(&env);
    let recipient = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    let result = client.try_transfer_ticket(&stranger, &ticket_id, &recipient);
    assert_eq!(result, Err(Ok(Error::NotOwner)));

    let expected = TicketTransferred {
        ticket_id,
        from: stranger.clone(),
        to: recipient.clone(),
    }
    .to_xdr(&env, &client.address);
    let published = env
        .events()
        .all()
        .events()
        .iter()
        .filter(|event| **event == expected)
        .count();
    assert_eq!(published, 0);
}
