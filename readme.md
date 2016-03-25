### pubsub-rs

#### simple tcp based pubsub for rust.

#### overview

pubsub-rs is a simple text based messaging service that 
supports topic based publish/subscribe over tcp.

pubsub-rs implements a simple text based messaging protocol that can be driven 
from telnet, netcat or other tcp enabled clients to syndicate messages 
around a distributed network.

pubsub-rs is a work in progress.

### protocol commands

The following are the supported commands.
```
i:[user]            - identity this user.
s:[topic]           - subscribes to this topic.
u:[topic]           - unsubscribes from this topic.
p:[topic]:[message] - publish a message to this topic.
```

### receiving messages

Once a socket has subscribed to a topic, they will receive messages
in the following form.
```
m:[topic]:[user]:[message]
```
