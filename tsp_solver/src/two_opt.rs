use itertools::Itertools;

struct Edge {
    n1: usize,
    n2: usize,
    ni1: usize,
    ni2: usize,
    cost: f32
}

pub fn solve(cost: &[Vec<f32>], route: &mut Vec<usize>){
    let mut cnt = 0;
    loop {
        let mut has_improved = false;
        let edges = get_edges(cost, route);
        let it = edges.iter().combinations(2);
        for e in it.into_iter(){
            if e[0].n1 == e[1].n2 || e[1].n1 == e[0].n2{
                continue
            }
            let org_cost = e[0].cost + e[1].cost;
            let new_cost = cost[e[0].n1][e[1].n1] + cost[e[0].n2][e[1].n2];

            if new_cost < org_cost{
                update_route(route, e[0], e[1]);
                has_improved = true;
                cnt += 1;
                break
            }
        }

        if !has_improved{break}
    }

    // println!("two-opt: {}", cnt);
}

fn update_route(route: &mut Vec<usize>, e1: &Edge, e2: &Edge){
    let n1 = e1.ni1;
    let n2 = e2.ni1;

    let drain = route.drain(n1+1..n2+1).collect::<Vec<usize>>().into_iter().rev();
    route.splice(n1+1..n1+1, drain);
}

fn get_edges(cost_matrix: &[Vec<f32>], route: &[usize]) -> Vec<Edge>{
    let mut edge_list = Vec::new();
    for i in 1..cost_matrix.len(){
        let n1 = route[i-1];
        let ni1 = i-1;
        let n2 = route[i];
        let ni2 = i;
        let cost = cost_matrix[n1][n2];
        let edge = Edge{n1, n2, ni1, ni2, cost};
        edge_list.push(edge);
    }

    let n1 = *route.last().unwrap();
    let ni1 = route.len() - 1;
    let n2 = route[0];
    let ni2 = 0;
    let cost = cost_matrix[n1][n2];
    let edge = Edge{n1, n2, ni1, ni2, cost};
    edge_list.push(edge);
    edge_list
}

#[test]
fn test_two_opt(){
    let graph = vec![(1., 1.), (1., 2.), (1., 3.), (3., 3.), (3., 2.), (3., 1.)];
    let cost = super::cost_matrix::create(&graph);
    let mut route = vec![1, 0, 2, 4, 3, 5];
    solve(&cost, &mut route);
    println!("cost: {:#?}", route);
}
