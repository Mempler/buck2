/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::sync::Arc;

use crate::error::ErrorImpl;
use crate::AnyError;

impl crate::Error {
    pub fn context<C: std::fmt::Display + Send + Sync + 'static>(self, context: C) -> Self {
        Self(Arc::new(ErrorImpl::WithContext(Arc::new(context), self)))
    }

    #[cold]
    fn new_anyhow_with_context<E: AnyError, C: std::fmt::Display + Send + Sync + 'static>(
        e: E,
        c: C,
    ) -> anyhow::Error {
        Into::<Self>::into(e).context(c).into()
    }
}

/// Provides the `context` method for `Result`.
///
/// This trait is analogous to the `anyhow::Context` trait. It is mostly a drop-in replacement, and
/// in the near future, uses of `anyhow::Context` in `buck2/app` will be broadly replaced with use
/// of this trait. Subsequently, additional APIs will be provided for annotating errors with
/// structured context data.
pub trait Context<T>: Sealed {
    fn context<C: std::fmt::Display + Send + Sync + 'static>(self, context: C)
    -> anyhow::Result<T>;

    fn with_context<C, F>(self, f: F) -> anyhow::Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}
pub trait Sealed {}

impl<T, E: AnyError> Sealed for std::result::Result<T, E> {}

impl<T, E: AnyError> Context<T> for std::result::Result<T, E> {
    fn context<C>(self, c: C) -> anyhow::Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(crate::Error::new_anyhow_with_context(e, c)),
        }
    }

    fn with_context<C, F>(self, f: F) -> anyhow::Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(crate::Error::new_anyhow_with_context(e, f())),
        }
    }
}

// FIXME(JakobDegen): This impl should not exist. We should make people write error types for these
// conditions. Let's have it for now to ease migration though.

// Can't use our derive macro because it creates reference to `::buck2_error`, which doesn't exist in this
// crate
#[derive(Debug, buck2_error_derive::Error)]
#[error("NoneError")]
struct NoneError;

impl<T> Sealed for Option<T> {}

impl<T> Context<T> for Option<T> {
    fn context<C>(self, c: C) -> anyhow::Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Some(x) => Ok(x),
            None => Err(crate::Error::new_anyhow_with_context(NoneError, c)),
        }
    }

    fn with_context<C, F>(self, f: F) -> anyhow::Result<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(x) => Ok(x),
            None => Err(crate::Error::new_anyhow_with_context(NoneError, f())),
        }
    }
}
