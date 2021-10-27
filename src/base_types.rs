use crate::{VarInt, NbtTag, BigString, ChatJson, Position, McUuid};

auto_enum! {
    Direction; VarInt {
        0 => Down,
        1 => Up,
        2 => North,
        3 => South,
        4 => West,
        5 => East,
    }

    ParticleData; VarInt {
        0 => AmbientEntityEffect,
        1 => AngryVillager,
        2 => Barrier,
        3 => Light,
        4 => Block: VarInt,
        5 => Bubble,
        6 => Cloud,
        7 => Crit,
        8 => DamageIndicator,
        9 => DragonBreath,
        10 => DrippingLava,
        11 => FallingLava,
        12 => LandingLava,
        13 => DrippingWater,
        14 => FallingWater,
        15 => Dust: DustParticleData,
        16 => DustColorTransition: DustColorTransitionParticleData,
        17 => Effect,
        18 => ElderGuardian,
        19 => EnchantedHit,
        20 => Enchant,
        21 => EndRod,
        22 => EntityEffect,
        23 => ExplosionEmitter,
        24 => Explosion,
        25 => FallingDust: FallingDustParticleData,
        26 => Firework,
        27 => Fishing,
        28 => Flame,
        29 => SoulFireFlame,
        30 => Soul,
        31 => Flash,
        32 => HappyVillager,
        33 => Composter,
        34 => Heart,
        35 => InstantEffect,
        36 => Item: ItemParticleData,
        37 => Vibration: VibrationParticleData,
        38 => ItemSlime,
        39 => ItemSnowball,
        40 => LargeSmoke,
        41 => Lava,
        42 => Mycelium,
        43 => Note,
        44 => Poof,
        45 => Portal,
        46 => Rain,
        47 => Smoke,
        48 => Sneeze,
        49 => Spit,
        50 => SquidInk,
        51 => SweepAttack,
        52 => TotemOfUndying,
        53 => Underwater,
        54 => Splash,
        55 => Witch,
        56 => BubblePop,
        57 => CurrentDown,
        58 => BubbleColumnUp,
        59 => Nautilus,
        60 => Dolphin,
        61 => CampfireCosySmoke,
        62 => CampfireSignalSmoke,
        63 => DrippingHoney,
        64 => FallingHoney,
        65 => LandingHoney,
        66 => FallingNectar,
        67 => FallingSporeBlossom,
        68 => Ash,
        69 => CrimsonSpore,
        70 => WarpedSpore,
        71 => SporeBlossomAir,
        72 => DrippingObsidianTear,
        73 => FallingObsidianTear,
        74 => LandingObsidianTear,
        75 => ReversePortal,
        76 => WhiteAsh,
        77 => SmallFlame,
        78 => Snowflake,
        79 => DrippingDripstoneLava,
        80 => FallingDripstoneLava,
        81 => DrippingDripstoneWater,
        82 => FallingDripstoneWater,
        83 => GlowSquidInk,
        84 => Glow,
        85 => WaxOn,
        86 => WaxOff,
        87 => ElectricSpark,
        88 => Scrape,
    }

    Pose; VarInt {
        0 => Standing,
        1 => FallFlying,
        2 => Sleeping,
        3 => Swimming,
        4 => SpinAttack,
        5 => Sneaking,
        6 => LongJumping,
        7 => Dying,
    }

    VillagerType; VarInt {
        0 => Desert,
        1 => Jungle,
        2 => Plains,
        3 => Savanna,
        4 => Snow,
        5 => Swamp,
        6 => Taiga,
    }

    VillagerProfession; VarInt {
        0 => None,
        1 => Armorer,
        2 => Butcher,
        3 => Cartographer,
        4 => Cleric,
        5 => Farmer,
        6 => Fisherman,
        7 => Fletcher,
        8 => LeatherWorker,
        9 => Librarian,
        10 => Mason,
        11 => Nitwit,
        12 => Shepherd,
        13 => ToolSmith,
        14 => WeaponSmith,
    }

    MetadataEntryType; VarInt {
        0 => EntryByte: u8,
        1 => EntryVarInt: VarInt,
        2 => EntryFloat: f32,
        3 => EntryString: BigString,
        4 => EntryChat: ChatJson,
        5 => EntryOptChat: (bool, Option<ChatJson>),
        6 => EntrySlot: SlotData,
        7 => EntryBoolean: bool,
        8 => EntryRotation: (f32, f32, f32),
        9 => EntryPosition: Position,
        10 => EntryOptPosition: (bool, Option<Position>),
        11 => EntryDirection: Direction,
        12 => EntryOptUuid: (bool, Option<McUuid>),
        13 => EntryOptBlockId: VarInt,
        14 => EntryNbt: NbtTag,
        15 => EntityParticle: Particle,
        16 => EntryVillagerData: (VillagerType, VillagerProfession, VarInt),
        17 => EntryOptVarInt: VarInt,
        18 => EntryPose: Pose,
    }
}

auto_struct! {
    DustParticleData {
        red: f32,
        green: f32,
        blue: f32,
        scale: f32,
    }

    DustColorTransitionParticleData {
        from_red: f32,
        from_green: f32,
        from_blue: f32,
        scale: f32,
        to_red: f32,
        to_green: f32,
        to_blue: f32,
    }

    FallingDustParticleData {
        block_state: VarInt,
    }

    ItemParticleData {
        item: SlotData,
    }

    VibrationParticleData {
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
        dest_x: f64,
        dest_y: f64,
        dest_z: f64,
        ticks: i32,
    }

    Particle {
        data: ParticleData,
    }

    SlotData {
        present: bool,
        item_id: Option<VarInt> | present => None,
        item_count: Option<u8> | present => None,
        nbt: Option<NbtTag> | present => None,
    }

    MetadataEntry {
        index: u8,
        entry_type: Option<MetadataEntryType> | index == 0xff => None,
    }
}