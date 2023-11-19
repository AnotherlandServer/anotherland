# Anotherland

## Introduction
Welcome to the Anotherland Server Emulator project, a community-driven initiative to resurrect the MMORPG, Otherland. This emulator aims to recreate the experience of Otherland by providing tools and instructions to set up and run your own server for the game.

## Prerequisites
Before you begin, ensure you have the following installed:
- A legally acquired copy of the game (Otherland)
- MongoDB
- RabbitMQ (only required for distributed execution)

## Installation
Clone the repository to your local machine:
```bash
git clone https://github.com/AnotherlandServer/anotherland.git
```

## Usage
The server can be run as a single process using the `standalone-server` command or distributed across multiple processes.

### Basic Command Structure
```bash
anotherland [OPTIONS] --external-ip <EXTERNAL_IP> --mongo-uri <MONGO_URI> <COMMAND>
```

### Commands
- `init-db` - Initialize the database.
- `data-import` - Import game data from the client.
- `login-server` - Start the login server.
- `realm-server` - Start a realm server.
- `frontend-server` - Start the frontend server.
- `node-server` - Start a node server.
- `instance-pool-server` - Start an instance pool server.
- `api-server` - Start the API server.
- `standalone-server` - Run all components in a single process.
- `help` - Display help information.

### Options
- `--external-ip <EXTERNAL_IP>` - Set the external IP address.
- `--mongo-uri <MONGO_URI>` - MongoDB connection URI.
- `--mongo-cluster-db <MONGO_CLUSTER_DB>` - MongoDB cluster database name (default: `anotherland`).

Additional options are available for each command. Use the `--help` option with any command to see more details.

## Configuration
Configure the server by setting the necessary environment variables. Examples include:
- `EXTERNAL_IP=192.168.178.45`
- `MONGO_URI=mongodb://localhost:27017`
- `MONGO_CLUSTER_DB=anotherland`
- `MAX_ACTIVE_SESSIONS=10`

## Running the Server
To run the server in standalone mode, use the following command:
```bash
anotherland --external-ip <EXTERNAL_IP> --mongo-uri <MONGO_URI> standalone-server
```

For distributed execution, start each server component separately using the relevant command.

## Contribution
Currently, we are not accepting contributions as Anotherland is in its initial development phase. Our immediate goal is to build a stable foundation for the project. We appreciate your interest and enthusiasm, and we intend to open the project for community contributions in the future.

## License
This project is licensed under the [AGPL-3.0 License](LICENSE).

## Disclaimer
This project is a fan-based initiative and is not officially affiliated with, endorsed by, or connected to any of the original creators or entities involved in the development of Otherland, including Game OL GmbH, DRAGO Entertainment S.A., or Tad Williams. This emulator is developed and maintained by enthusiasts with no commercial intent and respects the intellectual property rights of the original creators.
