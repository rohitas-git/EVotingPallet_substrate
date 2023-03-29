use frame_system::Config;

pub struct Candidate<T:Config>{
    name: String,
    votes_count: u64,
    who: T::AccountId
}

impl<T:Config> Candidate<T>{
    pub fn new(name:String, id: T::AccountId)->Self{
        Candidate { name: name, votes_count: 0u64, who: id}
    }

    pub fn vote_count(&self){
        println!("Number of Votes for {}: {}", self.name, self.votes_count);
    }
}