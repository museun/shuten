use super::{Align2, Dimension2};

/// The flow property of an item
///
/// This defines how an object participates in a layout
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Flow {
    /// The item participates in list, grid and table layouts
    ///
    /// This is the default for most widgets
    Inline,
    /// The item does not participate in layout.
    ///
    /// Its position is calculated using an anchor and an offset
    Relative {
        /// Where in the parent container this item should be anchored to
        anchor: Align2,
        /// The offset from the anchor to position this item
        offset: Dimension2,
    },
}

impl Flow {
    pub const fn is_relative(&self) -> bool {
        !self.is_inline()
    }

    pub const fn is_inline(&self) -> bool {
        matches!(self, Self::Inline)
    }
}

/// How flex items should be fitted
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum FlexFit {
    /// The container lets the child have any size that fits within the container
    #[default]
    Loose,
    /// The container forces its children to stretch to its size
    Tight,
}

/// Sizing along the main axis
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[non_exhaustive]
pub enum MainAxisSize {
    /// Make the container fill all available space along its main axis
    Max,
    /// Make the container fill the minimum amout of space along its main axis
    Min,
}

/// The alignment of the **main** axis
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MainAxisAlignment {
    /// Items should be aligned to the start of the container's main axis
    Start,
    /// Items should be aligned to the center of the container's main axis
    Center,
    /// Items should be aligned to the end of the container's main axis
    End,
    /// Spread items evenly where the gap is at the start and end of the
    /// container is half of the size of the gap between each adjacent item.
    SpaceAround,
    /// Spread items evenly with no gap at the start and end of the container
    ///
    /// If there is a single item, it will be at the start
    SpaceBetween,
    /// Spread items evenly where the gap is at the start and end of the
    /// container is the same size as the gap between each adjacent item.
    SpaceEvenly,
}

/// How items should be aligned in the main axis
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MainAxisAlignItems {
    /// Align items to the beginning of the container's main axis
    ///
    /// - left-to-right
    ///     - this is the top of the container
    /// - top-down
    ///     - this is the left side of the container
    Start,
    /// Align items to the end of the container's main axis
    ///
    /// - left-to-right
    ///     - this is the bottom of the container
    /// - top-down
    ///     - this is the right side of the container
    /// /// Align items to the center of the container's main axis
    Center,
    /// Align items to the end of the container's main axis
    ///
    /// - left-to-right
    ///     - this is the bottom of the container
    /// - top-down
    ///     - this is the right side of the container
    End,
    /// Stretch items to fill the maximum size of the container's main axis
    Stretch,
}

/// Alignment in the **other** axis
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum CrossAxisAlignment {
    /// Align items to the beginning of the container's cross axis
    ///
    /// - left-to-right
    ///     - this is the top of the container
    /// - top-down
    ///     - this is the left side of the container
    Start,
    /// Align items to the center of the container's cross axis
    Center,
    /// Align items to the end of the container's cross axis
    ///
    /// - left-to-right
    ///     - this is the bottom of the container
    /// - top-down
    ///     - this is the right side of the container
    End,
    /// Stretch items to fill the maximum size of the container's cross axis
    Stretch,
}

impl CrossAxisAlignment {
    /// What is the flex factor for the cross axis?
    pub const fn flex(&self) -> u16 {
        match self {
            Self::Start | Self::Center | Self::End => 0,
            Self::Stretch => 1,
        }
    }
}
