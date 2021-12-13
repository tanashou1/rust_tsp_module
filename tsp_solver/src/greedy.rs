
pub fn solve(cost: &[Vec<f32>]) -> Vec<usize>{
    let dim = cost.len();

    let mut remaining = (1..dim).collect::<Vec<usize>>();
    let mut current = 0;
    let mut route = vec![current];

    while !remaining.is_empty() {
        let next_node_index = calc_next_node_index(cost, current, &remaining);
        route.push(remaining.remove(next_node_index));
        current = *route.last().unwrap();
    }
    route
}

fn calc_next_node_index(cost: &[Vec<f32>], current: usize, remaining: &[usize]) -> usize{
    let mut min_distance = f32::INFINITY;
    let mut next_node_index = current;
    let n1 = current;
    for (i, &n2) in remaining.iter().enumerate() {
        if cost[n1][n2] < min_distance {
            min_distance = cost[n1][n2];
            next_node_index = i;
        }
    }
    next_node_index
}

#[test]
fn test_greedy(){
    let graph = vec![(1., 1.), (1., 2.), (2., 2.), (2., 1.)];
    let cost = super::cost_matrix::create(&graph);
    let route = solve(&cost);
    println!("cost: {:#?}", route);
}