use crate::{priv_as_ref};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Info {
    pub scope:String,
    pub name:String
}
priv_as_ref!(Info);
pub trait GetInfo {
    fn get_info(&self)->Info;
}
impl GetInfo for Info {
    fn get_info(&self)->Info {
        return self.clone();
    }
}