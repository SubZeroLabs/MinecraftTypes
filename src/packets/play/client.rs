use crate::{
    primitive::{McBoolean, McLong, McByte, McDouble, McFloat, McInteger, McShort, McUnsignedByte},
    Angle, VarLong, ChatJson, Decoder, Identifier, McUuid, NbtTag, Position, SlotData, VarInt,
};
minecraft_struct! { SpawnEntity
    entity_id = VarInt;encode;;decode;
    uuid = McUuid;encode;;decode;
    entity_type = VarInt;encode;;decode;
    x = McDouble;encode;;decode;
    y = McDouble;encode;;decode;
    z = McDouble;encode;;decode;
    pitch = Angle;encode;;decode;
    yaw = Angle;encode;;decode;
    data = McInteger;encode;;decode;
    velocity_x = McShort;encode;;decode;
    velocity_y = McShort;encode;;decode;
    velocity_z = McShort;encode;;decode;
}

minecraft_struct! { SpawnExperienceOrb
    entity_id = VarInt;encode;;decode;
    x = McDouble;encode;;decode;
    y = McDouble;encode;;decode;
    z = McDouble;encode;;decode;
    count = McShort;encode;;decode;
}

minecraft_struct! { SpawnLivingEntity
    entity_id = VarInt;encode;;decode;
    uuid = McUuid;encode;;decode;
    entity_type = VarInt;encode;;decode;
    x = McDouble;encode;;decode;
    y = McDouble;encode;;decode;
    z = McDouble;encode;;decode;
    yaw = Angle;encode;;decode;
    pitch = Angle;encode;;decode;
    head_pitch = Angle;encode;;decode;
    velocity_x = McShort;encode;;decode;
    velocity_y = McShort;encode;;decode;
    velocity_z = McShort;encode;;decode;
}

minecraft_struct! { SpawnPainting
    entity_id = VarInt;encode;;decode;
    uuid = McUuid;encode;;decode;
    motive = VarInt;encode;;decode;
    location = Position;encode;;decode;
    direction = McByte;encode;;decode;
}

minecraft_struct! { SpawnPlayer
    entity_id = VarInt;encode;;decode;
    uuid = McUuid;encode;;decode;
    x = McDouble;encode;;decode;
    y = McDouble;encode;;decode;
    z = McDouble;encode;;decode;
    yaw = Angle;encode;;decode;
    pitch = Angle;encode;;decode;
}

minecraft_struct! { SculkVibrationSignalBlock
    source_position = Position;encode;;decode;
    destination_identifier = Identifier;encode;;decode;
    destination = Position;encode;;decode;
    arrival_ticks = VarInt;encode;;decode;
}

minecraft_struct! { SculkVibrationSignalEntity
    source_position = Position;encode;;decode;
    destination_identifier = Identifier;encode;;decode;
    destination = VarInt;encode;;decode;
    arrival_ticks = VarInt;encode;;decode;
}

minecraft_struct! { EntityAnimation
    entity_id = VarInt;encode;;decode;
    animation = McUnsignedByte;encode;;decode;
}

minecraft_struct! { Statistics
    count = VarInt;encode;;decode;
    statistic = Vec<VarInt>;encode_arr;Decoder;decode_arr VarInt::from(*count);
}

minecraft_struct! { AcknowledgePlayerDigging
    location = Position;encode;;decode;
    block = VarInt;encode;;decode;
    status = VarInt;encode;;decode;
    successful = McBoolean;encode;;decode;
}

minecraft_struct! { BlockBreakAnimation
    entity_id = VarInt;encode;;decode;
    location = VarInt;encode;;decode;
    destroy_stage = McByte;encode;;decode;
}

minecraft_struct! { BlockEntityData
    location = Position;encode;;decode;
    action = McUnsignedByte;encode;;decode;
    nbt_data = NbtTag;encode;;decode;
}

minecraft_struct! { BlockAction
    location = Position;encode;;decode;
    action_id = McUnsignedByte;encode;;decode;
    action_param = McUnsignedByte;encode;;decode;
    block_type = VarInt;encode;;decode;
}

minecraft_struct! { BlockChange
    location = Position;encode;;decode;
    block_id = VarInt;encode;;decode;
}

// todo Boss Bar (https://wiki.vg/Protocol#Boss_Bar)

minecraft_struct! { ServerDifficulty
    difficulty = McUnsignedByte;encode;;decode;
    difficulty_locked = McBoolean;encode;;decode;
}

minecraft_struct! { ChatMessage
    chat_json = ChatJson;encode;;decode;
    position = McByte;encode;;decode;
    sender = McUuid;encode;;decode;
}

minecraft_struct! { ClearTitles
    reset = McBoolean;encode;;decode;
}

// todo Tab-Complete (https://wiki.vg/Protocol#Tab-Complete_.28clientbound.29)

// todo Declare Commands (https://wiki.vg/Protocol#Declare_Commands)

minecraft_struct! { CloseWindow
    window_id = McUnsignedByte;encode;;decode;
}

minecraft_struct! { WindowItems
    window_id = McUnsignedByte;encode;;decode;
    state_id = VarInt;encode;;decode;
    count = VarInt;encode;;decode;
    slot_data = Vec<SlotData>;encode_arr;Decoder;decode_arr VarInt::from(*count);
}

minecraft_struct! { WindowProperty
    window_id = McUnsignedByte;encode;;decode;
    property = McShort;encode;;decode;
    value = McShort;encode;;decode;
}

minecraft_struct! { SetSlot
    window_id = McUnsignedByte;encode;;decode;
    state_id = VarInt;encode;;decode;
    slot = McShort;encode;;decode;
    slot_data = SlotData;encode;;decode;
}

minecraft_struct! { SetCooldown
    item_id = VarInt;encode;;decode;
    cooldown_ticks = VarInt;encode;;decode;
}

minecraft_struct! { PluginMessage
    channel = Identifier;encode;;decode;
    data = Vec<McUnsignedByte>;encode_arr;Decoder;decode_to_end;
}

minecraft_struct! { NamedSoundEffect
    sound_name = Identifier;encode;;decode;
    sound_category = VarInt;encode;;decode;
    effect_position_x = McInteger;encode;;decode;
    effect_position_y = McInteger;encode;;decode;
    effect_position_z = McInteger;encode;;decode;
    volume = McFloat;encode;;decode;
    pitch = McFloat;encode;;decode;
}

minecraft_struct! { Disconnect
    reason = ChatJson;encode;;decode;
}

minecraft_struct! { EntityStatus
    entity_id = McInteger;encode;;decode;
    entity_status = McByte;encode;;decode;
}

minecraft_struct! { Explosion
    x = McFloat;encode;;decode;
    y = McFloat;encode;;decode;
    z = McFloat;encode;;decode;
    strength = McFloat;encode;;decode;
    record_count = VarInt;encode;;decode;
    records = Vec<(McUnsignedByte, McUnsignedByte, McUnsignedByte)>;encode_arr;Decoder;decode_arr VarInt::from(*record_count);
    player_motion_x = McFloat;encode;;decode;
    player_motion_y = McFloat;encode;;decode;
    player_motion_z = McFloat;encode;;decode;
}

minecraft_struct! { UnloadChunk
    chunk_x = McInteger;encode;;decode;
    chunk_z = McInteger;encode;;decode;
}

minecraft_struct! { ChangeGameState
    reason = McUnsignedByte;encode;;decode;
    value = McFloat;encode;;decode;
}

minecraft_struct! { OpenHorseWindow
    window_id = McByte;encode;;decode;
    number_of_slots = VarInt;encode;;decode;
    entity_id = McInteger;encode;;decode;
}

minecraft_struct! { InitializeWorldBorder
    x = McDouble;encode;;decode;
    z = McDouble;encode;;decode;
    old_diameter = McDouble;encode;;decode;
    new_diameter = McDouble;encode;;decode;
    speed = VarLong;encode;;decode;
    portal_teleport_boundary = VarInt;encode;;decode;
    warning_blocks = VarInt;encode;;decode;
    warning_time = VarInt;encode;;decode;
}

minecraft_struct! { KeepAlive
    keep_alive_id = McLong;encode;;decode;
}

minecraft_struct! { ChunkData
    chunk_x = McInteger;encode;;decode;
    chunk_z = McInteger;encode;;decode;
    bit_mask_length = VarInt;encode;;decode;
    primary_bit_mask = Vec<McLong>;encode_arr;Decoder;decode_arr VarInt::from(*bit_mask_length);
    height_maps = NbtTag;encode;;decode;
    biomes_length = VarInt;encode;;decode;
    biomes = Vec<VarInt>;encode_arr;Decoder;decode_arr VarInt::from(*biomes_length);
    size = VarInt;encode;;decode;
    data = Vec<McByte>;encode_arr;Decoder;decode_arr VarInt::from(*size);
    number_of_block_entities = VarInt;encode;;decode;
    block_entities = Vec<NbtTag>;encode_arr;Decoder;decode_arr VarInt::from(*number_of_block_entities);
}

minecraft_struct! { Effect
    effect_id = McInteger;encode;;decode;
    location = Position;encode;;decode;
    data = McInteger;encode;;decode;
    disable_relative_volume = McBoolean;encode;;decode;
}

// todo Particle (https://wiki.vg/Protocol#Particle_2)
// todo UpdateLight (https://wiki.vg/Protocol#Update_Light)

minecraft_struct! { JoinGame
    entity_id = McInteger;encode;;decode;
    is_hardcore = McBoolean;encode;;decode;
    gamemode = McUnsignedByte;encode;;decode;
    previous_gamemode = McByte;encode;;decode;
    world_count = VarInt;encode;;decode;
    world_names = Vec<Identifier>;encode_arr;Decoder;decode_arr VarInt::from(*world_count);
    dimension_codec = NbtTag;encode;;decode;
    dimension = NbtTag;encode;;decode;
    world_name = Identifier;encode;;decode;
    hashed_seed = McLong;encode;;decode;
    max_players = VarInt;encode;;decode;
    view_distance = VarInt;encode;;decode;
    reduced_debug_info = McBoolean;encode;;decode;
    enable_respawn_screen = McBoolean;encode;;decode;
    is_debug = McBoolean;encode;;decode;
    is_flat = McBoolean;encode;;decode;
}

// todo MapData (https://wiki.vg/Protocol#Map_Data)
// todo TradeList (https://wiki.vg/Protocol#Trade_List)

minecraft_struct! { EntityPosition
    entity_id = VarInt;encode;;decode;
    delta_x = McShort;encode;;decode;
    delta_y = McShort;encode;;decode;
    delta_z = McShort;encode;;decode;
    on_ground = McBoolean;encode;;decode;
}
