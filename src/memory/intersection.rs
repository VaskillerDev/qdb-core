pub trait Intersection {
    fn intersect(left: &Self, right: &Self) -> bool;
}
