//! Event capacity limits (issue #92).
//!
//! `max_tickets` caps how many tickets an event may ever issue. Every primary
//! mint path (`issue_ticket`, `purchase_primary`, `allocate_lottery`) checks
//! the cap against `tickets_issued`, so the boundary is the last ticket that
//! still fits.

use super::*;

fn ga(env: &Env) -> String {
    String::from_str(env, "GA")
}

#[test]
fn issue_ticket_stops_at_the_capacity_boundary() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_capacity(&organizer, &1, &Some(2u64));
    let owner = Address::generate(&env);

    issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);
    issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    assert_eq!(
        client.try_issue_ticket(
            &organizer,
            &1,
            &owner,
            &ga(&env),
            &String::from_str(&env, "A3"),
            &1_000i128,
        ),
        Err(Ok(Error::SoldOut))
    );
    assert_eq!(client.get_event(&1).tickets_issued, 2);
}

#[test]
fn purchase_primary_stops_at_the_capacity_boundary() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_capacity(&organizer, &1, &Some(1u64));
    client.set_tier_price(&organizer, &1, &ga(&env), &0i128);
    let first = Address::generate(&env);
    let second = Address::generate(&env);

    client.purchase_primary(&first, &1, &ga(&env), &String::from_str(&env, "A1"));

    assert_eq!(
        client.try_purchase_primary(&second, &1, &ga(&env), &String::from_str(&env, "A2")),
        Err(Ok(Error::SoldOut))
    );
    assert_eq!(client.get_event(&1).tickets_issued, 1);
}

#[test]
fn lottery_rejects_a_draw_larger_than_the_remaining_capacity() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_event_capacity(&organizer, &1, &Some(2u64));
    let entrants = soroban_sdk::vec![
        &env,
        Address::generate(&env),
        Address::generate(&env),
        Address::generate(&env),
    ];

    assert_eq!(
        client.try_allocate_lottery(&organizer, &1, &entrants, &3u32, &ga(&env), &0i128),
        Err(Ok(Error::SoldOut))
    );

    // Exactly filling the remaining capacity is still allowed.
    let winners = client.allocate_lottery(&organizer, &1, &entrants, &2u32, &ga(&env), &0i128);
    assert_eq!(winners.len(), 2);
    assert_eq!(client.get_event(&1).tickets_issued, 2);
}

#[test]
fn capacity_cannot_be_set_below_tickets_already_issued() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let owner = Address::generate(&env);
    issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);
    issue_sample_ticket(&env, &client, &organizer, 1, &owner, 1_000);

    assert_eq!(
        client.try_set_event_capacity(&organizer, &1, &Some(1u64)),
        Err(Ok(Error::InvalidCapacity))
    );
    // Equal to the issued count leaves no room but is not below it.
    client.set_event_capacity(&organizer, &1, &Some(2u64));
}

#[test]
fn zero_capacity_is_rejected() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);

    assert_eq!(
        client.try_set_event_capacity(&organizer, &1, &Some(0u64)),
        Err(Ok(Error::InvalidCapacity))
    );
}

#[test]
fn only_the_organizer_can_set_capacity() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let stranger = Address::generate(&env);

    assert_eq!(
        client.try_set_event_capacity(&stranger, &1, &Some(5u64)),
        Err(Ok(Error::NotOrganizer))
    );
}
