/*--------------------------------------------------------------------------
 pubsub-rs

 The MIT License (MIT)

 Copyright (c) 2016 Haydn Paterson (sinclair) <haydn.developer@gmail.com>

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in
 all copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 THE SOFTWARE.
---------------------------------------------------------------------------*/



use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use super::super::protocol::Command;

///------------------------------------
/// Topic
///
/// Manages a collection of streams
/// bound to a given topic.
///------------------------------------
pub struct Topic {
    dict : Arc<Mutex<HashMap<String, TcpStream>>>    
}
impl Topic {
    pub fn new() -> Topic {
        Topic {
            dict: Arc::new(Mutex::new(HashMap::new()))
        }
    }
    ///-----------------------------------------
    /// subscribes this user to this topic.
    ///----------------------------------------- 
    pub fn subscribe(&self, user_id: String, stream: TcpStream) {
        let mut dict = self.dict.lock().unwrap();
        if !dict.contains_key(&user_id) {
            dict.insert(user_id, stream);
        }
    }
    
    ///-----------------------------------------
    /// unsubscribes this user from this topic.
    ///----------------------------------------- 
    pub fn unsubscribe(&self, user_id: String)  {
        let mut dict = self.dict.lock().unwrap();
        if dict.contains_key(&user_id) {
            dict.remove(&user_id);    
        }
    }
    // -----------------------------------------
    // publishes this message to this topic.
    // ----------------------------------------- 
    pub fn publish(&self, message: String) {
        let dict = self.dict.lock().unwrap();
        let mut bytes = message.into_bytes();
        for (_, mut stream) in dict.iter() {
            stream.write(&mut bytes).unwrap();
        }
    }
}
//------------------------------------
// Store
//------------------------------------
#[derive(Clone)]
pub struct Topics {
    dict: Arc<Mutex<HashMap<String, Topic>>>    
}
impl Topics {
    pub fn new() -> Topics {
        Topics {
            dict: Arc::new(Mutex::new(HashMap::new()))  
        }
    }
    ///-----------------------------------------
    /// subscribes this user to this topic.
    ///----------------------------------------- 
    pub fn subscribe(&self, topic_id: String, user_id: String, stream: TcpStream) {
        let mut dict = self.dict.lock().unwrap();
        if dict.contains_key(&topic_id) {
            let topic = dict.get(&topic_id).unwrap();
            topic.subscribe(user_id, stream);
        } else {
            let topic = Topic::new();
            topic.subscribe(user_id, stream);
            dict.insert(topic_id, topic);
        }
    }
    
    ///-----------------------------------------
    /// unsubscribes this user from this topic.
    ///-----------------------------------------
    pub fn unsubscribe(&self, topic_id: String, user_id: String) {
        let dict = self.dict.lock().unwrap();
        if dict.contains_key(&topic_id) {
            let topic = dict.get(&topic_id).unwrap();
            topic.unsubscribe(user_id);
        }
    }
    
    ///-----------------------------------------
    // publishes this message.
    ///----------------------------------------- 
    pub fn publish(&self, topic_id: String, user_id: String, message: String) {
        let dict = self.dict.lock().unwrap();
        if dict.contains_key(&topic_id) {
            let topic   = dict.get(&topic_id).unwrap();
            let command = Command::Message(topic_id, user_id, message);
            topic.publish(command.serialize());
        }
    }        
}