/// Emojis embedded in the device
#[derive(Debug, Clone, Copy)]
pub enum Emojis {
    /// A smiley face
    Smiley = 0x00,
    /// A laughing face
    Laugh = 0x01,
    /// A sad face
    Sad = 0x02,
    /// A mad face
    Mad = 0x03,
    /// An angry face
    Angry = 0x04,
    /// A crying face
    Cry = 0x05,
    /// A greedy face
    Greedy = 0x06,
    /// A cool face
    Cool = 0x07,
    /// A shy face
    Shy = 0x08,
    /// An awkward face
    Awake = 0x09,
    /// A heart
    Heart = 0x0A,
    /// A small heart
    SmallHeart = 0x0B,
    /// A broken heart
    BrokenHeart = 0x0C,
    /// A water drop
    WaterDrop = 0x0D,
    /// A flame
    Flame = 0x0E,
    /// A creeper
    Creeper = 0x0F,
    /// A mad creeper
    MadCreeper = 0x10,
    /// A sword
    Sword = 0x11,
    /// A wooden sword
    WoodenSword = 0x12,
    /// A crystal sword
    CrystalSword = 0x13,
    /// A house
    House = 0x14,
    /// A tree
    Tree = 0x15,
    /// A flower
    Flower = 0x16,
    /// An umbrella
    Umbrella = 0x17,
    /// Rain
    Rain = 0x18,
    /// A monster
    Monster = 0x19,
    /// A crab
    Crab = 0x1A,
    /// A duck
    Duck = 0x1B,
    /// A rabbit
    Rabbit = 0x1C,
    /// A cat
    Cat = 0x1D,
    /// Arrow up
    ArrowUp = 0x1E,
    /// Arrow down
    ArrowDown = 0x1F,
    /// Arrow left
    ArrowLeft = 0x20,
    /// Arrow right
    ArrowRight = 0x21,
    /// A smile
    Smile = 0x22,
}

impl Into<u8> for Emojis {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Emojis {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Emojis::Smiley,
            0x01 => Emojis::Laugh,
            0x02 => Emojis::Sad,
            0x03 => Emojis::Mad,
            0x04 => Emojis::Angry,
            0x05 => Emojis::Cry,
            0x06 => Emojis::Greedy,
            0x07 => Emojis::Cool,
            0x08 => Emojis::Shy,
            0x09 => Emojis::Awake,
            0x0A => Emojis::Heart,
            0x0B => Emojis::SmallHeart,
            0x0C => Emojis::BrokenHeart,
            0x0D => Emojis::WaterDrop,
            0x0E => Emojis::Flame,
            0x0F => Emojis::Creeper,
            0x10 => Emojis::MadCreeper,
            0x11 => Emojis::Sword,
            0x12 => Emojis::WoodenSword,
            0x13 => Emojis::CrystalSword,
            0x14 => Emojis::House,
            0x15 => Emojis::Tree,
            0x16 => Emojis::Flower,
            0x17 => Emojis::Umbrella,
            0x18 => Emojis::Rain,
            0x19 => Emojis::Monster,
            0x1A => Emojis::Crab,
            0x1B => Emojis::Duck,
            0x1C => Emojis::Rabbit,
            0x1D => Emojis::Cat,
            0x1E => Emojis::ArrowUp,
            0x1F => Emojis::ArrowDown,
            0x20 => Emojis::ArrowLeft,
            0x21 => Emojis::ArrowRight,
            0x22 => Emojis::Smile,
            _ => Emojis::Smiley,
        }
    }
}

/// An implementation of iterator over the emojis
impl Iterator for Emojis {
    type Item = Emojis;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Emojis::from((*self as u8 + 1) % 0x23))
    }
}
