use mst_kruskal_variants::{Graph, GraphMatrix, Kruskal};

// Library example usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let graph = GraphMatrix::new_random(0..10, 0.5, 1, 100, true, &mut rng)?;

    println!(
        "Generated a random graph with {} vertices.",
        graph.num_vertices()
    );

    let mut algo = Kruskal::new(&graph);

    let (mst_edges, total_cost) = algo.run();

    println!("MST Calculation complete.");
    println!("Total Cost: {}", total_cost);
    println!("Edges in MST: {}", mst_edges.len());

    for edge in mst_edges {
        println!("  {} -> {} (cost: {})", edge.from, edge.to, edge.weight);
    }

    Ok(())
}
