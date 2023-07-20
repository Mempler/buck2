/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt;
use std::fmt::Display;

use allocative::Allocative;

use crate::typing::custom::TyCustom;
use crate::typing::starlark_value::TyStarlarkValue;
use crate::typing::Ty;
use crate::typing::TyName;
use crate::typing::TypingAttr;
use crate::typing::TypingOracleCtx;
use crate::values::none::NoneType;
use crate::values::string::StarlarkStr;
use crate::values::StarlarkValue;

/// Type that is not a union.
#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd, Allocative)]
pub enum TyBasic {
    /// Type that contain anything
    Any,
    /// A name, represented by `"name"` in the Starlark type.
    /// Will never be a type that can be represented by another operation,
    /// e.g. never `"list"` because `Ty::List` could be used instead.
    Name(TyName),
    /// Type is handled by `StarlarkValue` trait implementation.
    StarlarkValue(TyStarlarkValue),
    /// Iter is a type that supports iteration, only used as arguments to primitive functions.
    /// The inner type is applicable for each iteration element.
    Iter(Box<Ty>),
    /// A list.
    List(Box<Ty>),
    /// A tuple. May be empty, to indicate the empty tuple.
    Tuple(Vec<Ty>),
    /// A dictionary, with key and value types
    Dict(Box<(Ty, Ty)>),
    /// Custom type.
    Custom(TyCustom),
}

impl TyBasic {
    pub(crate) fn none() -> TyBasic {
        TyBasic::starlark_value::<NoneType>()
    }

    pub(crate) fn starlark_value<'v, T: StarlarkValue<'v>>() -> TyBasic {
        TyBasic::StarlarkValue(TyStarlarkValue::new::<T>())
    }

    pub(crate) fn string() -> Self {
        TyBasic::starlark_value::<StarlarkStr>()
    }

    /// Create a list type.
    pub(crate) fn list(element: Ty) -> Self {
        TyBasic::List(Box::new(element))
    }

    /// Create a iterable type.
    pub(crate) fn iter(item: Ty) -> Self {
        TyBasic::Iter(Box::new(item))
    }

    /// Create a dictionary type.
    pub(crate) fn dict(key: Ty, value: Ty) -> Self {
        TyBasic::Dict(Box::new((key, value)))
    }

    /// Turn a type back into a name, potentially erasing some structure.
    /// E.g. the type `[bool]` would return `list`.
    /// Types like [`Ty::any`] will return `None`.
    pub fn as_name(&self) -> Option<&str> {
        match self {
            TyBasic::Name(x) => Some(x.as_str()),
            TyBasic::StarlarkValue(t) => Some(t.as_name()),
            TyBasic::List(_) => Some("list"),
            TyBasic::Tuple(_) => Some("tuple"),
            TyBasic::Dict(_) => Some("dict"),
            TyBasic::Custom(c) => c.as_name(),
            TyBasic::Any | TyBasic::Iter(_) => None,
        }
    }

    /// If I do `self[i]` what will the resulting type be.
    pub(crate) fn indexed(self, i: usize) -> Ty {
        match self {
            TyBasic::Any => Ty::any(),
            TyBasic::List(x) => *x,
            TyBasic::Tuple(xs) => xs.get(i).cloned().unwrap_or(Ty::never()),
            // Not exactly sure what we should do here
            _ => Ty::any(),
        }
    }

    /// See what lies behind an attribute on a type
    pub(crate) fn attribute(&self, attr: TypingAttr, ctx: TypingOracleCtx) -> Result<Ty, ()> {
        // There are some structural types which have to be handled in a specific way
        match self {
            TyBasic::Any => Ok(Ty::any()),
            _ => match ctx.oracle.attribute(self, attr) {
                Some(r) => r,
                None => Ok(Ty::any()),
            },
        }
    }
}

impl Display for TyBasic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TyBasic::Any => write!(f, "\"\""),
            TyBasic::Name(x) => write!(f, "{}", x),
            TyBasic::StarlarkValue(x) => write!(f, "{}", x),
            TyBasic::Iter(x) => write!(f, "iter({})", x),
            TyBasic::List(x) => write!(f, "[{}]", x),
            TyBasic::Tuple(xs) => {
                if xs.len() == 1 {
                    write!(f, "({},)", xs[0])
                } else {
                    display_container::fmt_container(f, "(", ")", xs)
                }
            }
            TyBasic::Dict(k_v) => write!(f, "{{{}: {}}}", k_v.0, k_v.1),
            TyBasic::Custom(c) => Display::fmt(c, f),
        }
    }
}