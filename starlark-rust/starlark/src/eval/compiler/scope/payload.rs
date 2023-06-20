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

//! AST payload for type checking.

// We use CST as acronym for compiler-specific AST.

use std::collections::HashMap;

use dupe::OptionDupedExt;

use crate::eval::compiler::scope::BindingId;
use crate::eval::compiler::scope::ModuleScopeData;
use crate::eval::compiler::scope::ResolvedIdent;
use crate::eval::compiler::scope::ScopeId;
use crate::syntax::ast::AstArgumentP;
use crate::syntax::ast::AstAssignIdentP;
use crate::syntax::ast::AstAssignP;
use crate::syntax::ast::AstExprP;
use crate::syntax::ast::AstIdentP;
use crate::syntax::ast::AstLoadP;
use crate::syntax::ast::AstNoPayload;
use crate::syntax::ast::AstParameterP;
use crate::syntax::ast::AstPayload;
use crate::syntax::ast::AstStmtP;
use crate::syntax::ast::AstTypeExprP;
use crate::syntax::payload_map::AstPayloadFunction;
use crate::typing::Interface;
use crate::values::typing::TypeCompiled;
use crate::values::FrozenValue;

/// Compiler-specific AST payload.
#[derive(Debug)]
pub(crate) struct CstPayload;
impl AstPayload for CstPayload {
    type LoadPayload = Interface;
    /// Information about how identifier binding is resolved.
    ///
    /// This is `None` when CST is created.
    /// All payload objects are filled with binding ids for all assign idents
    /// during analysis.
    ///
    /// When compilation starts, all payloads are `Some`.
    type IdentPayload = Option<ResolvedIdent>;
    /// Binding for an identifier in assignment position.
    ///
    /// This is `None` when CST is created.
    /// All payload objects are filled with binding ids for all assign idents
    /// during analysis.
    ///
    /// When compilation starts, all payloads are `Some`.
    type IdentAssignPayload = Option<BindingId>;
    type DefPayload = ScopeId;
    /// Populated before evaluation of top level statements.
    type TypeExprPayload = Option<TypeCompiled<FrozenValue>>;
}

pub(crate) struct CompilerAstMap<'a, 'f> {
    pub(crate) scope_data: &'a mut ModuleScopeData<'f>,
    pub(crate) loads: &'a HashMap<String, Interface>,
}

impl AstPayloadFunction<AstNoPayload, CstPayload> for CompilerAstMap<'_, '_> {
    fn map_load(&mut self, import_path: &str, (): ()) -> Interface {
        self.loads
            .get(import_path)
            .duped()
            .unwrap_or_else(Interface::empty)
    }

    fn map_ident(&mut self, (): ()) -> Option<ResolvedIdent> {
        None
    }

    fn map_ident_assign(&mut self, (): ()) -> Option<BindingId> {
        None
    }

    fn map_def(&mut self, (): ()) -> ScopeId {
        self.scope_data.new_scope().0
    }

    fn map_type_expr(&mut self, (): ()) -> Option<TypeCompiled<FrozenValue>> {
        None
    }
}

pub(crate) type CstExpr = AstExprP<CstPayload>;
pub(crate) type CstTypeExpr = AstTypeExprP<CstPayload>;
pub(crate) type CstAssign = AstAssignP<CstPayload>;
pub(crate) type CstAssignIdent = AstAssignIdentP<CstPayload>;
pub(crate) type CstIdent = AstIdentP<CstPayload>;
pub(crate) type CstArgument = AstArgumentP<CstPayload>;
pub(crate) type CstParameter = AstParameterP<CstPayload>;
pub(crate) type CstStmt = AstStmtP<CstPayload>;
pub(crate) type CstLoad = AstLoadP<CstPayload>;
