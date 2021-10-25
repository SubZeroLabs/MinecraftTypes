use crate::{
    primitive::{McBoolean, McByte, McDouble, McFloat, McInteger, McLong, McShort, McUnsignedByte},
    Angle, ChatJson, Decoder, Identifier, McString, McUuid, NbtTag, Position, SlotData, VarInt,
    VarLong,
};
minecraft_struct! { SpawnEntity
    entity_id = VarInt;;decode;
    uuid = McUuid;;decode;
    entity_type = VarInt;;decode;
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    pitch = Angle;;decode;
    yaw = Angle;;decode;
    data = McInteger;;decode;
    velocity_x = McShort;;decode;
    velocity_y = McShort;;decode;
    velocity_z = McShort;;decode;
}

minecraft_struct! { SpawnExperienceOrb
    entity_id = VarInt;;decode;
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    count = McShort;;decode;
}

minecraft_struct! { SpawnLivingEntity
    entity_id = VarInt;;decode;
    uuid = McUuid;;decode;
    entity_type = VarInt;;decode;
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    yaw = Angle;;decode;
    pitch = Angle;;decode;
    head_pitch = Angle;;decode;
    velocity_x = McShort;;decode;
    velocity_y = McShort;;decode;
    velocity_z = McShort;;decode;
}

minecraft_struct! { SpawnPainting
    entity_id = VarInt;;decode;
    uuid = McUuid;;decode;
    motive = VarInt;;decode;
    location = Position;;decode;
    direction = McByte;;decode;
}

minecraft_struct! { SpawnPlayer
    entity_id = VarInt;;decode;
    uuid = McUuid;;decode;
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    yaw = Angle;;decode;
    pitch = Angle;;decode;
}

minecraft_struct! { SculkVibrationSignalBlock
    source_position = Position;;decode;
    destination_identifier = Identifier;;decode;
    destination = Position;;decode;
    arrival_ticks = VarInt;;decode;
}

minecraft_struct! { SculkVibrationSignalEntity
    source_position = Position;;decode;
    destination_identifier = Identifier;;decode;
    destination = VarInt;;decode;
    arrival_ticks = VarInt;;decode;
}

minecraft_struct! { EntityAnimation
    entity_id = VarInt;;decode;
    animation = McUnsignedByte;;decode;
}

minecraft_struct! { Statistics
    count = VarInt;;decode;
    statistic = Vec<VarInt>;Decoder;decode_arr VarInt::from(*count);
}

minecraft_struct! { AcknowledgePlayerDigging
    location = Position;;decode;
    block = VarInt;;decode;
    status = VarInt;;decode;
    successful = McBoolean;;decode;
}

minecraft_struct! { BlockBreakAnimation
    entity_id = VarInt;;decode;
    location = VarInt;;decode;
    destroy_stage = McByte;;decode;
}

minecraft_struct! { BlockEntityData
    location = Position;;decode;
    action = McUnsignedByte;;decode;
    nbt_data = NbtTag;;decode;
}

minecraft_struct! { BlockAction
    location = Position;;decode;
    action_id = McUnsignedByte;;decode;
    action_param = McUnsignedByte;;decode;
    block_type = VarInt;;decode;
}

minecraft_struct! { BlockChange
    location = Position;;decode;
    block_id = VarInt;;decode;
}

// todo Boss Bar (https://wiki.vg/Protocol#Boss_Bar)

minecraft_struct! { ServerDifficulty
    difficulty = McUnsignedByte;;decode;
    difficulty_locked = McBoolean;;decode;
}

minecraft_struct! { ChatMessage
    chat_json = ChatJson;;decode;
    position = McByte;;decode;
    sender = McUuid;;decode;
}

minecraft_struct! { ClearTitles
    reset = McBoolean;;decode;
}

// todo Tab-Complete (https://wiki.vg/Protocol#Tab-Complete_.28clientbound.29)

// todo Declare Commands (https://wiki.vg/Protocol#Declare_Commands)

minecraft_struct! { CloseWindow
    window_id = McUnsignedByte;;decode;
}

minecraft_struct! { WindowItems
    window_id = McUnsignedByte;;decode;
    state_id = VarInt;;decode;
    count = VarInt;;decode;
    slot_data = Vec<SlotData>;Decoder;decode_arr VarInt::from(*count);
}

minecraft_struct! { WindowProperty
    window_id = McUnsignedByte;;decode;
    property = McShort;;decode;
    value = McShort;;decode;
}

minecraft_struct! { SetSlot
    window_id = McUnsignedByte;;decode;
    state_id = VarInt;;decode;
    slot = McShort;;decode;
    slot_data = SlotData;;decode;
}

minecraft_struct! { SetCooldown
    item_id = VarInt;;decode;
    cooldown_ticks = VarInt;;decode;
}

minecraft_struct! { PluginMessage
    channel = Identifier;;decode;
    data = Vec<McUnsignedByte>;Decoder;decode_to_end;
}

minecraft_struct! { NamedSoundEffect
    sound_name = Identifier;;decode;
    sound_category = VarInt;;decode;
    effect_position_x = McInteger;;decode;
    effect_position_y = McInteger;;decode;
    effect_position_z = McInteger;;decode;
    volume = McFloat;;decode;
    pitch = McFloat;;decode;
}

minecraft_struct! { Disconnect
    reason = ChatJson;;decode;
}

minecraft_struct! { EntityStatus
    entity_id = McInteger;;decode;
    entity_status = McByte;;decode;
}

minecraft_struct! { Explosion
    x = McFloat;;decode;
    y = McFloat;;decode;
    z = McFloat;;decode;
    strength = McFloat;;decode;
    record_count = VarInt;;decode;
    records = Vec<(McUnsignedByte, McUnsignedByte, McUnsignedByte)>;Decoder;decode_arr VarInt::from(*record_count);
    player_motion_x = McFloat;;decode;
    player_motion_y = McFloat;;decode;
    player_motion_z = McFloat;;decode;
}

minecraft_struct! { UnloadChunk
    chunk_x = McInteger;;decode;
    chunk_z = McInteger;;decode;
}

minecraft_struct! { ChangeGameState
    reason = McUnsignedByte;;decode;
    value = McFloat;;decode;
}

minecraft_struct! { OpenHorseWindow
    window_id = McByte;;decode;
    number_of_slots = VarInt;;decode;
    entity_id = McInteger;;decode;
}

minecraft_struct! { InitializeWorldBorder
    x = McDouble;;decode;
    z = McDouble;;decode;
    old_diameter = McDouble;;decode;
    new_diameter = McDouble;;decode;
    speed = VarLong;;decode;
    portal_teleport_boundary = VarInt;;decode;
    warning_blocks = VarInt;;decode;
    warning_time = VarInt;;decode;
}

minecraft_struct! { KeepAlive
    keep_alive_id = McLong;;decode;
}

minecraft_struct! { ChunkData
    chunk_x = McInteger;;decode;
    chunk_z = McInteger;;decode;
    bit_mask_length = VarInt;;decode;
    primary_bit_mask = Vec<McLong>;Decoder;decode_arr VarInt::from(*bit_mask_length);
    height_maps = NbtTag;;decode;
    biomes_length = VarInt;;decode;
    biomes = Vec<VarInt>;Decoder;decode_arr VarInt::from(*biomes_length);
    size = VarInt;;decode;
    data = Vec<McByte>;Decoder;decode_arr VarInt::from(*size);
    number_of_block_entities = VarInt;;decode;
    block_entities = Vec<NbtTag>;Decoder;decode_arr VarInt::from(*number_of_block_entities);
}

minecraft_struct! { Effect
    effect_id = McInteger;;decode;
    location = Position;;decode;
    data = McInteger;;decode;
    disable_relative_volume = McBoolean;;decode;
}

// todo Particle (https://wiki.vg/Protocol#Particle_2)
// todo UpdateLight (https://wiki.vg/Protocol#Update_Light)

minecraft_struct! { JoinGame
    entity_id = McInteger;;decode;
    is_hardcore = McBoolean;;decode;
    gamemode = McUnsignedByte;;decode;
    previous_gamemode = McByte;;decode;
    world_count = VarInt;;decode;
    world_names = Vec<Identifier>;Decoder;decode_arr VarInt::from(*world_count);
    dimension_codec = NbtTag;;decode;
    dimension = NbtTag;;decode;
    world_name = Identifier;;decode;
    hashed_seed = McLong;;decode;
    max_players = VarInt;;decode;
    view_distance = VarInt;;decode;
    reduced_debug_info = McBoolean;;decode;
    enable_respawn_screen = McBoolean;;decode;
    is_debug = McBoolean;;decode;
    is_flat = McBoolean;;decode;
}

// todo MapData (https://wiki.vg/Protocol#Map_Data)
// todo TradeList (https://wiki.vg/Protocol#Trade_List)

minecraft_struct! { EntityPosition
    entity_id = VarInt;;decode;
    delta_x = McShort;;decode;
    delta_y = McShort;;decode;
    delta_z = McShort;;decode;
    on_ground = McBoolean;;decode;
}

minecraft_struct! { EntityPositionAndRotation
    entity_id = VarInt;;decode;
    delta_x = McShort;;decode;
    delta_y = McShort;;decode;
    delta_z = McShort;;decode;
    yaw = Angle;;decode;
    pitch = Angle;;decode;
    on_ground = McBoolean;;decode;
}

minecraft_struct! { EntityRotation
    entity_id = VarInt;;decode;
    yaw = Angle;;decode;
    pitch = Angle;;decode;
    on_ground = McBoolean;;decode;
}

minecraft_struct! { VehicleMove
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    yaw = McFloat;;decode;
    pitch = McFloat;;decode;
}

minecraft_struct! { OpenBook
    hand = VarInt;;decode;
}

minecraft_struct! { OpenWindow
    window_id = VarInt;;decode;
    window_type = VarInt;;decode;
    window_title = ChatJson;;decode;
}

minecraft_struct! { OpenSignEditor
    location = Position;;decode;
}

minecraft_struct! { Ping
    id = McInteger;;decode;
}

minecraft_struct! { CraftRecipeResponse
    window_id = McByte;;decode;
    recipe = Identifier;;decode;
}

minecraft_struct! { PlayerAbilities
    flags = McByte;;decode;
    flying_speed = McFloat;;decode;
    field_of_view_modifier = McFloat;;decode;
}

minecraft_struct! { EndCombatEvent
    duration = VarInt;;decode;
    entity_id = McInteger;;decode;
}

minecraft_struct!(EnterCombatEvent);

minecraft_struct! { DeathCombatEvent
    player_id = VarInt;;decode;
    entity_id = McInteger;;decode;
    message = ChatJson;;decode;
}

// todo PlayerInfo (https://wiki.vg/Protocol#Player_Info)

minecraft_struct! { FacePlayer
    feet_eyes = VarInt;;decode;
    target_x = McDouble;;decode;
    target_y = McDouble;;decode;
    target_z = McDouble;;decode;
    is_entity = McBoolean;;decode;
    entity_id = Option<VarInt>;Decoder;decode_if *is_entity;
    entity_feet_eyes = Option<VarInt>;Decoder;decode_if *is_entity;
}

minecraft_struct! { PlayerPositionAndLook
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    yaw = McFloat;;decode;
    pitch = McFloat;;decode;
    flags = McByte;;decode;
    teleport_id = VarInt;;decode;
    dismount_vehicle = McBoolean;;decode;
}

minecraft_struct! { UnlockRecipes
    action = VarInt;;decode;
    crafting_recipe_book_open = McBoolean;;decode;
    crafting_recipe_book_filter_active = McBoolean;;decode;
    smelting_recipe_book_open = McBoolean;;decode;
    smelting_recipe_book_filter_active = McBoolean;;decode;
    blast_furnace_recipe_book_open = McBoolean;;decode;
    blast_furnace_recipe_book_filter_active = McBoolean;;decode;
    smoker_recipe_book_open = McBoolean;;decode;
    smoker_recipe_book_filter_active = McBoolean;;decode;
    arr_size_1 = VarInt;;decode;
    recipe_ids = Vec<Identifier>;Decoder;decode_arr VarInt::from(*arr_size_1);
    arr_size_2 = Option<VarInt>;Decoder;decode_if *action == 0;
    recipe_ids_2 = Option<Vec<Identifier>>;Decoder;decode_arr_if *action == 0 VarInt::from(*arr_size_2.as_ref().unwrap());
}

minecraft_struct! { DestroyEntities
    count = VarInt;;decode;
    entity_ids = Vec<VarInt>;Decoder;decode_arr VarInt::from(*count);
}

minecraft_struct! { RemoveEntityEffect
    entity_id = VarInt;;decode;
    effect_id = McByte;;decode;
}

minecraft_struct! { ResourcePackSend
    url = McString;;decode 32767;
    hash = McString;;decode 40;
    forced = McBoolean;;decode;
    has_prompt_message = McBoolean;;decode;
    prompt_message = Option<ChatJson>;Decoder;decode_if *has_prompt_message;
}

minecraft_struct! { Respawn
    dimension = NbtTag;;decode;
    world_name = Identifier;;decode;
    hashed_seed = McLong;;decode;
    gamemode = McUnsignedByte;;decode;
    previous_gamemode = McUnsignedByte;;decode;
    is_debug = McBoolean;;decode;
    is_flat = McBoolean;;decode;
    copy_metadata = McBoolean;;decode;
}

minecraft_struct! { EntityHeadLook
    entity_id = VarInt;;decode;
    head_yaw = Angle;;decode;
}

minecraft_struct! { MultiBlockChange
    chunk_section_position = McLong;;decode;
    trust_edges_change = McBoolean;;decode;
    blocks_array_size = VarInt;;decode;
    blocks = Vec<VarLong>;Decoder;decode_arr VarInt::from(*blocks_array_size);
}

minecraft_struct! { SelectAdvancementTab
    has_id = McBoolean;;decode;
    optional_identifier = Option<McString>;Decoder;decode_str_if *has_id VarInt::from(32767);
}

minecraft_struct! { ActionBar
    action_bar_text = ChatJson;;decode;
}

minecraft_struct! { WorldBorderCenter
    x = McDouble;;decode;
    z = McDouble;;decode;
}

minecraft_struct! { WorldBorderLerpSize
    old_diameter = McDouble;;decode;
    new_diameter = McDouble;;decode;
    speed = VarLong;;decode;
}

minecraft_struct! { WorldBorderSize
    diameter = McDouble;;decode;
}

minecraft_struct! { WorldBorderWarningDelay
    warning_time = VarInt;;decode;
}

minecraft_struct! { WorldBorderWarningReach
    warning_blocks = VarInt;;decode;
}

minecraft_struct! { Camera
    camera_id = VarInt;;decode;
}

minecraft_struct! { HeldItemChange
    slot = McByte;;decode;
}

minecraft_struct! { UpdateViewPosition
    chunk_x = VarInt;;decode;
    chunk_z = VarInt;;decode;
}

minecraft_struct! { UpdateViewDistance
    view_distance = VarInt;;decode;
}

minecraft_struct! { SpawnPosition
    location = Position;;decode;
    angle = McFloat;;decode;
}

minecraft_struct! { DisplayScoreboard
    position = McByte;;decode;
    score_name = McString;;decode 16;
}

// todo EntityMetadata (https://wiki.vg/Protocol#Entity_Metadata)

minecraft_struct! { AttachEntity
    attached_entity_id = McInteger;;decode;
    holding_entity_id = McInteger;;decode;
}

minecraft_struct! { EntityVelocity
    entity_id = VarInt;;decode;
    velocity_x = McShort;;decode;
    velocity_y = McShort;;decode;
    velocity_z = McShort;;decode;
}

// todo EntityEquipment (https://wiki.vg/Protocol#Entity_Equipment)

minecraft_struct! { SetExperience
    experience_bar = McFloat;;decode;
    level = VarInt;;decode;
    total_experience = VarInt;;decode;
}

minecraft_struct! { UpdateHealth
    health = McFloat;;decode;
    food = VarInt;;decode;
    food_saturation = McFloat;;decode;
}

minecraft_struct! { ScoreboardObjective
    objective_name = McString;;decode 16;
    mode = McByte;;decode;
    objective_value = Option<ChatJson>;Decoder;decode_if *mode == 0 || *mode == 1;
    objective_type = Option<VarInt>;Decoder;decode_if *mode == 0 || *mode == 1;
}

minecraft_struct! { SetPassengers
    entity_id = VarInt;;decode;
    passenger_count = VarInt;;decode;
    passengers = Vec<VarInt>;Decoder;decode_arr VarInt::from(*passenger_count);
}

// todo Teams (https://wiki.vg/Protocol#Teams)

minecraft_struct! { UpdateScore
    entity_name = McString;;decode 40;
    action = McByte;;decode;
    objective_name = McString;;decode 16;
    value = Option<VarInt>;Decoder;decode_if *action != 1;
}

minecraft_struct! { SetTitleSubtitle
    subtitle_text = ChatJson;;decode;
}

minecraft_struct! { TimeUpdate
    world_age = McLong;;decode;
    time_of_date = McLong;;decode;
}

minecraft_struct! { SetTitleText
    title_text = ChatJson;;decode;
}

minecraft_struct! { SetTitleTimes
    fade_in = McInteger;;decode;
    stay = McInteger;;decode;
    fade_out = McInteger;;decode;
}

minecraft_struct! { EntitySoundEffect
    sound_id = VarInt;;decode;
    sound_category = VarInt;;decode;
    entity_id = VarInt;;decode;
    volume = McFloat;;decode;
    pitch = McFloat;;decode;
}

minecraft_struct! { SoundEffect
    sound_id = VarInt;;decode;
    sound_category = VarInt;;decode;
    effect_position_x = McInteger;;decode;
    effect_position_y = McInteger;;decode;
    effect_position_z = McInteger;;decode;
    volume = McFloat;;decode;
    pitch = McFloat;;decode;
}

minecraft_struct! { StopSound
    flags = McByte;;decode;
    source = Option<VarInt>;Decoder;decode_if *flags == 3 || *flags == 1;
    sound = Option<VarInt>;Decoder;decode_if *flags == 2 || *flags == 3;
}

minecraft_struct! { PlayerListHeaderAndFooter
    header = ChatJson;;decode;
    footer = ChatJson;;decode;
}

minecraft_struct! { NbtQueryResponse
    transaction_id = VarInt;;decode;
    nbt = NbtTag;;decode;
}

minecraft_struct! { CollectItem
    collected_entity_id = VarInt;;decode;
    collector_entity_id = VarInt;;decode;
    pickup_item_count = VarInt;;decode;
}

minecraft_struct! { EntityTeleport
    entity_id = VarInt;;decode;
    x = McDouble;;decode;
    y = McDouble;;decode;
    z = McDouble;;decode;
    yaw = Angle;;decode;
    pitch = Angle;;decode;
    on_ground = McBoolean;;decode;
}

// todo Advancements (https://wiki.vg/Protocol#Advancements)
// todo EntityProperties (https://wiki.vg/Protocol#Entity_Properties)

minecraft_struct! { EntityEffect
    entity_id = VarInt;;decode;
    effect_id = McByte;;decode;
    amplifier = McByte;;decode;
    duration = VarInt;;decode;
    flags = McByte;;decode;
}

// todo DeclareRecipes (https://wiki.vg/Protocol#Declare_Recipes)
// todo Tags (https://wiki.vg/Protocol#Tags)
