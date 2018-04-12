use branchers::handlers::BranchersHandler;
use constraints::handlers::ConstraintsHandler;
use variables::handlers::VariablesHandler;

#[allow(dead_code)]
#[derive(Clone)]
struct Node<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    variables: Variables,
    constraints: Constraints,
    brancher: Branchers,
}
