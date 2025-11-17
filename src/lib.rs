use libc::size_t;
use webgraph::prelude::*;

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
fn from_char_buff<'a>(ptr: *const u8, len: size_t) -> Option<&'a str> {
    // SAFETY: We need to make sure that pointer is not null.
    if ptr.is_null() {
        None
    } else {
        unsafe {
            let buff = std::slice::from_raw_parts(ptr, len as usize);
            // SAFETY: Using the standard from_utf8 function, we ensure that the sequence is a valid string or NONE.
            // In case of invalid sequence of strings the function returns None.
            // Considering that the pointer is also not null there is no scenario under which the return value of this function won't be a proper string.
            // There is no chance for an undefined behaviour.
            // Therefor the return value of this function is both safe and sound.
            // As a consequence not_unsafe_ptr_arg_deref warning is not relevant and has to be supressed.
            std::str::from_utf8(buff).ok()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dims(str: *const u8, len: size_t, nodes: *mut size_t, arcs: *mut u64) {
    let graph_filename = from_char_buff(str, len).unwrap();
    let graph = BvGraph::with_basename(graph_filename).load().unwrap();

    let n = graph.num_nodes();
    let m = graph.num_arcs();

    unsafe {
        if !nodes.is_null() {
            *nodes = n;
        }
        if !arcs.is_null() {
            *arcs = m;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn neighborhood(
    str: *const u8,
    len: size_t,
    node_id: size_t,
    neighborhood: *mut size_t,
    n: *mut size_t,
) {
    let graph_filename = from_char_buff(str, len).unwrap();
    let graph = BvGraph::with_basename(graph_filename).load().unwrap();

    let mut neighborhood_vec = Vec::new();

    for node in graph.successors(node_id) {
        neighborhood_vec.push(node);
    }

    unsafe {
        if !neighborhood.is_null() {
            for (i, &node) in neighborhood_vec.iter().enumerate() {
                *neighborhood.add(i) = node;
            }
        }

        if !n.is_null() {
            *n = neighborhood_vec.len();
        }
    }
}
