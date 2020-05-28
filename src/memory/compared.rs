use qdb_ast::ast::types::DataType;
use std::cmp::Ordering;

/// Compared - trait for specific comparison inside used type
/// Example for write predicate : a == b
/// YouComparedType::comparing(a : YouComparedType, b : YouComparedType, predicate : YouComparedType::eq)
pub trait Compared {
    /// Needed function for implementation comparison logic
    /// Example:
    /// ```
    /// impl Compared for DataType {
    ///     fn compare_with_override(&self, other: Self) -> Option<Ordering> {
    ///         unimplemented!() // It's normal
    ///     }
    /// }
    /// impl Compared for &DataType {
    ///     fn compare_with_override(&self, other: Self) -> Option<Ordering> {
    ///         self.compare_with(&other)
    ///     }
    /// }
    /// ```
    fn compare_with_override(&self, other: Self) -> Option<Ordering>;

    fn comparing<F: Fn(&Ordering) -> bool, T: Compared>(left: T, right: T, predicate: F) -> bool {
        let maybe_order_result = left.compare_with_override(right);
        if maybe_order_result.is_some() {
            let order_result = maybe_order_result.unwrap();
            return predicate(&order_result);
        }
        false
    }
    // ==
    fn eq(ord: &Ordering) -> bool {
        use std::cmp::Ordering;
        return match ord {
            Ordering::Equal => true,
            _ => false,
        };
    }
    // !=
    fn neq(ord: &Ordering) -> bool {
        use std::cmp::Ordering;
        return match ord {
            Ordering::Equal => false,
            _ => true,
        };
    }
    // >=
    fn eq_or_gr(ord: &Ordering) -> bool {
        use std::cmp::Ordering;
        return match ord {
            Ordering::Equal => true,
            Ordering::Greater => true,
            _ => false,
        };
    }
    // >
    fn gr(ord: &Ordering) -> bool {
        use std::cmp::Ordering;
        return match ord {
            Ordering::Greater => true,
            _ => false,
        };
    }
    // <=
    fn eq_or_le(ord: &Ordering) -> bool {
        use std::cmp::Ordering;
        return match ord {
            Ordering::Equal => true,
            Ordering::Less => true,
            _ => false,
        };
    }
    // <
    fn le(ord: &Ordering) -> bool {
        use std::cmp::Ordering;
        return match ord {
            Ordering::Less => true,
            _ => false,
        };
    }
}
