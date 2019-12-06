use crate::matrix::Matrix;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operator {
    Multiply,
    Add,
    Subtract,
}

impl Display for Operator {
    /// Format Operator for display
    ///
    /// # Arguments
    /// * self - reference to this Operator
    /// * f - formatter to write to
    ///
    /// # Return
    /// The result of the write
    ///
    /// Author: Kendric Thompson
    fn fmt(&self, f: &mut Formatter) -> Result {
        let output = match self {
            Operator::Multiply => "Multiplied by\n",
            Operator::Add => "Added to\n",
            Operator::Subtract => "Minus\n",
        };
        write!(f, "{}", output)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    left_operand: Matrix,
    operator: Operator,
    right_operand: Matrix,
    result: (RefCell<Option<Matrix>>),
}

impl Operation {
    /// Do Operation, based on operator set
    ///
    /// # Arguments
    /// * self - reference to this Operation
    ///
    /// # Returns
    /// Returns the Matrix resulting from this operation
    ///
    /// Author: Matthew Krohn
    pub fn do_operation(&self) -> Matrix {
        match self.operator {
            Operator::Add => self.left_operand.add_mat(&self.right_operand),
            Operator::Subtract => self.left_operand.sub_mat(&self.right_operand),
            Operator::Multiply => self.left_operand.mul_mat(&self.right_operand),
            //			_ => {
            //				println!("Other operation");
            //				Matrix::new(0, 0, vec![])
            //			}
        }
    }

    /// Do operation, and then replace the stored result
    ///
    /// # Arguments
    /// * self - reference to this Operation
    ///
    /// Author: Matthew Krohn
    pub fn do_operation_and_store(&self) {
        let matr = self.do_operation();
        self.result.replace(Some(matr));
    }
}

impl Display for Operation {
    /// Format Operation for display
    ///
    /// # Arguments
    /// * self - reference to this Operation
    /// * f - formatter to write to
    ///
    /// # Returns
    /// The result of the write operation
    ///
    /// Author: Jennifer Kulich
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut output = String::new();
        output = format!(
            "{}{}\n{}\n{}",
            output, self.left_operand, self.operator, self.right_operand
        );
        let borrowed_option = self.result.borrow();
        if let Some(matr) = &*borrowed_option {
            output = format!("\n{}\n{}\n\n{}", output, "Equals", matr);
        }
        write!(f, "{}", output)
    }
}
