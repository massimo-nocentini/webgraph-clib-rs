
#include <stdio.h>
#include <stdlib.h>
#include "../cbindgen/webgraphclibrs.h"

int main() {
    printf("Testing webgraph-clib-rs bindings...\n");

    size_t nodes = 0;
    uint64_t arcs = 0;
    const uint8_t graph_path[] = "1k-0001p"; // Example graph path
    size_t len = sizeof(graph_path) - 1; // Exclude null terminator
    dims(graph_path, len, &nodes, &arcs);
    
    printf("Graph has %zu nodes and %llu arcs.\n", nodes, arcs);

    size_t *neighborhood_vec = malloc (nodes * sizeof(size_t));
    
    size_t n = 0;
    size_t node_id = 435; // Example node ID
    neighborhood(graph_path, len, node_id, neighborhood_vec, &n);

    printf("Neighborhood of node %zu (size %zu): ", node_id, n);
    for (size_t i = 0; i < n; i++) {
        printf("%zu ", neighborhood_vec[i]);
    }
    printf("\n");

    free(neighborhood_vec);

    return 0;
}