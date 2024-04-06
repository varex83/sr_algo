pub trait Union {
    fn union(&self, other: &Self) -> Self;

    fn intersection(&self, other: &Self) -> Self;

    fn complement(&self, other: &Self) -> Self;

    fn difference(&self, other: &Self) -> Self;

    fn symmetric_difference(&self, other: &Self) -> Self;
}

impl<T> Union for Vec<T>
where
    T: Eq + Clone,
{
    fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for item in other {
            if !result.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for item in self {
            if other.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }

    fn complement(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for item in self {
            if !other.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }

    fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for item in self {
            if !other.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }

    fn symmetric_difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for item in self {
            if !other.contains(item) {
                result.push(item.clone());
            }
        }
        for item in other {
            if !self.contains(item) {
                result.push(item.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(a.union(&b), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_intersection() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(a.intersection(&b), vec![3]);
    }

    #[test]
    fn test_complement() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(a.complement(&b), vec![1, 2]);
    }

    #[test]
    fn test_difference() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(a.difference(&b), vec![1, 2]);
    }

    #[test]
    fn test_symmetric_difference() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(a.symmetric_difference(&b), vec![1, 2, 4, 5]);
    }
}
