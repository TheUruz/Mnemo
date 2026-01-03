#[derive(Debug)]
pub struct Emojis {
    pub folder: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub unknown: &'static str,
    pub executable: &'static str,
    pub mnemo: &'static str
}

pub const EMOJIS: Emojis = Emojis {
    folder: "📁",
    warning: "⚠️",
    info: "ℹ️",
    unknown: "❔",
    executable: "⚙️",
    mnemo: "🤖"
};
