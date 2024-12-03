# Anotherland

![Anotherland Logo](/images/anotherland-logo.jpg)

## Introduction
Welcome to the **Anotherland Server Emulator** project, a community-driven initiative to resurrect the MMORPG, *Otherland*. This emulator aims to recreate the experience of *Otherland* by providing tools and instructions to set up and run your own server for the game.

Currently, the project is undergoing a major rewrite and is not yet playable. If you simply want to run a local instance of the server,  
you should check out the tag `v0.1.1` and follow the README there, or use the pre-compiled Docker image with the same name.

## Prerequisites
Before you begin, ensure you have the following installed:
- **Rust 1.85.0-nightly**
- A legally acquired copy of the game (*Otherland Next*)
- **MongoDB**

## Compilation
Clone the repository to your local machine:
```bash
git clone https://github.com/AnotherlandServer/anotherland.git
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
  
- **`world_service`** *(not yet implemented)*  
  Responsible for running a collection of zones (maps) defined by the database.
  
- **`dungeon_service`** *(not yet implemented)*  
  Responsible for running dungeon instances.
  
- **`myland_service`** *(not yet implemented)*  
  Responsible for running Myland instances.
  
- **`emergency_service`** *(not yet implemented)*  
  Responsible for handling emergencies.
  
- **`battleground_service`** *(not yet implemented)*  
  Responsible for running battlegrounds.
  
- **`frontend_server`**  
  A staging area for clients immediately after connecting to a realm. It serves clients until they’ve selected a character to play as, at which point they are redirected to one of the `cluster_server` endpoints.
  
- **`cluster_server`** *(not yet implemented)*  
  Routes connected clients’ requests to the appropriate zone or dungeon server.

## Running the Server
Run each process at least once. You can use the `--help` argument with each process to view available options and their default values.  
When specifying public addresses (e.g., for the `frontend_server`), avoid using `127.0.0.1`, as the *Otherland* client may struggle to resolve it.

## Contribution
Currently, we are not accepting contributions as Anotherland is in its initial development phase. Our immediate goal is to build a stable foundation for the project. We appreciate your interest and enthusiasm and intend to open the project for community contributions in the future.

## License
This project is licensed under the [AGPL-3.0 License](LICENSE).

## Disclaimer
This project is a fan-based initiative and is not officially affiliated with, endorsed by, or connected to any of the original creators or entities involved in the development of *Otherland*, including Game OL GmbH, DRAGO Entertainment S.A., or Tad Williams. This emulator is developed and maintained by enthusiasts with no commercial intent and respects the intellectual property rights of the original creators.
