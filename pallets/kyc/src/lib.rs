// Add required imports and dependencies
pub use pallet::*;

#[cfg(test)]
mod mock;

#[frame_support::pallet]
pub mod pallet {

	// use frame_benchmarking::runtime_decl_for_benchmark::OptionQuery;
	use frame_support::{pallet_prelude::*, traits::Hash, Blake2_128Concat};
	use frame_system::pallet_prelude::{OriginFor, *};

	// Declare the pallet type
	// This is a placeholder to implement traits and methods.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/* ------------------------------ Addn Structs ------------------------------ */
	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct KycData<T: Config> {
		metadata: MetaData<T>,
		bulkdata: BulkData,
	}
	impl<T: Config> KycData<T>{
		fn new(meta: MetaData<T>, bulk: BulkData)->Self{}
	}

	/// For Personal details which will be uploaded to blockchain
	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct MetaData<T: Config> {
		name: BoundedVec<u8, ConstU32<100>>,
		age: u32,
		on_chain_address: T::AccountId,
		aadhar_number: u32,
	}
	impl<T: Config> MetaData<T> {
		fn new(
			name: BoundedVec<u8, ConstU32<100>>,
			age: u32,
			on_chain_address: T::AccountId,
			aadhar_number: u32,
		) -> Self {
			MetaData { name, age, on_chain_address, aadhar_number }
		}
	}

	/// For documents, they gets uploaded to any decentralised data storage like IPFS
	/// Have to store hashes for documents
	pub struct BulkData {
		birth_certificate: bool,
		pan_card: bool,
		driver_license: bool,
		ipfc_data: Hash,
	}
	impl BulkData{
		fn new(){}
	}

	pub enum KycProvider {
		IPFS
	}
	trait DecentStorage{
	}


	/* ------------------------------ Pallet Config ----------------------------- */
	// Add the runtime configuration trait
	// All types and constants go here.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/* --------------------------------- Storage -------------------------------- */
	#[pallet::storage]
	#[pallet::getter(fn kyc_data)]
	pub type KycDataForUser<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, KycData<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn oracle_list)]
	pub type OracleList<T: Config> = StorageValue<_, KycProvider>;

	/* --------------------------------- Events --------------------------------- */
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		UserRegistered,
		UserReqKycVerify,
		KycVerififedByOracles,
		ChangeOracleList,
		KYCVerificationRequest { data: KycData, provider: KycProvider},
		IpfsHashRecieved,
		NoEvent,
	}

	/* ---------------------------------- Hooks --------------------------------- */
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/* -------------------------------- EXtrinics ------------------------------- */
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn store_data(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn update_data(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn revoke_data(origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

/* -------------------------------------------------------------------------- */
/* TEST */
/* -------------------------------------------------------------------------- */

// Test pallet log in a mock runtime
// Test storage in a mock runtime

// Test events in a mock runtime
// Assuming you use the default generation of deposit_event with the generate_deposit macro,
// all pallet events are stored under the system / events key with some extra information as an
// EventRecord.

#[cfg(test)]
mod test {

	use crate::mock::*;

	fn kyc_verification() {
		/// This data will be stored in the database and  also a transaction will be
		/// recorded on-chain stating that XYZ(user) submitted a request to get KYC verified .
		fn kyc_store_verify() {
			new_test_ext().execute_with(|| {})
		}
		/// KYC providers will fetch the data for KYC verification.
		fn oracle_fetch() {
			new_test_ext().execute_with(|| {})
		}
		/// After doing the KYC verification they submit the result and related data to KYC
		/// distributed oracle.
		fn oracle_push() {
			new_test_ext().execute_with(|| {})
		}
		/// The KYC dApp will push the data onto the IPFS cluster.
		fn data_push_ipfs() {}
		/// Receive the IPFS hash of the above data and the dApp will push the  metadata of the user
		/// onto the blockchain
		fn push_metadata_bulkdata_hash() {}
	}

	fn kyc_update() {}

	fn other_user_data() {}
}
