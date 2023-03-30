use frame_system::Config;
use codec::{Encode,Decode};
use scale_info::TypeInfo;

// #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, Debug)]
#[derive(Debug,Decode, Encode, Clone, PartialEq, Default, TypeInfo)]
pub struct CandidateInfo<T:Config>{
    name: String,
    votes_count: u64,
    who: T::AccountId
}

pub trait Candidate<T:Config>{
    fn new(name:String, id: T::AccountId)->Self;

    fn vote_count(&self);

    fn increase_vote(&mut self);
}

impl<T:Config> CandidateInfo<T>{
    pub fn new_candidate(name:String, id: T::AccountId)->Self{
        CandidateInfo { name: name, votes_count: 0u64, who: id}
    }
}
impl<T:Config> Candidate<T> for CandidateInfo<T>{
    fn new(name:String, id: T::AccountId)->Self{
        CandidateInfo::new_candidate(name,id)
    }

    fn vote_count(&self){
        println!("Number of Votes for {}: {}", self.name, self.votes_count);
    }

    fn increase_vote(&mut self){self.votes_count+=1;}
}