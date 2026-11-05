use mst_kruskal_variants::{GraphMatrix, Kruskal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let graph = GraphMatrix::new_random(0..10, 0.5, 1..=100, &mut rng)?;
    let mst = Kruskal::new(&graph).run();

    println!("Total cost: {}", mst.cost);
    println!("Edges: {}", mst.edges.len());
    for edge in &mst.edges {
        println!("  {} -> {} (cost: {})", edge.from, edge.to, edge.weight);
    }
    Ok(())
}
