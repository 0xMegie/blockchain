# Changelog

All notable changes to the `ticketing` contract are documented here.
This project follows [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Fixed
- Repair merged organizer approval and packed ticket lookup definitions, and
  update a stale event-creation test so the contract builds again.

### Changed
- Centralize organizer authorization and terminal ticket status checks without
  changing operation-specific revocation rules (#348, #349).
- Keep ticket lifecycle encoding with the types in `types.rs`, preserving
  crate-root exports and byte-identical WASM for the isolated move (#347).
- Upgrade `soroban-sdk` from 26 to 28. See [CONTRIBUTING.md](CONTRIBUTING.md)
  for the dependency versioning policy and upgrade procedure.

### Added
- `burn_used_ticket(organizer, ticket_id)` deletes a checked-in ticket's record
  (and metadata URI) once `starts_at + USED_TICKET_RETENTION_SECONDS` (30 days)
  has passed. Revoked or unused tickets are rejected with `TicketNotUsed`, and
  early calls with `RetentionPeriodNotElapsed` (new codes 48 and 49) (#113).
- `TicketTransferred` event (`ticket_id` topic, `from`, `to`) published by
  `transfer_ticket`. Batch transfers, gift claims and resale purchases do not
  publish it yet (#115).
- `total_tickets()` returns the number of tickets ever issued across all
  events. It only grows, so revoked, used and burned tickets still count (#114).
- Optional ticket metadata URI set at issuance through
  `issue_ticket_with_metadata`, bounded by `MAX_METADATA_URI_LEN` (256 bytes),
  and read back through `token_uri`. `issue_ticket` keeps its signature and
  mints a ticket without a URI (#109).
- Soulbound events: `set_event_transferable(organizer, event_id, false)` makes
  an event's tickets non-transferable across direct transfers, batch transfers,
  gift claims and resale. The setting is locked once a ticket is issued. New
  error `NotTransferable` (#108).
- Paginated organizer event listing `list_organizer_events(organizer, start,
  limit)`, with `limit` clamped to `MAX_PAGE_SIZE` (50) (#112).
- SEP-50 read functions `owner_of`, `balance` and `token_uri`, backed by a
  per-owner ticket counter. The SEP-50 review, with its gaps, is in
  [docs/SEP50.md](docs/SEP50.md) (#110). The issue asked for `balance_of`, but
  SEP-50 names the function `balance`, so the contract follows the standard.
- Status-only scanner query `is_ticket_valid(ticket_id) -> Result<bool, Error>`
  with lifecycle, authorization-free read, and TTL tests. The existing
  owner-aware `is_valid(ticket_id, owner)` ABI is preserved (#350).
- Admin-managed organizer allowlist (`approve_organizer`,
  `revoke_organizer`, `is_approved_organizer`). `create_event` and
  `create_event_with_options` now reject callers that are not approved
  with `OrganizerNotApproved`, which stops event-id squatting (#135). See
  [docs/ORGANIZER_ALLOWLIST.md](docs/ORGANIZER_ALLOWLIST.md).
- Authorization test matrix covering every mutating entry point (#136).
- Initial `ticketing` contract: event registration, issuance, primary
  sale, transfer, on-chain verification, check-in, revocation, and a
  resale marketplace with anti-scalping price caps and organizer
  royalties.
