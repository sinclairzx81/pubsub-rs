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
    pub fn subscribe(&self, user_key: String, stream: TcpStream) {
        let mut dict = self.dict.lock().unwrap();
        if !dict.contains_key(&user_key) {
            dict.insert(user_key, stream);
        }
    }
    
    ///-----------------------------------------
    /// unsubscribes this user from this topic.
    ///----------------------------------------- 
    pub fn unsubscribe(&self, user_key: String)  {
        let mut dict = self.dict.lock().unwrap();
        if dict.contains_key(&user_key) {
            dict.remove(&user_key);    
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

    ///-----------------------------------------
    /// renames this user_key to a new key.
    ///-----------------------------------------     
    pub fn rename_user_key(&self, old_user_key: String, new_user_key: String) {
        let mut dict = self.dict.lock().unwrap();
        if dict.contains_key(&old_user_key) {
            let stream = dict.remove(&old_user_key).unwrap();
            dict.insert(new_user_key, stream);
        }
    }
    
    ///-----------------------------------------
    /// deletes this user_key.
    ///----------------------------------------- 
    pub fn delete_user_key(&self, user_key:String) {
        let mut dict = self.dict.lock().unwrap();
        if dict.contains_key(&user_key) {
            dict.remove(&user_key).unwrap();
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
    pub fn subscribe(&self, topic_key: String, user_key: String, stream: TcpStream) {
        let mut dict = self.dict.lock().unwrap();
        if dict.contains_key(&topic_key) {
            let topic = dict.get(&topic_key).unwrap();
            topic.subscribe(user_key, stream);
        } else {
            let topic = Topic::new();
            topic.subscribe(user_key, stream);
            dict.insert(topic_key, topic);
        }
    }
    
    ///-----------------------------------------
    /// unsubscribes this user from this topic.
    ///-----------------------------------------
    pub fn unsubscribe(&self, topic_key: String, user_key: String) {
        let dict = self.dict.lock().unwrap();
        if dict.contains_key(&topic_key) {
            let topic = dict.get(&topic_key).unwrap();
            topic.unsubscribe(user_key);
        }
    }
    
    ///-----------------------------------------
    /// publishes this message.
    ///----------------------------------------- 
    pub fn publish(&self, topic_key: String, user_key: String, message: String) {
        let dict = self.dict.lock().unwrap();
        if dict.contains_key(&topic_key) {
            let topic   = dict.get(&topic_key).unwrap();
            let command = Command::Message(topic_key, user_key, message);
            topic.publish(command.serialize());
        }
    }
    
    ///-----------------------------------------
    /// renames this user_key.
    ///-----------------------------------------     
    pub fn rename_user_key(&self, old_user_key: String, new_user_key:String) {
        let dict = self.dict.lock().unwrap();
        for (_, topic) in dict.iter() {
            topic.rename_user_key(
                old_user_key.clone(), 
                new_user_key.clone());
        }
    } 
    
    ///-----------------------------------------
    /// deletes this user_key.
    ///-----------------------------------------     
    pub fn delete_user_key(&self, user_key: String) {
        let dict = self.dict.lock().unwrap();
        for (_, topic) in dict.iter() {
            topic.delete_user_key(user_key.clone());
        }
    }          
}