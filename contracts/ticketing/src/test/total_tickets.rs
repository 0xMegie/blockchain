//! Global issued-ticket count (issue #114).
//!
//! `total_tickets` reads the ticket id counter, so it counts every ticket
//! minted across all events. Revoking or checking in a ticket does not lower
//! it.

use super::*;

#[test]
fn total_tickets_is_zero_before_any_ticket_is_issued() {
    let (_env, client, _token, _token_asset, _admin, _organizer) = setup();

    assert_eq!(client.total_tickets(), 0);
}

#[test]
fn total_tickets_counts_tickets_across_events() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    make_event(&env, &client, &organizer, 2);
    let buyer = Address::generate(&env);
    issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);
    issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);
    issue_sample_ticket(&env, &client, &organizer, 2, &buyer, 1_000);

    assert_eq!(client.total_tickets(), 3);
}

#[test]
fn total_tickets_does_not_drop_when_tickets_are_revoked_or_checked_in() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let revoked = issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);
    let used = issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);
    client.revoke_ticket(&organizer, &revoked);
    client.check_in(&organizer, &used);

    assert_eq!(client.total_tickets(), 2);
}
