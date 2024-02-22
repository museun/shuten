use super::{lerp, pos2, vec2, Offset, Pos2, ResizeDelta, Vec2};

/// A rectangle
///
/// (e.g. a 2d matrix)
///
/// This is composed of a `min` point (the _left-top_) and a `max` point (the _right-bottom_)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Rect {
    pub min: Pos2,
    pub max: Pos2,
}

/// Create a rect starting at 0,0 with the provided `size`
pub fn rect(size: Vec2) -> Rect {
    Rect::from_min_size(Pos2::ZERO, size)
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            min: Pos2::ZERO,
            max: Pos2::ZERO,
        }
    }
}

impl Rect {
    /// A zero sized [`Rect`]
    pub const ZERO: Self = Self {
        min: pos2(0, 0),
        max: pos2(0, 0),
    };

    /// Create a new [`Rect`] from a `min` point and a `max` point
    ///
    /// The `min` point is the _left-top_ of the rectangle
    ///
    /// The `max` point is the _right-top_ of the rectangle;
    pub const fn from_min_max(min: Pos2, max: Pos2) -> Self {
        Self { min, max }
    }

    /// Create a new [`Rect`] from a `min` point and a size
    ///
    /// The `min` point is the _left-top_ of the rectangle
    pub fn from_min_size(min: Pos2, size: Vec2) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    /// Create a new [`Rect`] from a center point and a size
    ///
    /// This'll center the rectangle at `center` and use `size` as the _diameter_
    pub fn from_center_size(center: Pos2, size: Vec2) -> Self {
        Self {
            min: center - (size / 2),
            max: center + (size / 2),
        }
    }

    /// Get the area of the rectangle
    pub const fn area(&self) -> u16 {
        self.width() * self.height()
    }

    /// Get the `x` coordinate of the rectangle
    ///
    /// This is the `left` side
    pub const fn x(&self) -> u16 {
        self.min.x
    }

    /// Get the `y` coordinate of the rectangle
    ///
    /// This is the `top` side
    pub const fn y(&self) -> u16 {
        self.min.y
    }

    /// Set the `left-top` position of the rectangle
    pub fn set_pos(&mut self, pos: Pos2) {
        *self = self.with_pos(pos)
    }

    /// Set the `size` of the rectangle
    pub fn set_size(&mut self, size: Vec2) {
        *self = self.with_size(size)
    }

    /// Make a new rectangle with this `left-top` position
    pub fn with_pos(self, pos: Pos2) -> Self {
        Self::from_min_size(pos, self.size())
    }

    /// Make a new rectangle with this `size`
    pub fn with_size(self, size: Vec2) -> Self {
        Self::from_min_size(self.min, size)
    }

    /// Distance from this rectangle to that point
    pub fn distance_to_point(&self, pos: Pos2) -> f32 {
        self.distance_sq_to_point(pos).sqrt()
    }

    /// Distance from this rectangle to that point, squared
    pub fn distance_sq_to_point(&self, pos: Pos2) -> f32 {
        fn distance(min: f32, max: f32, t: f32) -> f32 {
            match () {
                _ if min > t => min - t,
                _ if t > max => t - max,
                _ => 0.0,
            }
        }

        let dx = distance(self.min.x as f32, self.max.x as f32, pos.x as f32);
        let dy = distance(self.min.y as f32, self.max.y as f32, pos.y as f32);
        dx.mul_add(dx, dy * dy)
    }

    /// Get the size of the rectangle
    ///
    /// e.g `{width, height}`
    pub fn size(&self) -> Vec2 {
        (self.max - self.min).to_vec2()
    }

    /// The width of the rectangle
    pub const fn width(&self) -> u16 {
        self.max.x.saturating_sub(self.min.x)
    }

    /// The height of the rectangle
    pub const fn height(&self) -> u16 {
        self.max.y.saturating_sub(self.min.y)
    }

    /// Clamp that point inside of this rectangle
    pub fn clamp(&self, pos: Pos2) -> Pos2 {
        pos.clamp(self.min, self.max)
    }

    /// Clamp that rectangle inside of this rectangle
    pub fn clamp_rect(&self, rect: Self) -> Self {
        let min = rect.min.max(self.min).min(pos2(
            self.right().saturating_sub(rect.width()),
            self.bottom().saturating_sub(rect.height()),
        ));
        Self::from_min_size(min, rect.size())
    }

    /// Expand this rectangle on all sides by `d`
    pub fn expand(&self, d: u16) -> Self {
        self.expand2(Vec2::splat(d))
    }

    /// Shrink this rectangle on all sides by `d`
    pub fn shrink(&self, d: u16) -> Self {
        self.shrink2(Vec2::splat(d))
    }

    /// Expand this rectangle on the horizontal side by the `x` vector and the vertical side by the `y` vector
    pub fn expand2(&self, v: Vec2) -> Self {
        Self::from_min_max(self.min - v, self.max + v)
    }

    /// Shrink this rectangle on the horizontal side by the `x` vector and the vertical side by the `y` vector
    pub fn shrink2(&self, v: Vec2) -> Self {
        Self::from_min_max(self.min + v, self.max - v)
    }

    /// Does this rectangle contain that point?
    pub const fn contains(&self, pos: Pos2) -> bool {
        self.min.x <= pos.x && pos.x < self.max.x && self.min.y <= pos.y && pos.y < self.max.y
    }

    /// Does this rectangle contain that rectangle?
    pub const fn contains_rect(&self, other: Self) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    /// The minimum rect that contains both rects
    pub fn union(&self, other: Self) -> Self {
        Self::from_min_max(self.min.min(other.min), self.max.max(other.max))
    }

    /// The area that covered by both rects
    pub fn intersect(&self, other: Self) -> Self {
        Self::from_min_max(self.min.max(other.min), self.max.min(other.max))
    }

    /// Does this rectangle intersect with the other rectangle?
    pub const fn intersects(&self, other: Self) -> bool {
        self.min.x <= other.max.x
            && other.min.x <= self.max.x
            && self.min.y <= other.max.y
            && other.min.y <= self.max.y
    }

    /// The left edge of the rectangle
    pub const fn left(&self) -> u16 {
        self.min.x
    }

    /// The right edge of the rectangle
    pub const fn right(&self) -> u16 {
        self.max.x.saturating_sub(1)
    }

    /// The top edge of the rectangle
    pub const fn top(&self) -> u16 {
        self.min.y
    }

    /// The bottom edge of the rectangle
    pub const fn bottom(&self) -> u16 {
        self.max.y.saturating_sub(1)
    }

    /// The center point of the rectangle
    pub const fn center(&self) -> Pos2 {
        pos2((self.min.x + self.max.x) / 2, (self.min.y + self.max.y) / 2)
    }

    /// The left-top corner
    pub const fn left_top(&self) -> Pos2 {
        pos2(self.left(), self.top())
    }

    /// The right-top corner
    pub const fn right_top(&self) -> Pos2 {
        pos2(self.right(), self.top())
    }

    /// The left-bottom corner
    pub const fn left_bottom(&self) -> Pos2 {
        pos2(self.left(), self.bottom())
    }

    /// The right-bottom corner
    pub const fn right_bottom(&self) -> Pos2 {
        pos2(self.right(), self.bottom())
    }

    /// The left-center position
    pub const fn left_center(&self) -> Pos2 {
        pos2(self.left(), self.center().y)
    }

    /// The right-center position
    pub const fn right_center(&self) -> Pos2 {
        pos2(self.right(), self.center().y)
    }

    /// The center-top position
    pub const fn center_top(&self) -> Pos2 {
        pos2(self.center().x, self.top())
    }

    /// The center-bottom position
    pub const fn center_bottom(&self) -> Pos2 {
        pos2(self.center().x, self.bottom())
    }

    /// Split this rectangle into 2 rectangles, horizontally using an absolute position
    pub fn split_horizontal(&self, from_left: u16) -> (Self, Self) {
        let left = Self::from_min_size(self.min, vec2(from_left, self.height()));
        let right = Self::from_min_size(
            left.right_top(),
            vec2(self.width() - from_left, self.height()),
        );
        (left, right)
    }

    /// Split this rectangle into 2 rectangles, horizontally using a ratio (in the range of `0.0 ..= 1.0`)
    pub fn split_horizontal_ratio(&self, ratio: f32) -> (Self, Self) {
        let at = lerp(self.min.x as f32..=self.max.x as f32, ratio);
        let left = Self::from_min_max(self.min, pos2(at as u16, self.max.y));
        let right = Self::from_min_max(pos2(at as u16, self.min.y), self.max);
        (left, right)
    }

    /// Split this rectangle into 2 rectangles, vertically using an absolute position
    pub fn split_vertical(&self, from_bottom: u16) -> (Self, Self) {
        let top = Self::from_min_max(self.min, self.max - pos2(0, from_bottom));
        let bottom = Self::from_min_max(pos2(self.min.x, self.max.y - from_bottom), self.max);
        (top, bottom)
    }

    /// Split this rectangle into 2 rectangles, vertically using a ratio (in the range of `0.0 ..= 1.0`)
    pub fn split_vertical_ratio(&self, ratio: f32) -> (Self, Self) {
        let at = lerp(self.min.y as f32..=self.max.y as f32, ratio);
        let top = Self::from_min_max(self.min, pos2(self.max.x, at as u16));
        let bottom = Self::from_min_max(pos2(self.min.x, at as u16), self.max);
        (top, bottom)
    }

    /// Move this rect by `offset`
    pub fn translate(&self, d: Offset) -> Self {
        let pos = pos2(
            Self::add(self.x(), self.width(), d.x),
            Self::add(self.y(), self.height(), d.y),
        );
        Self::from_min_size(pos, self.size())
    }

    fn add(d: u16, a: u16, t: i32) -> u16 {
        i32::from(d)
            .saturating_add(t)
            .clamp(0, (u16::MAX - a) as i32) as u16
    }
}

impl Rect {
    /// Get the `nth` row of this rectangle
    pub fn row(&self, nth: u16) -> Option<Self> {
        if nth >= self.height() {
            return None;
        }

        Some(Self::from_min_size(
            self.left_top() + pos2(0, nth),
            vec2(self.width(), 1),
        ))
    }

    /// Get an iterator for all of the row-starts
    pub fn rows(&self) -> impl Iterator<Item = Pos2> {
        let Pos2 { x, mut y } = self.left_top();
        let bottom = self.bottom();

        std::iter::from_fn(move || {
            if y - 1 > bottom {
                return None;
            }

            let s = Some(pos2(x, y));
            y += 1;
            s
        })
    }

    /// Get an iterator for all of the column-starts
    pub fn columns(&self) -> impl Iterator<Item = Pos2> {
        let Pos2 { mut x, y } = self.left_top();
        let right = self.right();

        std::iter::from_fn(move || {
            if x - 1 > right {
                return None;
            }

            let s = Some(pos2(x, y));
            x += 1;
            s
        })
    }

    /// Get an iterator over each `position` in this rectangle
    pub fn indices(&self) -> impl Iterator<Item = Pos2> {
        let start = self.left_top();
        let end = self.right_bottom();
        let (sx, sy) = (start.x, start.y);
        let (ex, ey) = (end.x.max(sx + 1), end.y.max(sy + 1));
        (sy..=ey).flat_map(move |y| (sx..=ex).map(move |x| pos2(x, y)))
    }
}

impl std::ops::Add<ResizeDelta> for Rect {
    type Output = Self;
    fn add(mut self, rhs: ResizeDelta) -> Self::Output {
        match rhs {
            ResizeDelta::Left(d) => self.min.x = self.min.x.saturating_sub(d),
            ResizeDelta::Up(d) => self.min.y = self.min.y.saturating_sub(d),
            ResizeDelta::Right(d) => self.max.x += d,
            ResizeDelta::Down(d) => self.max.y += d,
        }
        self
    }
}

impl std::ops::AddAssign<ResizeDelta> for Rect {
    fn add_assign(&mut self, rhs: ResizeDelta) {
        *self = *self + rhs
    }
}

impl std::ops::Sub<ResizeDelta> for Rect {
    type Output = Self;
    fn sub(mut self, rhs: ResizeDelta) -> Self::Output {
        match rhs {
            ResizeDelta::Left(d) if self.min.x + d < self.max.x => self.min.x += d,
            ResizeDelta::Up(d) if self.min.y + d < self.max.y => self.min.y += d,

            ResizeDelta::Right(d) if self.min.x + d < self.max.x => {
                self.max.x = self.max.x.saturating_sub(d)
            }
            ResizeDelta::Down(d) if self.min.y + d < self.max.y => {
                self.max.y = self.max.y.saturating_sub(d)
            }
            _ => {}
        }
        self
    }
}

impl std::ops::SubAssign<ResizeDelta> for Rect {
    fn sub_assign(&mut self, rhs: ResizeDelta) {
        *self = *self - rhs
    }
}

#[allow(dead_code)]
impl Rect {
    fn extend_with_x(&self, x: u16) -> Self {
        Self {
            min: self.min.min(pos2(x, self.min.y)),
            ..*self
        }
    }

    fn extend_with_y(&self, y: u16) -> Self {
        Self {
            min: self.min.min(pos2(self.min.x, y)),
            ..*self
        }
    }

    fn extend_with(&self, pos: Pos2) -> Self {
        Self {
            min: self.min.min(pos),
            max: self.max.max(pos),
        }
    }

    fn shrink_with_x(&self, x: u16) -> Self {
        Self {
            min: self.min.max(pos2(x, self.min.y)),
            ..*self
        }
    }

    fn shrink_with_y(&self, y: u16) -> Self {
        Self {
            min: self.min.max(pos2(self.min.x, y)),
            ..*self
        }
    }

    fn shrink_with(&self, pos: Pos2) -> Self {
        Self {
            min: self.min.max(pos),
            max: self.max.min(pos),
        }
    }

    fn translate_with(&self, delta: Offset) -> Self {
        Self {
            min: pos2(
                Self::add(self.x(), self.width(), delta.x),
                Self::add(self.y(), self.height(), delta.y),
            ),
            max: pos2(
                Self::add(self.right(), self.left(), delta.x),
                Self::add(self.bottom(), self.top(), delta.y),
            ),
        }
    }
}
