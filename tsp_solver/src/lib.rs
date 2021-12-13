use pyo3::prelude::*;
mod cost_matrix;
mod greedy;
mod two_opt;

#[pyfunction]
fn solve(graph: Vec<(f32, f32)>) -> PyResult<Vec<usize>> {
    let cost = cost_matrix::create(&graph);
    let mut route = greedy::solve(&cost);
    two_opt::solve(&cost, &mut route);

    Ok(route)
}

#[pymodule]
fn tsp_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
