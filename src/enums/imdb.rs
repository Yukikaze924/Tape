pub enum IMDbCategory {
    Movies = 1,
    Series = 2,
    Documentaries = 3,
    Films = 4,
}

impl IMDbCategory {
    pub fn from_interger(index: usize) -> Self {
        match index {
            1 => Self::Movies,
            2 => Self::Series,
            3 => Self::Documentaries,
            4 => Self::Films,
            _ => Self::Movies,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            IMDbCategory::Movies => "Movies".to_string(),
            IMDbCategory::Series => "TV Series".to_string(),
            IMDbCategory::Documentaries => "Documentaries".to_string(),
            IMDbCategory::Films => "Films".to_string(),
        }
    }
}