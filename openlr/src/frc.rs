use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum FRC {
    FRC0 = 0,
    FRC1 = 1,
    FRC2 = 2,
    FRC3 = 3,
    FRC4 = 4,
    FRC5 = 5,
    FRC6 = 6,
    FRC7 = 7,
}

impl Copy for FRC {}

impl FRC {
    #[allow(dead_code)]
    pub fn to_usize(&self) -> usize {
        match self {
            FRC::FRC0 => 0,
            FRC::FRC1 => 1,
            FRC::FRC2 => 2,
            FRC::FRC3 => 3,
            FRC::FRC4 => 4,
            FRC::FRC5 => 5,
            FRC::FRC6 => 6,
            FRC::FRC7 => 7,
        }
    }

    pub(crate) fn from_u8(i: u8) -> Self {
        match i {
            0 => FRC::FRC0,
            1 => FRC::FRC1,
            2 => FRC::FRC2,
            3 => FRC::FRC3,
            4 => FRC::FRC4,
            5 => FRC::FRC5,
            6 => FRC::FRC6,
            7 => FRC::FRC7,
            _ => unreachable!(),
        }
    }
}
