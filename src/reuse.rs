use std::cell::RefCell;
use std::collections::RingBuf;
use std::hash::Hash;
use std::hash;
use num::Zero;

use super::{SearchProblem, astar};

struct ReusableSearchProblemWrapper<'a, N, Rsp: 'a> {
    start: RefCell<Option<N>>,
    end: N,
    rsp: &'a Rsp
}

/// ReusableSearchProblem is like a regular SearchProblem but without
/// the `start()` and `is_end()` checks.  Instead, the start and end
/// will be provided when `astar_r()` is called.
pub trait ReusableSearchProblem<N, C, I: Iterator<Item=(N, C)>> {
    /// A function that estimates the cost to get from
    /// a node to the end.
    /// heuristic(end_state) should always be 0.
    #[inline(always)]
    fn heuristic(&self, &N) -> C;
    /// A function returning the neighbors of a search state along
    /// with the cost to get to that state.
    #[inline(always)]
    fn neighbors(&self, at: &N) -> I;
    /// This method is used if an estimated length of the path
    /// is available.
    #[inline(always)]
    fn estimate_length(&self) -> Option<usize> { None }
}

impl <'a, N, C, I: Iterator<Item=(N, C)>, Rsp> SearchProblem<N, C, I> for ReusableSearchProblemWrapper<'a, N, Rsp>
where N: PartialEq, Rsp: ReusableSearchProblem<N, C, I>
{
    fn start(&self) -> N { return self.start.borrow_mut().take().unwrap() }
    fn is_end(&self, node: &N) -> bool { (&self.end) == node }
    fn heuristic(&self, node: &N) -> C { self.rsp.heuristic(node) }
    fn neighbors(&self, node: &N) -> I { self.rsp.neighbors(node) }
}



pub fn astar_r<N, C, I, S: ReusableSearchProblem<N, C, I>,H>(s: &S, start: N, end: N) -> Option<RingBuf<N>>
where H: hash::Hasher+hash::Writer,
      N: Hash<H> + PartialEq,
      C: PartialOrd + Zero + Clone,
      I: Iterator<Item=(N, C)>
{
    let rspw = ReusableSearchProblemWrapper {
        start: RefCell::new(Some(start)),
        end: end,
        rsp: s
    };

    astar(rspw)
}


