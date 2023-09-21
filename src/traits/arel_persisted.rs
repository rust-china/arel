pub trait ArelPersisted {
    fn set_persisted(&mut self, persisted: bool);
    fn persited(&self) -> bool;
}
