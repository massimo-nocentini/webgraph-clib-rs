
#include <stdio.h>
#include "../cbindgen/webgraphclibrs.h"

int main() {
    printf("Testing webgraph-clib-rs bindings...\n");

    size_t nodes = 0;
    uint64_t arcs = 0;
    const uint8_t graph_path[] = "1k-0001p"; // Example graph path
    size_t len = sizeof(graph_path) - 1; // Exclude null terminator
    dims(graph_path, len, &nodes, &arcs);
    
    printf("Graph has %zu nodes and %llu arcs.\n", nodes, arcs);

    return 0;
}