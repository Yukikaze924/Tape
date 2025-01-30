use colored::Colorize;

pub enum IMDbCategory {
    Movies = 0,
    Series = 1,
    Documentaries = 2,
    Films = 3,
}

impl IMDbCategory {
    pub fn from_int(index: usize) -> Self {
        match index {
            0 => Self::Movies,
            1 => Self::Series,
            2 => Self::Documentaries,
            3 => Self::Films,
            _ => unreachable!(),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            IMDbCategory::Movies => "movies".to_string(),
            IMDbCategory::Series => "series".to_string(),
            IMDbCategory::Documentaries => "documentaries".to_string(),
            IMDbCategory::Films => "films".to_string(),
        }
    }
    pub fn count() -> usize {
        return 4;
    }
    pub fn get_color_list() -> Vec<String> {
        return vec![
            "Movies".truecolor(255, 140, 0).bold().to_string(),
            "TV Series".truecolor(255, 215, 0).bold().to_string(),
            "Documentaries".truecolor(0, 255, 255).bold().to_string(),
            "Short Films".truecolor(0,191,255).bold().to_string()
        ];
    }
}


macro_rules! mpaa_classification {
    ( $( $name:ident => $description:expr ),* ) => {
        #[derive(Debug)]
        pub enum MPAAClassification {
            $(
                $name,
            )*
        }

        impl MPAAClassification {
            pub fn from_integer(index: usize) -> Self {
                match index {
                    $(
                        x if x == MPAAClassification::$name as usize => MPAAClassification::$name,
                    )*
                    _ => unreachable!(),
                }
            }

            pub fn to_string(&self) -> String {
                match self {
                    $(
                        MPAAClassification::$name => $description.to_string(),
                    )*
                }
            }

            pub fn to_string_array() -> Vec<String> {
                vec![
                    $(
                        $description.to_string(),
                    )*
                ]
            }
        }
    };
}

mpaa_classification!(
    G => "G",
    PG => "PG",
    PG13 => "PG-13",
    R => "R",
    NC17 => "NC-17"
);