use frame_system::Config;

use crate::candidates::CandidateInfo;

#[derive(Debug, Clone)]
pub struct VoterInfo<T:Config>{
    voted: bool,
    who: T::AccountId,
    voted_for: Option<CandidateInfo::<T>>
}

pub trait Voter<T:Config>{
     fn new(id: T::AccountId)-> Self;
     fn voted(&mut self);
     fn check_voted(&self)->bool;
     fn info(&self);
     fn voted_for(&self)->Option<CandidateInfo::<T>>;
}

impl<T:Config> VoterInfo<T>{
    fn new_voter(id: T::AccountId)-> Self {
        VoterInfo{voted: false, who: id, voted_for: None} 
    }
}

impl<T:Config> Voter<T> for VoterInfo<T>{
     fn new(id: T::AccountId)-> Self {
        VoterInfo::<T>::new_voter(id)
    }

     fn voted(&mut self){self.voted= true;}

     fn check_voted(&self)->bool {self.voted}

     fn info(&self){ println!("Vote status:{},\nVoted for {:?}", self.check_voted(), self.voted_for() ); }

     fn voted_for(&self)->Option<CandidateInfo::<T>>{ if self.check_voted() {self.voted_for()} else {Option::None}}
}

// Alternative: struct VoterInfo && trait Voter