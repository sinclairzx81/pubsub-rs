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

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::prelude::*;
use std::io::{BufReader, Result};
use std::thread;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use super::super::protocol::Command;
use super::topics::Topics;


///-----------------------------------------
/// Client
///
/// Internally manages a server client.
///-----------------------------------------
struct Client;
impl Client {
    ///----------------------------------------------------------
    /// creates a new client with this topic store and stream.
    ///----------------------------------------------------------
    pub fn create(topics: Topics, stream : TcpStream) -> Result<()> {
        
        //-----------------------------------------
        // initialize client state.
		//----------------------------------------- 
        let mut reader   = BufReader::new(stream.try_clone().unwrap());
        let mut buffer   = String::new();
        let     user_key = Arc::new(Mutex::new(Uuid::new_v4().to_hyphenated_string()));
        
        // -----------------------------------------
        // read from stream.
		// -----------------------------------------
        while try!(reader.read_line(&mut buffer)) > 0 {
            let user_key = user_key.clone();
            match Command::parse(&buffer) {
                Ok(command) => match command {
                    Command::Identity(new_user_key) => {
                        let mut user_key = user_key.lock().unwrap();
                        topics.rename_user_key(user_key.clone(), new_user_key.clone());
                        *user_key = new_user_key;
                    },
                    Command::Subscribe(topic_key) => {
                        let user_key = user_key.lock().unwrap();
                        let stream   = stream.try_clone().unwrap();
                        topics.subscribe(topic_key, user_key.clone(), stream);
                    },
                    Command::Unsubscribe(topic_key) => {
                        let user_key = user_key.lock().unwrap();
                        topics.unsubscribe(topic_key, user_key.clone());                       
                    },
                    Command::Publish(topic_key, message) => {
                        let user_key = user_key.lock().unwrap();
                        topics.publish(topic_key, user_key.clone(), message);
                    },
                    _ => { /* do nothing */ }
                }, Err(error) => println!("{:?}", error)
            }; buffer.clear();  
        }
        println!("ENDED");
        Ok(())
    }
}

///-----------------------------------------
/// Server
///
/// Sets up a tcp listener, listens on the 
/// given port.
///-----------------------------------------
pub struct Server;
impl Server {
    
    ///------------------------------------------
    /// binds a pubsub server to this addr.
    ///------------------------------------------
    pub fn bind<T: ToSocketAddrs>(addr: T, topics: Topics) -> Result<()> {
        let listener = try!(TcpListener::bind(addr));
        for stream in listener.incoming() {
            let stream  = try!(stream);
            let topics  = topics.clone();
            let _       = thread::spawn(move || Client::create(topics, stream));
        } Ok(())
    }
}