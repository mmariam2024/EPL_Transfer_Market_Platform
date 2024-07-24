# EPL Player Transfer Management System

This project implements a system to manage player transfers and transfer bids in the English Premier League (EPL). It provides functionalities to create, retrieve, and manage players, transfers, and transfer bids.

## Features

- **Player Management**:
  - Create new players.
  - Retrieve all players or specific players by their ID.
  - Retrieve players by their current club.

- **Transfer Management**:
  - Create new transfers.
  - Retrieve all transfers or specific transfers by their ID.

- **Transfer Bid Management**:
  - Create new transfer bids.
  - Retrieve all transfer bids or specific transfer bids by their ID.
  - Accept or reject transfer bids.

## Data Structures

### Player
Represents an EPL football player.

Attributes:
- `id`: Unique identifier for the player.
- `name`: Name of the player.
- `position`: Position of the player on the field.
- `current_club`: Club where the player is currently playing.
- `market_value`: Market value of the player.
- `transfer_status`: Transfer status of the player (e.g., "available", "transferred").
- `contract_until`: Contract end date.
- `age`: Age of the player.
- `nationality`: Nationality of the player.
- `created_at`: Timestamp when the player was created.

### Transfer
Represents a transfer of a player from one EPL club to another.

Attributes:
- `id`: Unique identifier for the transfer.
- `player_id`: ID of the transferred player.
- `from_club`: Club from which the player is transferred.
- `to_club`: Club to which the player is transferred.
- `transfer_fee`: Transfer fee paid for the player.
- `transfer_date`: Date of the transfer.
- `contract_duration`: Duration of the contract with the new club.
- `created_at`: Timestamp when the transfer was created.

### Transfer Bid
Represents a bid for transferring a player from one EPL club to another.

Attributes:
- `id`: Unique identifier for the bid.
- `player_id`: ID of the player for whom the bid is made.
- `from_club`: Club making the bid.
- `to_club`: Club receiving the bid.
- `bid_amount`: Amount offered in the bid.
- `bid_status`: Status of the bid (e.g., "pending", "accepted", "rejected").
- `created_at`: Timestamp when the bid was created.

## API Endpoints

### Player Endpoints

- **Create Player**:
  - `create_player(payload: PlayerPayload) -> Result<Player, Message>`
  - Payload: `{name, position, current_club, market_value, contract_until, age, nationality}`

- **Get Players**:
  - `get_players() -> Result<Vec<Player>, Message>`

- **Get Player by ID**:
  - `get_player_by_id(id: u64) -> Result<Player, Message>`

- **Get Players by Club**:
  - `get_players_by_club(club_name: String) -> Result<Vec<Player>, Message>`

### Transfer Endpoints

- **Create Transfer**:
  - `create_transfer(payload: TransferPayload) -> Result<Transfer, Message>`
  - Payload: `{player_id, from_club, to_club, transfer_fee, transfer_date, contract_duration}`

- **Get Transfers**:
  - `get_transfers() -> Result<Vec<Transfer>, Message>`

- **Get Transfer by ID**:
  - `get_transfer_by_id(id: u64) -> Result<Transfer, Message>`

### Transfer Bid Endpoints

- **Create Transfer Bid**:
  - `create_transfer_bid(payload: TransferBidPayload) -> Result<TransferBid, Message>`
  - Payload: `{player_id, from_club, to_club, bid_amount}`

- **Get Transfer Bids**:
  - `get_transfer_bids() -> Result<Vec<TransferBid>, Message>`

- **Get Transfer Bid by ID**:
  - `get_transfer_bid_by_id(id: u64) -> Result<TransferBid, Message>`

- **Accept Transfer Bid**:
  - `accept_transfer_bid(id: u64) -> Result<Message, Message>`

- **Reject Transfer Bid**:
  - `reject_transfer_bid(id: u64) -> Result<Message, Message>`

## Error Handling

The system uses a `Message` enum to represent various success and error messages:

- `Success(String)`: Represents a successful operation with a message.
- `Error(String)`: Represents an error with a message.
- `NotFound(String)`: Represents a "not found" error with a message.
- `InvalidPayload(String)`: Represents an invalid payload error with a message.

## License

This project is licensed under the MIT License.




## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```