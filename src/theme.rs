use std::path::PathBuf;

use forecast::Icon;

pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn icon_path(&self, icon: &Icon) -> Option<PathBuf> {
        match *icon {
            Icon::ClearDay => Some("Sun"),
            Icon::ClearNight => Some("Moon"),
            Icon::Rain => Some("Cloud-Rain"),
            Icon::Snow => Some("Cloud-Snow"),
            Icon::Sleet => Some("Cloud-Snow-Alt"),
            Icon::Wind => Some("Wind"),
            Icon::Fog => Some("Cloud-Fog"),
            Icon::Cloudy => Some("Cloud"),
            Icon::PartlyCloudyDay => Some("Cloud-Sun"),
            Icon::PartlyCloudyNight => Some("Cloud-Moon"),
            Icon::Unknown(_) => None,
        }.map(|x| {
            let theme = match *self {
                Theme::Light => "Light",
                Theme::Dark => "Dark",
            };
            format!("{}-{}", theme, x).into()
        })
    }
}
