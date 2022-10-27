use anyhow::Result;

pub trait NotificationSerice {
    fn name(&self) -> String;
    fn send(&self) -> Result<()>;
}
