use pubsub::protocol::Command;

#[test]
fn parse_ok() {
   Command::parse("i:user").unwrap();
   Command::parse("s:topic").unwrap();
   Command::parse("u:topic").unwrap();
   Command::parse("p:topic:hello world").unwrap();
}

#[test]
#[should_panic]
fn parse_panic() {
   Command::parse("user").unwrap();
}