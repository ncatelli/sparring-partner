use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PunchKind {
    Jab = 1,
    Cross = 2,
    LeadHook = 3,
    RearHook = 4,
    LeadUppercut = 5,
    RearUppercut = 6,
}

impl PunchKind {
    pub fn as_u8(self) -> u8 {
        u8::from(self)
    }
}

impl From<PunchKind> for u8 {
    fn from(punch: PunchKind) -> Self {
        punch as u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Combo(Vec<PunchKind>);

impl Combo {
    pub fn new<T: AsRef<[PunchKind]>>(punches: T) -> Self {
        let owned_punches = punches.as_ref().to_vec();

        Self(owned_punches)
    }
}

pub trait Edge {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weight(usize);

impl Weight {
    pub fn new(weight: usize) -> Self {
        Self(weight)
    }

    pub fn as_usize(self) -> usize {
        self.0
    }
}

impl Edge for Weight {}

impl From<usize> for Weight {
    fn from(weight: usize) -> Self {
        Self::new(weight)
    }
}

impl std::ops::Add<Weight> for Weight {
    type Output = Weight;

    fn add(self, rhs: Weight) -> Self::Output {
        self + rhs.0
    }
}

impl std::ops::Add<usize> for Weight {
    type Output = Weight;

    fn add(self, rhs: usize) -> Self::Output {
        let sum = self.0 + rhs;
        Self::from(sum)
    }
}

impl std::ops::AddAssign<Weight> for Weight {
    fn add_assign(&mut self, rhs: Weight) {
        *self += rhs.0;
    }
}

impl std::ops::AddAssign<usize> for Weight {
    fn add_assign(&mut self, rhs: usize) {
        let sum = self.0 + rhs;
        *self = Self::from(sum);
    }
}

#[derive(Debug)]
pub struct Graph<K, E: Edge> {
    nodes: HashMap<K, E>,
}

impl Graph<(PunchKind, PunchKind), Weight> {
    pub fn insert(&mut self, first: &PunchKind, next: PunchKind) -> Option<Weight> {
        // Default impl should guarantee the edge exists and is safe to unwrap.
        // There should be no way to construct this graph without all variants
        // existing.
        let next_weight = self.nodes.get_mut(&(*first, next)).unwrap();
        let prev_val = *next_weight;

        *next_weight += 1;

        // if no values were seen prior, return None or else return a Some with
        // the previous value.
        if prev_val == Weight::from(0) {
            None
        } else {
            Some(prev_val)
        }
    }
}

impl<K, E: Edge> Graph<K, E>
where
    Self: Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Graph<(PunchKind, PunchKind), Weight> {
    fn default() -> Self {
        let punches = [
            PunchKind::Jab,
            PunchKind::Cross,
            PunchKind::LeadHook,
            PunchKind::RearHook,
            PunchKind::LeadUppercut,
            PunchKind::RearUppercut,
        ];

        let nodes = punches
            .into_iter()
            .flat_map(|first| punches.into_iter().map(move |next| (first, next)))
            .fold(HashMap::new(), |mut acc, x| {
                acc.insert(x, Weight::new(0));
                acc
            });

        Self { nodes }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_weighted_pairs() {
        let mut graph: Graph<_, Weight> = Graph::new();
        for punch_kind in [PunchKind::Cross].into_iter().cycle().take(5) {
            graph.insert(&PunchKind::Jab, punch_kind);
        }

        let jab_cross = graph.nodes.get(&(PunchKind::Jab, PunchKind::Cross));
        assert_eq!(Some(&Weight::from(5)), jab_cross);
    }
}
