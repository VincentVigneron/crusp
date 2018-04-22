use constraints::Propagator;
use variables::Array;
use variables::int_var::BoundsIntVar;

constraint_build!(
    struct Propagator = IncreasingPropagator;
    fn new();
    fn propagate<VarType: BoundsIntVar>(x: Array<VarType>) -> ();
    );

#[derive(Debug, Clone)]
pub struct IncreasingPropagator {}
impl Propagator for IncreasingPropagator {}
impl IncreasingPropagator {
    pub fn new() -> IncreasingPropagator {
        IncreasingPropagator {}
    }

    pub fn propagate<VarType: BoundsIntVar>(&self, array: &mut Array<VarType>) {
        let len = array.variables.len();
        for i in 0..(len - 1) {
            unsafe {
                let lhs: &mut VarType = array_get_mut!(array[i]);
                let rhs: &mut VarType = array_get_mut!(array[i + 1]);
                let _ = lhs.less_than(rhs);
            }
        }
        for i in 0..(len - 1) {
            unsafe {
                let lhs: &mut VarType = array_get_mut!(array[len - 2 - i]);
                let rhs: &mut VarType = array_get_mut!(array[len - 1 - i]);
                let _ = lhs.less_than(rhs);
            }
        }
    }
}
