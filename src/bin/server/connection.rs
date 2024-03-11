use std::sync::Arc;

use async_std::prelude::*;
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_chat::{FromClient,FromServer};
use async_chat::utils::{self, ChatResult};


use crate::group_table::GroupTable;



pub async fn serve(socket:TcpStream,groups:Arc<GroupTable>)->ChatResult<()>{
    let outbound=Arc::new(Outbound::new(socket.clone()));
    Ok(())
}


use async_std::sync::Mutex;
pub struct Outbound(Mutex<TcpStream>);

impl Outbound{
    pub fn new(to_client:TcpStream)->Self{
        Self(Mutex::new(to_client))
    }
    pub async fn send(&self,packet:FromServer)->ChatResult<()>{
        let mut guard=self.0.lock().await;

        utils::send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }
}
