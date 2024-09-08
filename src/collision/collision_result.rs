use raylib::math::Vector2;

#[derive(Debug, Default, Clone)]
pub struct CollisionResult(pub [Option<Vector2>; 4]);

impl CollisionResult {
    pub(crate) const TOP: usize = 0;
    pub(crate) const RIGHT: usize = 1;
    pub(crate) const BOTTOM: usize = 2;
    pub(crate) const LEFT: usize = 3;

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

    #[allow(clippy::needless_pass_by_value)]
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

impl FromIterator<Option<Vector2>> for CollisionResult {
    fn from_iter<T: IntoIterator<Item = Option<Vector2>>>(iter: T) -> Self {
        iter.into_iter()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| match *chunk {
                [] => CollisionResult::default(),
                [top] => CollisionResult::new(top, None, None, None),
                [top, right] => CollisionResult::new(top, right, None, None),
                [top, right, bottom] => CollisionResult::new(top, right, bottom, None),
                [top, right, bottom, left] => CollisionResult::new(top, right, bottom, left),
                [..] => unreachable!(),
            })
            .collect::<CollisionResult>()
    }
}
