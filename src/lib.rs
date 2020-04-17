
#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{OP_BIND_ACTOR, OP_REMOVE_ACTOR, CapabilityConfiguration};
use codec::deserialize;

use std::error::Error;
use std::sync::RwLock;

const SYSTEM_ACTOR: &str = "system";

#[cfg(not(feature = "static_plugin"))]
capability_provider!({{project-name | pascal_case }}Provider, {{project-name | pascal_case }}Provider::new);

const CAPABILITY_ID: &str = "new:{{project-name}}"; // TODO: change this to an appropriate capability ID

pub struct {{project-name | pascal_case }}Provider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
}

impl Default for {{project-name | pascal_case }}Provider {
    fn default() -> Self {
        let _ = env_logger::try_init();

        {{project-name | pascal_case}}Provider { 
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),           
        }
    }
}

impl {{project-name | pascal_case}}Provider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error>> {        

        // Handle actor binding metadata here...
        // This is typically where you would establish a
        // client or connection to a resource on behalf of
        // an actor

        Ok(vec![])
    }   
    
    fn deconfigure(
        &self,
        _config: CapabilityConfiguration,        
    ) -> Result<Vec<u8>, Box<dyn Error>> {

        // Handle removal of resources claimed by an actor here
        Ok(vec![])
    }
}

impl CapabilityProvider for {{project-name | pascal_case}}Provider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "New {{ project-name | pascal_case }} Capability Provider" // TODO: change this friendly name
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {            
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.deconfigure(deserialize(msg)?),
            _ => Err("bad dispatch".into()),
        }
    }
}
