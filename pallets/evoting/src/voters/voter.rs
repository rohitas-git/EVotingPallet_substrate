use frame_system::Config;

use crate::candidates::Candidate;


pub struct Voter<T:Config>{
    voted: bool,
    who: T::AccountId,
    voted_for: Option<Candidate::<T>>
}

impl<T:Config> Voter<T>{
    pub fn new(id: T::AccountId)-> Self {
        Voter{voted: false, who: id, voted_for: None} 
    }

    pub fn give_vote(&mut self){self.voted= true;}
}