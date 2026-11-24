//! Types shared by every crate in the workspace, so no crate passes a bare `String` or `u128`
//! where an identifier or a refusal code is meant.
//!
//! - [`Code`]: the one list of refusal codes. Every refusal maps to one.
//! - [`OrgId`], [`ConnectionId`], [`ItemId`], [`RequestId`]: UUID identifiers.
//! - [`ProjectRef`]: a project as it is written on the wire, `<org>/<project>`.

mod code;
mod id;
mod project_ref;

pub use code::{Code, UnknownCode};
pub use id::{ConnectionId, IdError, ItemId, OrgId, RequestId};
pub use project_ref::{ProjectRef, ProjectRefError};
