# Rust Library for MST Kruskal Variations (WIP)

Implementation of various iterations of **Kruskal's Algorithm** for finding Minimum Spanning Trees (MST).

From the **Graph Optimization** course of the University of Milan's CS Master.

## Features

### Graph Representations
*   **`GraphMatrix`**: A flattened adjacency matrix representation. Best for dense graphs or when memory locality is prioritized.
*   **`GraphStars`**: An adjacency list representation (Forward Star). Best for sparse graphs.

### Algorithms
*   **`Kruskal`**: Standard implementation using a Binary Heap.
*   **`FilterKruskal`**: Uses a filtered QuickSelect approach to partition edges.
*   **`QuickSortKruskal`**: A variant using QuickSort logic to process edges.
*   **`SkewedFilterKruskal`**: _(Righini, Righini 2022)_ A filtered Kruskal variant with skewed pivot selection. 
*   **`StarQuickSortKruskal` (SQSK)**: Optimized specifically for the `GraphStars` structure.

## Setup

Ensure you have Rust installed (stable toolchain).

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Loplolo/mst-kruskal-variation.git
    cd mst-library
    ```

2.  **Build the project:**
    It is recommended to build in release to ensure compiler optimizations are applied.
    
    ```bash
    cargo build --release
    ```
4. To run the main example (located in `src/main.rs`):
    ```bash
    cargo run --release
    ```
