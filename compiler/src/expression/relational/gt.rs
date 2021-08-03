// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Enforces a relational `>` operator in a resolved Leo program.

use crate::{value::ConstrainedValue, GroupType};
use leo_errors::{CompilerError, LeoError, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::bits::ComparatorGadget;
use snarkvm_r1cs::ConstraintSystem;

pub fn evaluate_gt<'a, F: PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    left: ConstrainedValue<'a, F, G>,
    right: ConstrainedValue<'a, F, G>,
    span: &Span,
) -> Result<ConstrainedValue<'a, F, G>, LeoError> {
    let unique_namespace = cs.ns(|| format!("evaluate {} > {} {}:{}", left, right, span.line_start, span.col_start));
    let constraint_result = match (left, right) {
        (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
            num_1.greater_than(unique_namespace, &num_2)
        }
        (val_1, val_2) => {
            return Err(LeoError::from(CompilerError::incompatible_types(
                format!("{} > {}", val_1, val_2),
                span,
            )));
        }
    };

    let boolean = constraint_result
        .map_err(|_| LeoError::from(CompilerError::cannot_evaluate_expression(">".to_string(), span)))?;

    Ok(ConstrainedValue::Boolean(boolean))
}
