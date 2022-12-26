#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

    // 0. Think of Design of pallet 
    // 1. Configure the pallet to emit event
    // 2. Define those event (The defined events indicate that call to pallet has been completed successfully)
    // 3. Implement Errors for pallet in case event fails 
    // 4. Implement a storage mechanism to add claim to the blockchain
    // 5. Implement logic for the call 


#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event] 
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a claim has been created.
        ClaimCreated { who: T::AccountId, claim: T::Hash },
        /// Event emitted when a claim is revoked by the owner.
        ClaimRevoked { who: T::AccountId, claim: T::Hash },
    }

    #[pallet::error]
    pub enum Error<T> {
      /// The claim already exists.
      AlreadyClaimed,
      /// The claim does not exist, so it cannot be revoked.
      NoSuchClaim,
      /// The claim is owned by another account, so caller can't revoke it.
      NotClaimOwner,
    }

    // Create a key-value map, where each claim points to the owner and the block number when the claim was made
    #[pallet::storage]
    pub(super) type Claims<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // Verify that the specified claim has not already been stored.
            ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the claim with the sender and block number.
            Claims::<T>::insert(&claim, (&sender, current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated { who: sender, claim });

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // Get owner of the claim, if none return an error.
            let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotClaimOwner);

            // Remove claim from storage.
            Claims::<T>::remove(&claim);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
            Ok(())
        }
    }

}