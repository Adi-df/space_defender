use std::sync::Arc;

use hecs::{Entity, World};

pub struct Life<T>
where
    T: Send + Sync + 'static + FnOnce(&mut World, &Entity),
{
    pub life: u16,
    death: Arc<Box<T>>,
}

impl<T> Life<T>
where
    T: Send + Sync + 'static + FnOnce(&mut World, &Entity),
{
    pub fn new(l: u16, d: Box<T>) -> Self {
        Self {
            life: l,
            death: Arc::new(d),
        }
    }
}
