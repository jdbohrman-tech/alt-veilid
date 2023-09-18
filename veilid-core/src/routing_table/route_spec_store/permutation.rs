use super::*;

/// number of route permutations is the number of unique orderings
/// for a set of nodes, given that the first node is fixed
fn _get_route_permutation_count(hop_count: usize) -> usize {
    if hop_count == 0 {
        unreachable!();
    }
    // a single node or two nodes is always fixed
    if hop_count == 1 || hop_count == 2 {
        return 1;
    }
    // more than two nodes has factorial permutation
    // hop_count = 3 -> 2! -> 2
    // hop_count = 4 -> 3! -> 6
    (3..hop_count).fold(2usize, |acc, x| acc * x)
}
pub type PermReturnType = (Vec<usize>, bool);
pub type PermFunc<'t> = Box<dyn FnMut(&[usize]) -> Option<PermReturnType> + Send + 't>;

/// get the route permutation at particular 'perm' index, starting at the 'start' index
/// for a set of 'hop_count' nodes. the first node is always fixed, and the maximum
/// number of permutations is given by get_route_permutation_count()

pub fn with_route_permutations(
    hop_count: usize,
    start: usize,
    f: &mut PermFunc,
) -> Option<PermReturnType> {
    if hop_count == 0 {
        unreachable!();
    }
    // initial permutation
    let mut permutation: Vec<usize> = Vec::with_capacity(hop_count);
    for n in 0..hop_count {
        permutation.push(start + n);
    }
    // if we have one hop or two, then there's only one permutation
    if hop_count == 1 || hop_count == 2 {
        return f(&permutation);
    }

    // heaps algorithm, but skipping the first element
    fn heaps_permutation(
        permutation: &mut [usize],
        size: usize,
        f: &mut PermFunc,
    ) -> Option<PermReturnType> {
        if size == 1 {
            return f(permutation);
        }

        for i in 0..size {
            let out = heaps_permutation(permutation, size - 1, f);
            if out.is_some() {
                return out;
            }
            if size % 2 == 1 {
                permutation.swap(1, size);
            } else {
                permutation.swap(1 + i, size);
            }
        }

        None
    }

    // recurse
    heaps_permutation(&mut permutation, hop_count - 1, f)
}
