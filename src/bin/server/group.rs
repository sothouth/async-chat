
use std::{collections::HashMap, sync::Arc};

use crate::connection::Outbound;


pub struct Group{
    members:Vec<Arc<Outbound>>,
}

impl Group{
    pub fn new()->Self{
        Self{
            members:Vec::new(),
        }
    }
    pub fn join(&mut self,member:Arc<Outbound>){
        self.members.push(member);
    }
}
