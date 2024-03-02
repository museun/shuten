#[derive(Copy, Clone, PartialEq)]
pub struct Interest(u16);

impl std::fmt::Binary for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

impl std::fmt::Debug for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FIELDS: [&str; 11] = [
            "MOUSE_ENTER",
            "MOUSE_LEAVE",
            "MOUSE_MOVE",
            "MOUSE_CLICK",
            "MOUSE_DRAG",
            "MOUSE_SCROLL",
            "MOUSE_INSIDE",
            "MOUSE_OUTSIDE",
            "KEY_INPUT",
            "FOCUS_GAINED",
            "FOCUS_LOST",
        ];

        let mut seen = false;
        for (flag, repr) in (0..).zip(FIELDS) {
            if (self.0 >> flag) & 1 == 1 {
                if seen {
                    f.write_str(" | ")?;
                }
                f.write_str(repr)?;
                seen |= true
            }
        }

        if !seen {
            f.write_str("NONE")?
        }

        Ok(())
    }
}

impl std::ops::BitOr for Interest {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Interest {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::Not for Interest {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Interest {
    pub const NONE: Self = Self(0);

    pub const MOUSE_ENTER: Self = Self(1 << 0);
    pub const MOUSE_LEAVE: Self = Self(1 << 1);
    pub const MOUSE_MOVE: Self = Self(1 << 2);
    pub const MOUSE_CLICK: Self = Self(1 << 3);
    pub const MOUSE_DRAG: Self = Self(1 << 4);
    pub const MOUSE_SCROLL: Self = Self(1 << 5);

    pub const MOUSE_INSIDE: Self = Self(1 << 6);
    pub const MOUSE_OUTSIDE: Self = Self(1 << 7);

    pub const KEY_INPUT: Self = Self(1 << 8);

    pub const FOCUS_GAINED: Self = Self(1 << 9);
    pub const FOCUS_LOST: Self = Self(1 << 10);

    pub const MOUSE: Self = Self(
        Self::MOUSE_INSIDE.0
            | Self::MOUSE_OUTSIDE.0
            | Self::MOUSE_ENTER.0
            | Self::MOUSE_LEAVE.0
            | Self::MOUSE_MOVE.0
            | Self::MOUSE_CLICK.0
            | Self::MOUSE_DRAG.0
            | Self::MOUSE_SCROLL.0,
    );

    pub const FOCUS: Self = Self(Self::FOCUS_GAINED.0 | Self::FOCUS_LOST.0);

    pub const fn is_mouse_any(&self) -> bool {
        self.is_mouse_enter()
            || self.is_mouse_inside()
            || self.is_mouse_outside()
            || self.is_mouse_leave()
            || self.is_mouse_move()
            || self.is_mouse_click()
            || self.is_mouse_drag()
    }

    pub const fn is_focus_any(&self) -> bool {
        self.is_focus_gained() || self.is_focus_lost()
    }

    pub const fn is_none(&self) -> bool {
        self.0 == 0
    }

    pub const fn is_mouse_enter(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    pub const fn is_mouse_leave(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub const fn is_mouse_move(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub const fn is_mouse_click(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub const fn is_mouse_drag(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    pub const fn is_mouse_scroll(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    pub const fn is_mouse_outside(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    pub const fn is_mouse_inside(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    pub const fn is_key_input(&self) -> bool {
        self.0 & (1 << 8) != 0
    }

    pub const fn is_focus_gained(&self) -> bool {
        self.0 & (1 << 9) != 0
    }

    pub const fn is_focus_lost(&self) -> bool {
        self.0 & (1 << 10) != 0
    }
}
