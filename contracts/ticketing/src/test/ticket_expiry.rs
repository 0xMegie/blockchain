//! Ticket expiry (issue #76).
//!
//! `ticket_expires_at` ends a ticket's validity. From that instant on,
//! `check_in` and every transfer path reject the ticket with `TicketExpired`.

use super::*;

const EXPIRES_AT: u64 = 5_000;

fn issued_ticket(env: &Env, client: &TicketingContractClient, organizer: &Address) -> u64 {
    issue_sample_ticket(env, client, organizer, 1, &Address::generate(env), 1_000)
}

#[test]
fn check_in_is_rejected_from_the_expiry_instant() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_ticket_expiry(&organizer, &1, &Some(EXPIRES_AT));
    let before_expiry = issued_ticket(&env, &client, &organizer);
    let at_expiry = issued_ticket(&env, &client, &organizer);

    env.ledger().set_timestamp(EXPIRES_AT - 1);
    client.check_in(&organizer, &before_expiry);

    env.ledger().set_timestamp(EXPIRES_AT);
    assert_eq!(
        client.try_check_in(&organizer, &at_expiry),
        Err(Ok(Error::TicketExpired))
    );
}

#[test]
fn transfer_is_rejected_once_the_ticket_has_expired() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_ticket_expiry(&organizer, &1, &Some(EXPIRES_AT));
    let owner = Address::generate(&env);
    let friend = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    env.ledger().set_timestamp(EXPIRES_AT);
    assert_eq!(
        client.try_transfer_ticket(&owner, &ticket_id, &friend),
        Err(Ok(Error::TicketExpired))
    );
    assert_eq!(client.owner_of(&ticket_id), owner);
}

#[test]
fn transfer_is_allowed_up_to_the_expiry_instant() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_ticket_expiry(&organizer, &1, &Some(EXPIRES_AT));
    let owner = Address::generate(&env);
    let friend = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    env.ledger().set_timestamp(EXPIRES_AT - 1);
    client.transfer_ticket(&owner, &ticket_id, &friend);

    assert_eq!(client.owner_of(&ticket_id), friend);
}

#[test]
fn expiry_must_be_in_the_future_when_set() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    env.ledger().set_timestamp(100);

    assert_eq!(
        client.try_set_event_ticket_expiry(&organizer, &1, &Some(100u64)),
        Err(Ok(Error::InvalidTicketExpiry))
    );
    client.set_event_ticket_expiry(&organizer, &1, &Some(101u64));
}

#[test]
fn clearing_the_expiry_lets_tickets_be_checked_in_again() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_ticket_expiry(&organizer, &1, &Some(EXPIRES_AT));
    let ticket_id = issued_ticket(&env, &client, &organizer);

    env.ledger().set_timestamp(EXPIRES_AT + 1);
    client.set_event_ticket_expiry(&organizer, &1, &None);
    client.check_in(&organizer, &ticket_id);

    assert_eq!(client.get_ticket(&ticket_id).status, TicketStatus::Used);
}
