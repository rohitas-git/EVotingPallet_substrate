License: Unlicense

# Pallet Documentation
This pallet is used for conducting elections. It provides functionality for registering voters and candidates, casting votes, and retrieving election results. It defines a simple voting system where users can register as voters and candidates, and vote for candidates within a configured election period. The winner is the candidate with the most votes.

## Dependencies
This pallet depends on frame_support::pallet_prelude, frame_system::pallet_prelude, frame_support::pallet, frame_support::StorageMap, frame_support::BoundedVec, frame_support::StorageValue, and frame_system::Config.

## Usage
Pallet Configuration
To configure the pallet, use the Config trait, which depends on frame_system::Config. It also requires type RuntimeEvent, which is an event type that can be used to generate events from this pallet.

rust
Copy code
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}
## Voter and Candidate Information
This pallet contains two structs: VoterInfo and CandidateInfo. 

VoterInfo contains the following fields:

vote_status: a boolean indicating whether or not the voter has cast a vote
voted_for: an optional T::AccountId indicating the candidate the voter has voted for, or None if the voter has not voted

CandidateInfo contains the following fields:

vote_count: the number of votes received by the candidate

## Election Information
This pallet also contains a struct called ElectionInfo which contains the following fields:

start_block: an optional T::BlockNumber indicating the block number at which the election will start
end_block: an optional T::BlockNumber indicating the block number at which the election will end
## Storage
This pallet uses the following storage items:

AccountToVoterInfo: a map from T::AccountId to VoterInfo<T>
AccountToCandidateInfo: a map from T::AccountId to CandidateInfo
ElectionConfig: a storage value of type ElectionInfo<T> representing the current state of the election
MaxVoteCandidate: a bounded vector containing the account IDs of the candidates who received the maximum number of votes
MaxVote: a storage value of type u32 representing the maximum number of votes received by any candidate
## Events
This pallet provides the following events:

RegisterVoter: emitted when a voter is successfully registered
RegisterCandidate: emitted when a candidate is successfully registered
VoteSuccess: emitted when a vote is successfully cast
RecieveVoteCount: emitted when the pallet receives a request to retrieve the vote count
ElectionConfigured: emitted when the election is successfully configured
WinnerVecStored: emitted when the winner vector is successfully stored
## Errors
This pallet provides the following errors:

AlreadyVoted: returned when a voter attempts to cast multiple votes
AlreadyRegistered: returned when a voter or candidate attempts to register multiple times
AlreadyConfiguredElection: returned when an election is already configured
NotRegistered: returned when a voter or candidate attempts to vote or perform other actions before registering
ElectionNotConfigured: returned when an action requiring the election to be configured is performed before configuration
ElectionNotStarted: returned when an action requiring the election to have started is performed before the start block
ElectionEnded: returned when an action requiring the election to be ongoing is performed after the end block
ElectionNotEnded: returned when an action requiring the election to have ended is performed before the end
