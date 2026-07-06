use mst_kruskal_variants::{Graph, GraphMatrix, Kruskal};

    fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let graph = GraphMatrix::new_random(0..10, 0.5, 1, 100, true, &mut rng)?;
    let mut algo = Kruskal::new(&graph);
    let (mst_edges, total_cost) = algo.run();
    println!("Total Cost: {}", total_cost);
    println!("Edges: {}", mst_edges.len());

    for edge in mst_edges {
        println!("  {} -> {} (cost: {})", edge.from, edge.to, edge.weight);
    }

    Ok(())
}
