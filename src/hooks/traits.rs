pub trait Hookable {
    type Err;
    fn get_hook(&self) -> &'static str;
    fn set_hook(&self, shell_config_path: Option<&str>) -> Result<String, Self::Err>;
}