use bevy::prelude::*;

//  borrowed from rapier
/// A bit mask identifying groups for interaction.
#[derive(Component, Reflect, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[reflect(Component, Hash, PartialEq)]
// #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Group(u32);

bitflags::bitflags! {
    impl Group: u32 {
        /// The group n°1.
        const PLAYER = 1 << 0;
        /// The group n°2.
        const ALLY = 1 << 1;
        /// The group n°3.
        const NPC = 1 << 2;
        /// The group n°4.
        const ENEMY = 1 << 3;
        /// The group n°5.
        const WEAPON = 1 << 4;
        /// The group n°6.
        const PROJECTILE = 1 << 5;
        /// The group n°7.
        const STRUCTURE = 1 << 6;
        /// The group n°8.
        const GROUP_8 = 1 << 7;
        /// The group n°9.
        const GROUP_9 = 1 << 8;
        /// The group n°10.
        const GROUP_10 = 1 << 9;
        /// The group n°11.
        const GROUP_11 = 1 << 10;
        /// The group n°12.
        const GROUP_12 = 1 << 11;
        /// The group n°13.
        const GROUP_13 = 1 << 12;
        /// The group n°14.
        const GROUP_14 = 1 << 13;
        /// The group n°15.
        const GROUP_15 = 1 << 14;
        /// The group n°16.
        const GROUP_16 = 1 << 15;
        /// The group n°17.
        const GROUP_17 = 1 << 16;
        /// The group n°18.
        const GROUP_18 = 1 << 17;
        /// The group n°19.
        const GROUP_19 = 1 << 18;
        /// The group n°20.
        const GROUP_20 = 1 << 19;
        /// The group n°21.
        const GROUP_21 = 1 << 20;
        /// The group n°22.
        const GROUP_22 = 1 << 21;
        /// The group n°23.
        const GROUP_23 = 1 << 22;
        /// The group n°24.
        const GROUP_24 = 1 << 23;
        /// The group n°25.
        const GROUP_25 = 1 << 24;
        /// The group n°26.
        const GROUP_26 = 1 << 25;
        /// The group n°27.
        const GROUP_27 = 1 << 26;
        /// The group n°28.
        const GROUP_28 = 1 << 27;
        /// The group n°29.
        const GROUP_29 = 1 << 28;
        /// The group n°30.
        const GROUP_30 = 1 << 29;
        /// The group n°31.
        const GROUP_31 = 1 << 30;
        /// The group n°32.
        const GROUP_32 = 1 << 31;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::ALL
    }
}
