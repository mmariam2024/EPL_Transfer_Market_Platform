#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Player {
    id: u64,
    name: String,
    position: String,
    current_club: String,
    market_value: u64,
    transfer_status: String, // e.g., "available", "transferred"
    contract_until: u64,
    age: u32,
    nationality: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Transfer {
    id: u64,
    player_id: u64,
    from_club: String,
    to_club: String,
    transfer_fee: u64,
    transfer_date: u64,
    contract_duration: u64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TransferBid {
    id: u64,
    player_id: u64,
    from_club: String,
    to_club: String,
    bid_amount: u64,
    bid_status: String, // e.g., "pending", "accepted", "rejected"
    created_at: u64,
}

impl Storable for Player {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Player {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Transfer {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Transfer {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for TransferBid {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TransferBid {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PLAYER_STORAGE: RefCell<StableBTreeMap<u64, Player, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TRANSFER_STORAGE: RefCell<StableBTreeMap<u64, Transfer, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static TRANSFER_BID_STORAGE: RefCell<StableBTreeMap<u64, TransferBid, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct PlayerPayload {
    name: String,
    position: String,
    current_club: String,
    market_value: u64,
    contract_until: u64,
    age: u32,
    nationality: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct TransferPayload {
    player_id: u64,
    from_club: String,
    to_club: String,
    transfer_fee: u64,
    transfer_date: u64,
    contract_duration: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct TransferBidPayload {
    player_id: u64,
    from_club: String,
    to_club: String,
    bid_amount: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

#[ic_cdk::update]
fn create_player(payload: PlayerPayload) -> Result<Player, Message> {
    if payload.name.is_empty()
        || payload.position.is_empty()
        || payload.current_club.is_empty()
        || payload.market_value == 0
    {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'position', 'current_club', and 'market_value' are provided."
                .to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let player = Player {
        id,
        name: payload.name,
        position: payload.position,
        current_club: payload.current_club,
        market_value: payload.market_value,
        transfer_status: "available".to_string(),
        contract_until: payload.contract_until,
        age: payload.age,
        nationality: payload.nationality,
        created_at: current_time(),
    };
    PLAYER_STORAGE.with(|storage| storage.borrow_mut().insert(id, player.clone()));
    Ok(player)
}

#[ic_cdk::query]
fn get_players() -> Result<Vec<Player>, Message> {
    PLAYER_STORAGE.with(|storage| {
        let players: Vec<Player> = storage
            .borrow()
            .iter()
            .map(|(_, player)| player.clone())
            .collect();

        if players.is_empty() {
            Err(Message::NotFound("No players found".to_string()))
        } else {
            Ok(players)
        }
    })
}

#[ic_cdk::query]
fn get_player_by_id(id: u64) -> Result<Player, Message> {
    PLAYER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, player)| player.id == id)
            .map(|(_, player)| player.clone())
            .ok_or(Message::NotFound("Player not found".to_string()))
    })
}

// Function to retrieve players by their club names
#[ic_cdk::query]
fn get_players_by_club(club_name: String) -> Result<Vec<Player>, Message> {
    PLAYER_STORAGE.with(|storage| {
        let players: Vec<Player> = storage
            .borrow()
            .iter()
            .filter(|(_, player)| player.current_club == club_name)
            .map(|(_, player)| player.clone())
            .collect();

        if players.is_empty() {
            Err(Message::NotFound("No players found".to_string()))
        } else {
            Ok(players)
        }
    })
}

#[ic_cdk::update]
fn create_transfer(payload: TransferPayload) -> Result<Transfer, Message> {
    // Validate the payload
    if payload.transfer_fee == 0 || payload.contract_duration == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'transfer_fee' and 'contract_duration' are provided.".to_string(),
        ));
    }

    // The player should not be transferred to the same club
    if payload.from_club == payload.to_club {
        return Err(Message::Error(
            "Player cannot be transferred to the same club.".to_string(),
        ));
    }

    // The player must exist and must not have been transferred
    let player = PLAYER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, player)| player.id == payload.player_id)
            .map(|(_, player)| player.clone())
    });

    if player.is_none() {
        return Err(Message::NotFound("Player not found".to_string()));
    }

    let player = player.unwrap();

    // Ensure the player is available for transfer and is a member of the from_club
    if player.transfer_status != "available" || player.current_club != payload.from_club {
        return Err(Message::Error(
            "Player is not available for transfer or is not a member of the from_club.".to_string(),
        ));
    }

    // The player must not have been transferred
    if player.transfer_status == "transferred" {
        return Err(Message::Error(
            "Player has already been transferred.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let transfer = Transfer {
        id,
        player_id: payload.player_id,
        from_club: payload.from_club.clone(),
        to_club: payload.to_club.clone(),
        transfer_fee: payload.transfer_fee,
        transfer_date: payload.transfer_date,
        contract_duration: payload.contract_duration,
        created_at: current_time(),
    };

    TRANSFER_STORAGE.with(|storage| storage.borrow_mut().insert(id, transfer.clone()));

    // Update player status to transferred and update current club and contract until
    let mut updated_player = player.clone();
    updated_player.transfer_status = "transferred".to_string();
    updated_player.current_club = payload.to_club.clone();
    updated_player.contract_until = payload.transfer_date + payload.contract_duration;
    PLAYER_STORAGE.with(|storage| storage.borrow_mut().insert(player.id, updated_player));

    Ok(transfer)
}

#[ic_cdk::query]
fn get_transfers() -> Result<Vec<Transfer>, Message> {
    TRANSFER_STORAGE.with(|storage| {
        let transfers: Vec<Transfer> = storage
            .borrow()
            .iter()
            .map(|(_, transfer)| transfer.clone())
            .collect();

        if transfers.is_empty() {
            Err(Message::NotFound("No transfers found".to_string()))
        } else {
            Ok(transfers)
        }
    })
}

#[ic_cdk::query]
fn get_transfer_by_id(id: u64) -> Result<Transfer, Message> {
    TRANSFER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, transfer)| transfer.id == id)
            .map(|(_, transfer)| transfer.clone())
            .ok_or(Message::NotFound("Transfer not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_transfer_bid(payload: TransferBidPayload) -> Result<TransferBid, Message> {
    if payload.bid_amount == 0 {
        return Err(Message::InvalidPayload(
            "Bid amount must be greater than 0.".to_string(),
        ));
    }

    let player = PLAYER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, player)| player.id == payload.player_id)
            .map(|(_, player)| player.clone())
    });

    if player.is_none() {
        return Err(Message::NotFound("Player not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let bid = TransferBid {
        id,
        player_id: payload.player_id,
        from_club: payload.from_club.clone(),
        to_club: payload.to_club.clone(),
        bid_amount: payload.bid_amount,
        bid_status: "pending".to_string(),
        created_at: current_time(),
    };

    TRANSFER_BID_STORAGE.with(|storage| storage.borrow_mut().insert(id, bid.clone()));
    Ok(bid)
}

#[ic_cdk::query]
fn get_transfer_bids() -> Result<Vec<TransferBid>, Message> {
    TRANSFER_BID_STORAGE.with(|storage| {
        let bids: Vec<TransferBid> = storage
            .borrow()
            .iter()
            .map(|(_, bid)| bid.clone())
            .collect();

        if bids.is_empty() {
            Err(Message::NotFound("No transfer bids found".to_string()))
        } else {
            Ok(bids)
        }
    })
}

#[ic_cdk::query]
fn get_transfer_bid_by_id(id: u64) -> Result<TransferBid, Message> {
    TRANSFER_BID_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, bid)| bid.id == id)
            .map(|(_, bid)| bid.clone())
            .ok_or(Message::NotFound("Transfer bid not found".to_string()))
    })
}

#[ic_cdk::update]
fn accept_transfer_bid(id: u64) -> Result<Message, Message> {
    let bid = TRANSFER_BID_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, bid)| bid.id == id)
            .map(|(_, bid)| bid.clone())
    });

    if bid.is_none() {
        return Err(Message::NotFound("Transfer bid not found".to_string()));
    }

    let mut bid = bid.unwrap();

    if bid.bid_status != "pending" {
        return Err(Message::Error(
            "Only pending bids can be accepted.".to_string(),
        ));
    }

    let player = PLAYER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, player)| player.id == bid.player_id)
            .map(|(_, player)| player.clone())
    });

    if player.is_none() {
        return Err(Message::NotFound("Player not found".to_string()));
    }

    let player = player.unwrap();

    // Update bid status to accepted
    bid.bid_status = "accepted".to_string();
    TRANSFER_BID_STORAGE.with(|storage| storage.borrow_mut().insert(id, bid.clone()));

    // Create transfer
    let transfer_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let transfer = Transfer {
        id: transfer_id,
        player_id: bid.player_id,
        from_club: bid.from_club.clone(),
        to_club: bid.to_club.clone(),
        transfer_fee: bid.bid_amount,
        transfer_date: current_time(),
        contract_duration: player.contract_until - current_time(), // Assume remaining contract duration
        created_at: current_time(),
    };

    TRANSFER_STORAGE.with(|storage| storage.borrow_mut().insert(transfer_id, transfer.clone()));

    // Update player status to transferred and update current club and contract until
    let mut updated_player = player.clone();
    updated_player.transfer_status = "transferred".to_string();
    updated_player.current_club = bid.to_club.clone();
    updated_player.contract_until = current_time() + transfer.contract_duration;
    PLAYER_STORAGE.with(|storage| storage.borrow_mut().insert(player.id, updated_player));

    Ok(Message::Success(
        "Transfer bid accepted and player transferred.".to_string(),
    ))
}

#[ic_cdk::update]
fn reject_transfer_bid(id: u64) -> Result<Message, Message> {
    let bid = TRANSFER_BID_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, bid)| bid.id == id)
            .map(|(_, bid)| bid.clone())
    });

    if bid.is_none() {
        return Err(Message::NotFound("Transfer bid not found".to_string()));
    }

    let mut bid = bid.unwrap();

    if bid.bid_status != "pending" {
        return Err(Message::Error(
            "Only pending bids can be rejected.".to_string(),
        ));
    }
    
    // Update bid status to rejected
    bid.bid_status = "rejected".to_string();
    TRANSFER_BID_STORAGE.with(|storage| storage.borrow_mut().insert(id, bid.clone()));

    Ok(Message::Success("Transfer bid rejected.".to_string()))
}

fn current_time() -> u64 {
    time()
}

ic_cdk::export_candid!();
