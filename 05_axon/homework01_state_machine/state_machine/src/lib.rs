use std::thread::sleep;
// use core::borrow::Borrow;
use std::time::Duration;
// use std::cmp::Ordering::Greater;
use rand::Rng;

#[derive(Debug)]
struct Block {
    height: i128,
    random_number: i64
}
impl Block {
    fn new() -> Self {
        Block { height: 0, random_number: 0 }
    }

    fn mock_consensus(&mut self) {
        self.random_number = rand::thread_rng().gen_range(0, 10);
    }

    fn update_height(&mut self) {
        self.height += 1;
    }

    fn height(self) -> i128{
        self.height
    }

    fn new_height(&mut self) {
        self.height += 1;
        println!("block height increase : {}", self.height);
    }

    fn valid_block(&mut self) -> bool {
        self.mock_consensus();
        if self.random_number>5 {
            true
        } else {
            false
        }
    }

    fn is_consensus(&mut self) -> bool {
        self.mock_consensus();
        if self.random_number>3 {
            true
        }else {
            false
        }
    }
}

#[derive(Debug)]
struct Commit {
    wait_time: Duration
}

impl Commit {
    fn new() -> Self {
        Commit{
            wait_time: Duration::new(6, 0)
        }
    }
}

#[derive(Debug)]
struct NewRound {
    wait_time: Duration
}

impl NewRound {
    fn new() -> Self {
        NewRound{
            wait_time: Duration::new(1, 0)
        }
    }
}

#[derive(Debug)]
struct Propose {
    wait_time: std::time::Duration
}

impl Propose {
    fn new() -> Self {
        Propose{
            wait_time: Duration::new(6, 0)
        }
    }
}

#[derive(Debug)]
struct Prevote {
    wait_time: std::time::Duration
}

impl Prevote {
    fn new() -> Self {
        Prevote{
            wait_time: Duration::new(6, 0)
        }
    }
}

#[derive(Debug)]
struct PrevoteNil {
    wait_time: std::time::Duration
}

impl PrevoteNil {
    fn new() -> Self {
        PrevoteNil{
            wait_time: Duration::new(1, 0)
        }
    }
}

#[derive(Debug)]
struct Precommit {
    wait_time: std::time::Duration
}

impl Precommit {
    fn new() -> Self {
        Precommit{
            wait_time: Duration::new(6, 0)
        }
    }
}

#[derive(Debug)]
struct PrecommitNil {
    wait_time: std::time::Duration
}

impl PrecommitNil {
    fn new() -> Self {
        PrecommitNil{
            wait_time: Duration::new(1, 0)
        }
    }
}

#[derive(Debug)]
struct ConsensusState<T> {
    state: T,
}

impl ConsensusState<Propose> {
    fn new() -> Self {
        ConsensusState {
            state: Propose::new(),
        }
    }
}

impl From<ConsensusState<Propose>> for ConsensusState<Prevote> {
    fn from(propose: ConsensusState<Propose>) -> ConsensusState<Prevote> {
        println!("last state is {:?}", propose);
        sleep(propose.state.wait_time);
        ConsensusState {
            state: Prevote::new(),
        }
    }
}

impl From<ConsensusState<Propose>> for ConsensusState<PrevoteNil> {
    fn from(propose: ConsensusState<Propose>) -> ConsensusState<PrevoteNil> {
        println!("last state is {:?}", propose);
        sleep(propose.state.wait_time);
        ConsensusState {
            state: PrevoteNil::new(),
        }
    }
}

impl From<ConsensusState<Prevote>> for ConsensusState<Precommit> {
    fn from(precommit: ConsensusState<Prevote>) -> ConsensusState<Precommit> {
        println!("last state is {:?}", precommit);
        sleep(precommit.state.wait_time);
        ConsensusState {
            state: Precommit::new(),
        }
    }
}

impl From<ConsensusState<PrevoteNil>> for ConsensusState<PrecommitNil> {
    fn from(prevote_nil: ConsensusState<PrevoteNil>) -> ConsensusState<PrecommitNil> {
        println!("last state is {:?}", prevote_nil);
        sleep(prevote_nil.state.wait_time);
        ConsensusState {
            state: PrecommitNil::new(),
        }
    }
}

impl From<ConsensusState<Prevote>> for ConsensusState<PrecommitNil> {
    fn from(prevote: ConsensusState<Prevote>) -> ConsensusState<PrecommitNil> {
        println!("last state is {:?}", prevote);
        sleep(prevote.state.wait_time);
        ConsensusState {
            state: PrecommitNil::new(),
        }
    }
}

impl From<ConsensusState<PrecommitNil>> for ConsensusState<NewRound> {
    fn from(new_height_or_round: ConsensusState<PrecommitNil>) -> ConsensusState<NewRound> {
        println!("last state is {:?}", new_height_or_round);
        sleep(new_height_or_round.state.wait_time);
        ConsensusState {
            state: NewRound::new(),
        }
    }
}

impl From<ConsensusState<Precommit>> for ConsensusState<NewRound> {
    fn from(new_height_or_round: ConsensusState<Precommit>) -> ConsensusState<NewRound> {
        println!("last state is {:?}", new_height_or_round);
        sleep(new_height_or_round.state.wait_time);
        ConsensusState {
            state: NewRound::new(),
        }
    }
}

impl From<ConsensusState<Precommit>> for ConsensusState<Commit> {
    fn from(new_height_or_round: ConsensusState<Precommit>) -> ConsensusState<Commit> {
        println!("last state is {:?}", new_height_or_round);
        sleep(new_height_or_round.state.wait_time);
        ConsensusState {
            state: Commit::new(),
        }
    }
}

impl From<ConsensusState<Commit>> for ConsensusState<Propose> {
    fn from(new_height: ConsensusState<Commit>) -> ConsensusState<Propose> {
        println!("last state is {:?}", new_height);
        sleep(new_height.state.wait_time);
        ConsensusState {
            state: Propose::new(),
        }
    }
}


impl From<ConsensusState<NewRound>> for ConsensusState<Propose> {
    fn from(new_round: ConsensusState<NewRound>) -> ConsensusState<Propose> {
        println!("last state is {:?}", new_round);
        sleep(new_round.state.wait_time);
        ConsensusState {
            state: Propose::new(),
        }
    }
}

enum ConsensusStateWrapper {
    Commit(ConsensusState<Commit>),
    NewRound(ConsensusState<NewRound>),
    Propose(ConsensusState<Propose>),
    Prevote(ConsensusState<Prevote>),
    PrevoteNil(ConsensusState<Prevote>),
    Precommit(ConsensusState<Precommit>),
    PrecommitNil(ConsensusState<PrecommitNil>),
}

impl ConsensusStateWrapper {
    fn new() -> Self {
        ConsensusStateWrapper::Propose(ConsensusState::new())
    }
    fn step(mut self, block :&mut Block) -> Self {
        match self {
            ConsensusStateWrapper::Propose(propose) => {
                if block.valid_block() {ConsensusStateWrapper::Prevote(propose.into())}
                else {
                    ConsensusStateWrapper::PrevoteNil(propose.into())
                }
                },
            ConsensusStateWrapper::Prevote(prevote) => {
                if block.is_consensus() {ConsensusStateWrapper::Precommit(prevote.into())}
                else {
                    ConsensusStateWrapper::PrecommitNil(prevote.into())
                }
                },
            ConsensusStateWrapper::PrevoteNil(prevote_nil) => ConsensusStateWrapper::PrecommitNil(prevote_nil.into()),
            ConsensusStateWrapper::Precommit(precommit) => {
                if block.is_consensus() {ConsensusStateWrapper::Commit(precommit.into())}
                else {
                    ConsensusStateWrapper::NewRound(precommit.into())
                }
                },
            ConsensusStateWrapper::PrecommitNil(precommit_nil) => ConsensusStateWrapper::NewRound(precommit_nil.into()),
            ConsensusStateWrapper::Commit(newheight) => {
                // add height
                block.new_height();
                ConsensusStateWrapper::Propose(newheight.into())},
            ConsensusStateWrapper::NewRound(newround) => ConsensusStateWrapper::Propose(newround.into()),
        }
    }
}

fn main() {
    let mut state_machine = ConsensusStateWrapper::new();
    let mut chain = Block::new();
    loop {
        state_machine = state_machine.step(&mut chain);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
    // cargo test --package state_machine --lib -- tests::it_works --exact --nocapture

}
