contract;

enum Error {
    EarlierTimestamp: (u64, u64),
}

abi MyContract {
    #[storage(read)]
    fn get_timestamp() -> u64;

    #[storage(read)]
    fn current_timestamp_later_than(timestamp: u64) -> bool;
}

impl MyContract for Contract {
    #[storage(read)]
    fn get_timestamp() -> u64 {
        std::block::timestamp()
    }

    #[storage(read)]
    fn current_timestamp_later_than(timestamp: u64) -> bool {
        let block_timestamp = std::block::timestamp();
        require(false, Error::EarlierTimestamp((timestamp, block_timestamp)));
        true
    }
}
