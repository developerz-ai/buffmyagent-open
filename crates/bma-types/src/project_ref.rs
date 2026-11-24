//! A project as it is written on the wire: `<org>/<project>`, two slugs.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

const SLUG_MIN: usize = 2;
const SLUG_MAX: usize = 39;

/// Why a string is not a project reference. It never carries the string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ProjectRefError {
    /// Not exactly two parts around one `/`.
    #[error("a project reference is <org>/<project>")]
    Shape,
    /// A part is not a slug.
    #[error(
        "a slug is 2 to 39 characters of a-z, 0-9 and single hyphens, and starts and ends with a letter or digit"
    )]
    Slug,
}

/// A project named by its organisation slug and its own slug. Every call is scoped to one.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProjectRef {
    org: String,
    project: String,
}

fn is_slug(text: &str) -> bool {
    let bytes = text.as_bytes();
    let edge = |byte: &u8| byte.is_ascii_lowercase() || byte.is_ascii_digit();
    (SLUG_MIN..=SLUG_MAX).contains(&bytes.len())
        && bytes.first().is_some_and(edge)
        && bytes.last().is_some_and(edge)
        && bytes.iter().all(|byte| edge(byte) || *byte == b'-')
        && !text.contains("--")
}

impl ProjectRef {
    /// The organisation's slug.
    #[must_use]
    pub fn org(&self) -> &str {
        &self.org
    }

    /// The project's slug.
    #[must_use]
    pub fn project(&self) -> &str {
        &self.project
    }
}

impl FromStr for ProjectRef {
    type Err = ProjectRefError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (org, project) = text.split_once('/').ok_or(ProjectRefError::Shape)?;
        if project.contains('/') {
            return Err(ProjectRefError::Shape);
        }
        if !is_slug(org) || !is_slug(project) {
            return Err(ProjectRefError::Slug);
        }
        Ok(Self {
            org: org.to_owned(),
            project: project.to_owned(),
        })
    }
}

impl fmt::Display for ProjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.org, self.project)
    }
}

impl fmt::Debug for ProjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProjectRef({self})")
    }
}

impl Serialize for ProjectRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for ProjectRef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
