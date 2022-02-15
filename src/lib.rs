// Copyright 2021 Nosrac

//! # NosracID
//!
//! Provides an extensible and flexible digital identity platform.
//! Designed with privacy in mind so that users can securely share aspects of their identify as needed.
//!
//! ### Key Features Include:
//! - Privacy
//! - Performance
//! - Flexibility


/// Basic Id structure for an individual
pub mod id;

/// Certified extensions of Id's.
/// Represents optional fields that an individual user may want to associate with their identity, but does not apply to everyone.
pub mod extension;

