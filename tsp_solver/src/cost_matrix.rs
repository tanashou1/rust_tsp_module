
pub fn create(graph: &[(f32, f32)]) -> Vec<Vec<f32>>{
    let dim = graph.len();
    let mut cost_matrix = vec![vec![0.; dim]; dim];
    for (i, n1) in graph.iter().enumerate(){
        for (j, n2) in graph.iter().enumerate().skip(i){
            let distance = ((n1.0 - n2.0).powf(2.) + (n1.1 - n2.1).powf(2.)).powf(0.5);
            cost_matrix[i][j] = distance;
            cost_matrix[j][i] = distance;
        }
    }
    cost_matrix
}

#[test]
fn test_calc(){
    let graph = vec![(1., 2.), (2., 2.), (10., 10.)];
    let cost = create(&graph);
    println!("cost: {:#?}", cost);
}