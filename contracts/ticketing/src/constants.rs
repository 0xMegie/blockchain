//! Contract limits and basis-point denominators.
//!
//! Centralizes the numeric limits used by the ticketing contract so they
//! have one source of truth instead of scattered magic numbers.

/// Ledgers a persistent or instance entry is extended to on each bump
/// (~31 days at 5s/ledger, matches other Soroban tooling defaults).
pub(crate) const LEDGER_BUMP: u32 = 535_679;

/// Remaining TTL, in ledgers, below which an entry is bumped.
pub(crate) const LEDGER_THRESHOLD: u32 = 500_000;

/// Maximum number of tickets accepted by the batch entry points.
pub const MAX_BATCH_SIZE: u32 = 50;

/// Ledgers that must pass between proposing and applying a payment token
/// change (~1 day at 5s/ledger).
pub const PAYMENT_TOKEN_CHANGE_DELAY_LEDGERS: u32 = 17_280;

/// Basis-point denominator: 10_000 bps == 100%.
pub const BPS_DENOMINATOR: u32 = 10_000;

/// Seconds after an event's `starts_at` before its used tickets may be burned
/// (issue #113). Thirty days.
pub const USED_TICKET_RETENTION_SECONDS: u64 = 30 * 24 * 60 * 60;

/// Maximum accepted length, in bytes, for the event `name` (Issue #125).
pub const MAX_NAME_LEN: u32 = 64;

/// Maximum accepted length, in bytes, for the event `category` (Issue #125).
pub const MAX_CATEGORY_LEN: u32 = 32;

/// Maximum accepted length, in bytes, for a ticket `tier` or `seat` (Issue #126).
pub const MAX_TICKET_LABEL_LEN: u32 = 32;

/// Maximum accepted length, in bytes, for a ticket metadata URI (Issue #109).
pub const MAX_METADATA_URI_LEN: u32 = 256;

/// Maximum number of event ids returned by one `list_organizer_events` page
/// (Issue #112). Larger requested pages are clamped to this bound.
pub const MAX_PAGE_SIZE: u32 = 50;

/// Stable lowercase labels for the optional common category vocabulary.
pub const CATEGORY_CONCERT: &str = "concert";
pub const CATEGORY_FLIGHT: &str = "flight";
pub const CATEGORY_SPORTS: &str = "sports";
pub const CATEGORY_FESTIVAL: &str = "festival";
pub const CATEGORY_CONFERENCE: &str = "conference";
pub const CATEGORY_OTHER: &str = "other";
