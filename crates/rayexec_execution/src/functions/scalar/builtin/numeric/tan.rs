use num_traits::Float;
use rayexec_error::Result;

use super::{UnaryInputNumericOperation, UnaryInputNumericScalar};
use crate::arrays::array::physical_type::PhysicalStorage;
use crate::arrays::array::{Array, ArrayData2};
use crate::arrays::datatype::DataType;
use crate::arrays::executor::builder::{ArrayBuilder, PrimitiveBuffer};
use crate::arrays::executor::scalar::UnaryExecutor;
use crate::arrays::storage::PrimitiveStorage;

pub type Tan = UnaryInputNumericScalar<TanOp>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TanOp;

impl UnaryInputNumericOperation for TanOp {
    const NAME: &'static str = "tan";
    const DESCRIPTION: &'static str = "Compute the tangent of value";

    fn execute_float<'a, S>(input: &'a Array, ret: DataType) -> Result<Array>
    where
        S: PhysicalStorage,
        S::Type<'a>: Float + Default,
        ArrayData2: From<PrimitiveStorage<S::Type<'a>>>,
    {
        let builder = ArrayBuilder {
            datatype: ret,
            buffer: PrimitiveBuffer::with_len(input.logical_len()),
        };
        UnaryExecutor::execute2::<S, _, _>(input, builder, |v, buf| buf.put(&v.tan()))
    }
}
