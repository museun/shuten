use shuten_core::geom::{pos2, Offset, Pos2, Rect};

/// Events produced when ***mouse capture*** is enabled
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MouseEvent {
    /// The mouse was moved
    Move {
        /// The position of the mouse cursor
        pos: Pos2,
    },
    /// The mouse was clicked
    Clicked {
        /// The position of the mouse cursor
        pos: Pos2,
        /// The button that was clicked
        button: MouseButton,
    },
    /// The mouse is held down
    Held {
        /// The position of the mouse cursor
        pos: Pos2,
        /// The button this is being held down
        button: MouseButton,
    },
    /// The mouse started to drag
    DragStart {
        /// The position of the mouse cursor
        pos: Pos2,
        /// The button that is being held down
        button: MouseButton,
    },
    /// The mouse is dragging
    Drag {
        /// The position where the drag started
        origin: Pos2,
        /// The current position of the mouse
        pos: Pos2,
        /// The difference between the origin and the current position
        delta: Offset,
        /// The button that is being held down
        button: MouseButton,
    },
    /// The mouse stopped dragging
    DragReleased {
        /// The position where the drag started
        origin: Pos2,
        /// The current position of the mouse
        pos: Pos2,
        /// The difference between the origin and the current position
        delta: Offset,
        /// The button that is being held down
        button: MouseButton,
    },
    /// The mouse was scrolled by the middle mouse button
    Scroll {
        /// The direction it was scrolled
        ///
        /// ***NOTE*** only vertical scrolling is supported
        ///
        /// So this'll always have the `x` component set to zero
        ///
        /// - If `y` is positive, its scrolling down
        /// - Otherwise if `y` is negative, its scrolling up
        dir: Offset,
        /// The position where the scroll occured
        pos: Pos2,
    },
}

impl MouseEvent {
    /// Was the mouse over this [`Rect`]?
    pub const fn mouse_over(&self, rect: Rect) -> bool {
        use MouseEvent::*;
        match *self {
            Move { pos, .. }
            | Held { pos, .. }
            | Clicked { pos, .. }
            | DragStart { pos, .. }
            | Scroll { pos, .. } => rect.contains(pos),

            Drag { origin, pos, .. } | DragReleased { origin, pos, .. } => {
                rect.contains(origin) && rect.contains(pos)
            }
        }
    }

    /// The [position](`Pos2`) of the mouse for this event
    pub const fn pos(&self) -> Pos2 {
        use MouseEvent::*;
        match *self {
            Move { pos, .. }
            | Clicked { pos, .. }
            | Held { pos, .. }
            | DragStart { pos, .. }
            | Drag { pos, .. }
            | DragReleased { pos, .. }
            | Scroll { pos, .. } => pos,
        }
    }

    /// Was this [`MouseButton`] held in this [`Rect`]?
    pub fn held_at(&self, rect: Rect, button: MouseButton) -> bool {
        matches!(*self, Self::Held { pos, button: b } if rect.contains(pos) && button == b)
    }

    /// Was this [`MouseButton`] clicked in this [`Rect`]?
    pub fn clicked_at(&self, rect: Rect, button: MouseButton) -> bool {
        matches!(*self, Self::Clicked { pos, button: b } if rect.contains(pos) && button == b)
    }
}

/// Mouse buttons during a [`MouseEvent`]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MouseButton {
    #[default]
    /// The primary mouse button, generally the left-button
    Primary,
    /// The secondary mouse button, generally the right-button
    Secondary,
    /// The middle mouse button
    Middle,
}

impl MouseButton {
    pub const fn is_primary(&self) -> bool {
        matches!(self, Self::Primary)
    }

    pub const fn is_secondary(&self) -> bool {
        matches!(self, Self::Secondary)
    }

    pub const fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
}

impl From<crossterm::event::MouseButton> for MouseButton {
    fn from(value: crossterm::event::MouseButton) -> Self {
        match value {
            crossterm::event::MouseButton::Left => Self::Primary,
            crossterm::event::MouseButton::Right => Self::Secondary,
            crossterm::event::MouseButton::Middle => Self::Middle,
        }
    }
}

#[derive(Default)]
enum Kind {
    #[default]
    None,
    Held,
    DragStart(Pos2),
    Drag(Pos2, Pos2),
}

#[derive(Default)]
pub struct MouseState {
    pos: Pos2,
    previous: Kind,
    button: Option<MouseButton>,
}

impl MouseState {
    fn check(&self, pos: Pos2, button: MouseButton) -> bool {
        self.pos == pos && self.button == Some(button)
    }

    pub fn update(&mut self, ev: crossterm::event::MouseEvent) -> Option<MouseEvent> {
        use crossterm::event::MouseEventKind as M;

        let pos = pos2(ev.column, ev.row);
        let event = match ev.kind {
            M::Down(button) => {
                let button = button.into();
                self.previous = Kind::Held;
                self.pos = pos;
                self.button = Some(button);
                MouseEvent::Held { pos, button }
            }

            M::Up(button) => {
                let button = button.into();
                match std::mem::take(&mut self.previous) {
                    Kind::Held if self.check(pos, button) => {
                        self.button.take();
                        MouseEvent::Clicked { pos, button }
                    }
                    Kind::Drag(origin, ..) if Some(button) == self.button => {
                        self.button.take();
                        MouseEvent::DragReleased {
                            origin,
                            pos,
                            delta: pos.as_offset() - self.pos.as_offset(),
                            button,
                        }
                    }
                    _ => return None,
                }
            }

            M::Moved => MouseEvent::Move { pos },

            M::ScrollDown => MouseEvent::Scroll {
                dir: Offset::DOWN,
                pos,
            },

            M::ScrollUp => MouseEvent::Scroll {
                dir: Offset::UP,
                pos,
            },

            M::Drag(button) => {
                let button = button.into();

                match std::mem::take(&mut self.previous) {
                    Kind::None if self.pos == pos => {
                        self.previous = Kind::Held;
                        self.button = Some(button);
                        self.pos = pos;
                        MouseEvent::Held { pos, button }
                    }

                    Kind::Held if self.pos == pos => {
                        self.previous = Kind::Held;
                        self.button = Some(button);
                        self.pos = pos;
                        return None;
                    }

                    Kind::None | Kind::Held => {
                        self.previous = Kind::DragStart(pos);
                        self.button = Some(button);
                        self.pos = pos;
                        MouseEvent::DragStart { pos, button }
                    }

                    Kind::DragStart(origin) if self.check(origin, button) => {
                        self.previous = Kind::Drag(origin, origin);
                        self.button = Some(button);
                        self.pos = origin;
                        MouseEvent::Drag {
                            pos,
                            button,
                            delta: Offset { x: 0, y: 0 },
                            origin,
                        }
                    }

                    Kind::Drag(old, origin) if self.check(origin, button) => {
                        self.previous = Kind::Drag(pos, origin);
                        self.button = Some(button);
                        self.pos = origin;
                        MouseEvent::Drag {
                            pos,
                            button,
                            delta: pos.as_offset() - old.as_offset(),
                            origin,
                        }
                    }

                    _ => return None,
                }
            }

            _ => return None,
        };

        Some(event)
    }
}
