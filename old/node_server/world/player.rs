pub enum PlayerSpawnMode {
    LoginFirstTime, // 1
    LoginNormal, // 2
    TravelDirect, // 3
    TravelPortal, // 4
    TravelCarrier, // 5
    TravelPoint, // 6   
}

impl Into<i32> for PlayerSpawnMode {
    fn into(self) -> i32 {
        match self {
            Self::LoginFirstTime => 1,
            Self::LoginNormal => 2,
            Self::TravelDirect => 3,
            Self::TravelPortal => 4,
            Self::TravelCarrier => 5,
            Self::TravelPoint => 6,
        }
    }
}