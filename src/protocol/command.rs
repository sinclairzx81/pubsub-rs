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

use std::error;
use std::fmt;

///---------------------------------------------------------------------
///
/// ParseError:
///
///---------------------------------------------------------------------
#[derive(Debug)]
pub struct ParseError {
    input: String
}
impl ParseError {
    pub fn new(input: String) -> ParseError {
        ParseError { input: input }
    } 
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: - {}", self.input)
    }
}
impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.input
    }
}

/// Command:
///
/// Protocol command type passed along transport. The protocol is 
/// a simple text based protocol with components of each command
/// delimited by ':'. the following outlines the protocol.
/// 
///  i:user               - (client->server) identifies this user.
///  s:topic              - (client->server) subscribes to this topic.
///  u:topic              - (client->server) unsubscribes from this topic.
///  p:topic:message      - (client->server) publishes this message to this topic.
///  m:topic:user:message - (server->client) a published message sent to this topic.
///
#[derive(Debug)]
pub enum Command {
  Identity      (String),       
  Subscribe     (String),        
  Unsubscribe   (String),        
  Publish       (String, String),
  Message       (String, String, String)
}

impl Command {
  /// serializes this command to string.
  pub fn serialize(&self) -> String {
    match *self {
        Command::Identity    (ref user)    => format!("i:{}", user),
        Command::Subscribe   (ref topic)   => format!("s:{}", topic),
        Command::Unsubscribe (ref topic)   => format!("u:{}", topic),
        Command::Publish     (ref topic, 
                              ref message) => format!("p:{}:{}", topic, message),
        Command::Message     (ref topic, 
                              ref user, 
                              ref message) => format!("m:{}:{}:{}", topic, user, message)
    }
  }
  
  /// parse this line into a protocol command.
  pub fn parse(command: &str) -> Result<Command, ParseError> {
    let command = command.trim_right();
    let split   = command.splitn(2, ":").collect::<Vec<_>>();
    if split.len() == 2 {
      match split[0] {
        "i" => {
           let user    = split[1].to_string();
           let command = Command::Identity(user);
           return Ok(command); 
        },
        "s" => {
           let topic   = split[1].to_string();
           let command = Command::Subscribe(topic);
           return Ok(command); 
        },
        "u" => {
           let topic   = split[1].to_string();
           let command = Command::Unsubscribe(topic);
           return Ok(command); 
        },
        "p" => {
            let split = split[1].splitn(2, ":").collect::<Vec<_>>();
            if split.len() == 2 {
                let topic   = split[0].to_string();
                let message = split[1].to_string();
                let command = Command::Publish(topic, message);
                return Ok(command);
            }
        },
        "m" => {
            let split = split[1].splitn(2, ":").collect::<Vec<_>>();
            if split.len() == 2 {
                let topic = split[0].to_string();
                let split = split[1].splitn(2, ":").collect::<Vec<_>>();
                if split.len() == 2 {
                    let user    = split[0].to_string();
                    let message = split[1].to_string();
                    let command = Command::Message(topic, user, message);
                    return Ok(command);
                }
            }
        }, _ => { /* defer to error.*/ }
      }
    } return Err(ParseError::new(command.to_string()));
  }
}
