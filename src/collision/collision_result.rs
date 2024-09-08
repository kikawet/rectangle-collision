use raylib::math::Vector2;

#[derive(Debug, Default)]
pub struct CollisionResult(pub [Option<Vector2>; 4]);

impl CollisionResult {
    const TOP: usize = 0;
    const RIGHT: usize = 1;
    const BOTTOM: usize = 2;
    const LEFT: usize = 3;

    pub fn new(
        top: Option<Vector2>,
        right: Option<Vector2>,
        bottom: Option<Vector2>,
        left: Option<Vector2>,
    ) -> Self {
        let mut result = [None; 4];

        result[Self::TOP] = top;
        result[Self::RIGHT] = right;
        result[Self::BOTTOM] = bottom;
        result[Self::LEFT] = left;

        Self(result)
    }

    pub fn into_option(self) -> Option<Self> {
        match self.0 {
            [None, None, None, None] => None,
            _ => Some(self),
        }
    }

    fn combine(self, other: CollisionResult) -> Self {
        let combined = [
            self.0[0].or(other.0[0]),
            self.0[1].or(other.0[1]),
            self.0[2].or(other.0[2]),
            self.0[3].or(other.0[3]),
        ];

        Self(combined)
    }
}

impl FromIterator<CollisionResult> for CollisionResult {
    fn from_iter<T: IntoIterator<Item = CollisionResult>>(iter: T) -> Self {
        iter.into_iter()
            .reduce(CollisionResult::combine)
            .unwrap_or_default()
    }
}
