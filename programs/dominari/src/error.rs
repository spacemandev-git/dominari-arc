use anchor_lang::prelude::*;

#[error_code]
pub enum DominariError {
    #[msg("Entity must be empty!")]
    InvalidEntity,

    #[msg("Instance already has max players!")]
    PlayerCountExceeded,

    #[msg("Players can only PAUSE or PLAY their game!")]
    InvalidPlayPhase,

    #[msg("Only players in this game can call this function!")]
    InvalidPlayer,

    #[msg("Game Paused")]
    GamePaused,
}

#[error_code]
pub enum ComponentErrors {
    #[msg("Invalid Owner!")]
    InvalidOwner,

    #[msg("Friendly fire not allowed!")]
    FriendlyFire,

    #[msg("String too long!")]
    StringTooLong,

    #[msg("Tile Occupied")]
    TileOccupied,

    #[msg("Player doesn't have that card")]
    InvalidCard,

    #[msg("Invalid Unit")]
    InvalidUnit,

    #[msg("Unit is recovering from last move")]
    UnitRecovering,

    #[msg("Unit cannot move that far")]
    UnitLacksMovement,
    
    #[msg("Trying to interact with a dead unit")]
    UnitDead,

    #[msg("Defender doesn't have a health bar")]
    NoHealthComponent,

    #[msg("Unit out of range")]
    OutOfRange,

    #[msg("Invalid Location")]
    InvalidLocation,
}
