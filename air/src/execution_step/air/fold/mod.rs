/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod fold_state;
mod utils;

pub(crate) use fold_state::FoldState;
pub(crate) use fold_state::IterableType;
pub(super) use utils::*;

use super::ExecutionCtx;
use super::ExecutionError;
use super::ExecutionResult;
use super::Instruction;
use super::ScalarRef;
use super::ValueAggregate;
use crate::execution_step::boxed_value::*;
