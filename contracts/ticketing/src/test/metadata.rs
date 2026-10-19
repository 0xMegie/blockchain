//! Optional ticket metadata URI set at issuance (Issue #109).
//!
//! `issue_ticket_with_metadata` attaches a URI that `token_uri` reads back.
//! The URI is optional, bounded by `MAX_METADATA_URI_LEN`, and never changes
//! a ticket's lifecycle or ownership.

use super::*;

const TICKET_URI: &str = "ipfs://bafybeigdyrzt/ticket.json";

#[test]
fn issue_with_metadata_stores_the_uri() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);

    let ticket_id = client.issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "VIP"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, TICKET_URI)),
    );

    assert_eq!(
        client.token_uri(&ticket_id),
        String::from_str(&env, TICKET_URI)
    );
}

#[test]
fn metadata_does_not_change_ticket_state() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);

    let ticket_id = client.issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "VIP"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, TICKET_URI)),
    );

    let ticket = client.get_ticket(&ticket_id);
    assert_eq!(ticket.owner, buyer);
    assert_eq!(ticket.status, TicketStatus::Valid);
    assert_eq!(ticket.original_price, 1_000);
    assert!(client.is_ticket_valid(&ticket_id));
}

#[test]
fn issue_without_metadata_reports_no_uri() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let ticket_id = issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);

    let result = client.try_token_uri(&ticket_id);
    assert_eq!(result, Err(Ok(Error::MetadataUriNotSet)));
}

#[test]
fn issue_with_none_behaves_like_issue_ticket() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);

    let ticket_id = client.issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "unassigned"),
        &1_000i128,
        &None,
    );

    assert_eq!(
        client.try_token_uri(&ticket_id),
        Err(Ok(Error::MetadataUriNotSet))
    );
}

#[test]
fn rejects_an_empty_uri() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);

    let result = client.try_issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, "")),
    );
    assert_eq!(result, Err(Ok(Error::EmptyNameOrCategory)));
}

#[test]
fn rejects_a_uri_longer_than_the_maximum() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let too_long = "u".repeat(MAX_METADATA_URI_LEN as usize + 1);

    let result = client.try_issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, &too_long)),
    );
    assert_eq!(result, Err(Ok(Error::StringTooLong)));
}

#[test]
fn accepts_a_uri_at_the_maximum_length() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let at_limit = "u".repeat(MAX_METADATA_URI_LEN as usize);

    let ticket_id = client.issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, &at_limit)),
    );

    assert_eq!(
        client.token_uri(&ticket_id),
        String::from_str(&env, &at_limit)
    );
}

#[test]
fn rejected_uri_does_not_consume_a_ticket_id() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let too_long = "u".repeat(MAX_METADATA_URI_LEN as usize + 1);

    let _ = client.try_issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, &too_long)),
    );
    let next_id = issue_sample_ticket(&env, &client, &organizer, 1, &buyer, 1_000);
    assert_eq!(next_id, 0);
}

#[test]
fn issue_with_metadata_is_organizer_only() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let stranger = Address::generate(&env);
    let buyer = Address::generate(&env);

    let result = client.try_issue_ticket_with_metadata(
        &stranger,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, TICKET_URI)),
    );
    assert_eq!(result, Err(Ok(Error::NotOrganizer)));
}

#[test]
fn token_uri_unknown_ticket_reports_not_found() {
    let (_env, client, _token, _token_asset, _admin, _organizer) = setup();

    let result = client.try_token_uri(&99);
    assert_eq!(result, Err(Ok(Error::TicketNotFound)));
}

#[test]
fn extend_ticket_ttl_keeps_the_uri_readable() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let ticket_id = client.issue_ticket_with_metadata(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "A12"),
        &1_000i128,
        &Some(String::from_str(&env, TICKET_URI)),
    );

    client.extend_ticket_ttl(&ticket_id);
    assert_eq!(
        client.token_uri(&ticket_id),
        String::from_str(&env, TICKET_URI)
    );
}
