mod active_record;

use crate::prelude::*;
pub use active_record::ActiveRecord;
use std::ops::{Deref, DerefMut};

pub struct ActiveModel<M> {
    record: ActiveRecord,
}

impl<M: ArelModel> Deref for ActiveModel<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.record
    }
}
impl<M: ArelModel> DerefMut for ActiveModel<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.record
    }
}

impl<M: ArelModel> ActiveModel<M> {
    pub fn validates(&self) -> bool {
        todo!()
    }
    pub fn is_changed(&self) -> bool {
        todo!()
    }
    pub async fn save(&mut self) -> anyhow::Result<()> {
        todo!()
    }
    pub async fn force_reload(&mut self) {
        todo!()
    }
}
