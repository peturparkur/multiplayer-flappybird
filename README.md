# Multiplayer Flappy-Bird

Websocket server using rust [actix](https://actix.rs/) framework.

The code was adapted from a simple chat/messaging room/lobby server.

## Game
The game code is contained within the [game](./src/game/) folder.

## Server/Network

We have two layers for the websocket communication.

### [Client](./src/socket.rs)

This contains the front-facing object that receives the messages from the front-end (or bots) and converts them into a more directed message to the [room-server](./src/game_lobby.rs)

### [Room](./src/game_lobby.rs)

This contains the meat of the code. It handles the messages received from the front-facing [client](./src/socket.rs), and send messages back to it.

The way the games are being created is a client can connect to the an endpoint such as `ws://localhost:8080/{room_id}`. If the room_id doesn't exist yet it will be created 
(NOTE: this should be changed for production as we don't want clients to spam create new lobbies).

Once within the lobby a game instance is created for the lobby and the user can either just spectate (not sending input), or create a character and ready up (send `!ready` after an input was sent). Currently input is a simple boolean `true`/`false` which will set the jump-state of the [bird](./src/game/objects/bird.rs) to the given value