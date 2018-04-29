use variables::Array;
use variables::int_var::ValuesIntVar;

constraint_build!(
    struct Propagator = propagator::SumPropagator;
    fn new(coefs: Vec<i32>);
    fn propagate(res: VarType, vars: Array<VarType>)
        where VarType: ValuesIntVar<Type=i32>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::Array;
    use variables::int_var::ValuesIntVar;
    #[derive(Debug, Clone)]

    // !!!!! COEFS > 0
    pub struct SumPropagator {
        coefs: Vec<i32>,
    }
    impl Propagator for SumPropagator {}
    impl SumPropagator {
        pub fn new(coefs: Vec<i32>) -> SumPropagator {
            SumPropagator { coefs: coefs }
        }

        // adding to propagator/constraint information about change view
        // add iter to array and size => len
        // [HarveySchimpf02]
        pub fn propagate<VarType: ValuesIntVar<Type = i32>>(
            &self,
            res: &mut VarType,
            vars: &mut Array<VarType>,
        ) -> Result<PropagationState, VariableError> {
            let _contributions: Vec<_> = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * (var.max() - var.min()))
                .collect();
            let min: i32 = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * var.min())
                .sum();
            let max: i32 = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * var.max())
                .sum();
            let _ = res.weak_upperbound(max)?;
            let _ = res.weak_lowerbound(min)?;
            let f = res.max() - min;
            //println!("{} {} {}")
            if f < 0 {
                return Err(VariableError::DomainWipeout);
            }
            let len = vars.len();
            for i in 0..len {
                unsafe {
                    let var: &mut VarType = array_get_mut!(vars[i]);
                    let bound = (f / self.coefs[i]) + var.min();
                    let _ = var.weak_upperbound(bound)?;
                }
            }
            Ok(PropagationState::FixPoint)
        }
    }
}
