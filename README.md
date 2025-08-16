# Anotherland

![Anotherland Logo](/images/anotherland-logo.jpg)

## Introduction
Welcome to the **Anotherland Server Emulator** project, a community-driven initiative to resurrect the MMORPG, *Otherland*. This emulator aims to recreate the experience of *Otherland* by providing tools and instructions to set up and run your own server for the game.

## Prerequisites
Before you begin, ensure you have the following installed:
- **Rust 1.89.0-nightly** (use rustup to automatically install the correct toolchain)
- A legally acquired copy of the game (*Otherland Next*)
- **MongoDB** (You need to configure a replica set for transactions to work)

## Compilation
Clone the repository to your local machine:
```bash
git clone https://github.com/AnotherlandServer/anotherland.git
```

Checkout all submodules:
```bash
cd anotherland
git submodule update --init
```

Set the environment variable `OTHERLAND_CLIENT_PATH` to the path of your game client installation.

Build the project using Cargo:
```bash
cargo build
```

## Architecture
The server is divided into multiple services that can be run across distributed machines.

### Per-cluster services:
- **`core_service`**  
  Provides central services like authentication and realm registration. It serves as the entry point for clients and the central registry for realms. This service also exposes a GraphQL API for internal use.
  
- **`login_server`**  
  The primary server for clients to connect to. This is the gateway to the *Otherland* cluster and allows clients to select a realm to log into. It exposes the main RakNet connection, the TCP queue, and the TCP verification service. After login, clients are redirected to a `frontend_server` of the selected realm.

### Per-realm services:
- **`realm_manager_service`**
  The cornerstone for each realm. It manages the realm's database and acts as a registry for cluster frontends and distributed zones.
  
- **`world_service`**
  Responsible for running the games world. 

- **`frontend_server`**
  A staging area for clients immediately after connecting to a realm. It serves clients until they’ve selected a character to play as, at which point they are redirected to one of the `cluster_server` endpoints.
  
- **`cluster_server`**
  Routes connected clients’ requests to the appropriate zone or dungeon server.

## Running the Server
### Bootstrap
1. Start the `core_service` and wait for the GraphQL interface to become available. Use the [Rover CLI](https://www.apollographql.com/docs/rover) with the `rover dev` command to launch an IDE for interfacing with the GraphQL API.
2. Use the `createRealm` mutation to create your first realm.
3. Start the `realm_manager_service` with the `--realm-id` parameter, specifying the ID of the realm you just created, to begin setting up the realm.
4. Once the service is fully started, use the `seed-realm` tool to seed the realm database by extracting content from the *Otherland* client files.

5. After completing these steps, start the remaining services and connect to your realm.

### General Notes
- Use the `--help` argument with each process to view available options and their default values.  
- When specifying public addresses (e.g., for the `frontend_server`), avoid using `127.0.0.1`, as the *Otherland* client may struggle to resolve it.

## Connecting to a Server
1. Open `UnrealEgine3/AmunGame/Config/DefaultUI.ini` within the client’s folder and locate the line:
   ```
   +ConfigureLoginAddress=(srvName="#UI.EU_Server_LIVE#", srvAddress="78.46.105.144", srvPort=6112, queuePort=53292)
   ```
   Replace the `srvName` option with any name you like. Set `srvAddress` to the public IP of your server. If you run Anotherland with default parameters, you don't need to modify `srvPort` or `queuePort`.

2. Open `Atlas/data/otherlandgame/config/clientcfg.ini` and locate the line:
   ```
   verificationSrv =78.46.105.144
   ```
   Replace the IP address with the public IP of your server to enable Steam login.

## Contribution
Currently, we are not accepting contributions as Anotherland is in its initial development phase. Our immediate goal is to build a stable foundation for the project. We appreciate your interest and enthusiasm and intend to open the project for community contributions in the future.

## License
This project is licensed under the [AGPL-3.0 License](LICENSE).

## Disclaimer
This project is a fan-based initiative and is not officially affiliated with, endorsed by, or connected to any of the original creators or entities involved in the development of *Otherland*, including Game OL GmbH, DRAGO Entertainment S.A., or Tad Williams. This emulator is developed and maintained by enthusiasts with no commercial intent and respects the intellectual property rights of the original creators.
