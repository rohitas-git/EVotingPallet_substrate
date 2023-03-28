// *#[pallet]
// 
    // *The #[pallet] macro is required to declare a pallet. 
    // This attribute macro is an attribute of the pallet module (mod pallet). 
    // 
    // *Within the pallet module, the #[pallet] macro serves as an entry point for additional #[pallet::*] macros 
    // *that describe the attributes used to identify the specific items the pallet requires. 
    // 
        // #[pallet]
        // pub mod pallet {
        //     ...
        // }
    // 
    // For example, a pallet typically includes a set of types, functions, and trait implementations 
    // that are aggregated by the construct_runtime! macro to build the runtime.

    /* ---------------------------- Development mode ---------------------------- */
    // #[pallet(dev_mode)] or #[frame_support::pallet(dev_mode)] 
    // to enable development mode for the pallet you're working on.

    // Development mode loosens some of the restrictions and requirements placed on production pallets to make it easier
    //  to iterate on your code during development and testing cycles. 
    
    // For example, if you enable development mode for a pallet:
    // 
    // You don't need to specify a weight on every #[pallet::call] declaration. 
    // By default, development mode assigns a weight of zero (0) to calls that don't have a weight explicitly specified.
    // 
    // You don't need to implement MaxEncodedLen on storage types. 
    // By default, development mode marks all storage items as unbounded.

    /* ---------------------------- Using the Pallet ---------------------------- */
    // Inside the module, the macro parses items with the attribute #[pallet::*]. 
    // Some #[pallet::*] attributes are mandatory and some are optional.
    // 
    // You can import system-level types from the frame_support and frame_system crates automatically 
    // by using the pallet_prelude from those crates. 
    // 
    // *The #[pallet] macro is similar to a derive macro in that 
    // *it expands the pallet types and trait implementations by reading the input.
    // 
    // In most cases, the macro doesn't modify any input. 
    // However, there are a few specific scenarios where—unlike a derive macro—this macro modifies its input.
    // The macro will modify the input in the following circumstances:
    // 1. If a generic is replaced with a type
    // 2. If a function or data structure is changed
    // 3. If docs are not provided by the user

// *#[pallet::config]
// 
    // *The #[pallet::config] macro is required to define the generic data types that the pallet uses.
    // This macro provides the constants that are part of the system-level Config trait for the pallet.
    // 
        // #[pallet::config]
        // pub trait Config: frame_system::Config + $optionally_some_other_supertraits
        // $optional_where_clause {}
        // 
        // 
        // #[pallet::config]
        // #[pallet::disable_frame_system_supertrait_check]
        // pub trait Config: pallet_timestamp::Config {}
    
// *#[pallet::constant]
// This macro adds information about the constants used in a pallet to the runtime metadata, including:
    // the constant name
    // the name of the associated types
    // the constant value
    // the value returned by Get::get() for the constant
    // 
        // #[pallet::config]
        // pub trait Config: frame_system::Config {
        //     #[pallet::constant] // puts attributes in metadata
        //     type MyGetParam: Get<u32>;
        // }

// *#[pallet::extra_constants]
// The #[pallet::extra_constants] macro enables you to add constants to the metadata.
//
// For example, you can declare a function that returns a generated value. 
// You can then use the #[pallet::extra_constants] macro to add the information for the generated value to the metadata:
// 
// #[pallet::extra_constants]
// impl<T: Config> Pallet<T> {
//   //Example function using extra_constants
//   fn example_extra_constants() -> u128 { 4u128 }
// }

// *#[pallet::pallet]
// *required to declare the pallet data structure placeholder 
// *to be used by construct_runtime! macro. 
// 
// This macro must be defined as a struct named Pallet with a generic type and no where clause.
// #[pallet::pallet]
// pub struct Pallet<T>(_);
// 
// *This macro can generate the Store trait to contain an associated type for each storage item 
// if you provide the #[pallet::generate_store($vis trait Store)] attribute macro.
// 
// #[pallet::pallet]
// #[pallet::generate_store(pub(super) trait Store)]
// pub struct Pallet<T>(_);

// *#[pallet::without_storage_info]
// *enables you to define pallet storage items that don't have a fixed size.
//
// By default, all pallet storage items are required to implement traits::StorageInfoTrait, 
// so that all key and value types have a fixed size based on the bound defined 
// in the pallet_prelude::MaxEncodedLen attribute. 
// This size limitation is required for parachain development to estimate the size of the Proof of Validity (PoV) blob.
//
// The #[pallet::without_storage_info] attribute macro allows you to override the default behavior if you require 
// unbounded storage for an entire pallet. 
// To use it, add the #[pallet::without_storage_info] attribute to the pallet struct like so:
// #[pallet::pallet]
// #[pallet::generate_store(pub(super) trait Store)]
// #[pallet::without_storage_info]
// pub struct Pallet<T>(_);
// 
// Note that you should only use the #[pallet::without_storage_info] macro if 
// you need to make all of the storage items in your pallet unbounded.
// 
// Because the #[pallet::without_storage_info] macro applies to all storage items in your pallet, you should only use it in a test or development environment. 
// You should never use the #[pallet::without_storage_info] attribute macro in a production environment.

// *#[pallet::unbounded]
// *enables you to declare a specific storage item as unbounded. 
// By default, all pallet storage items are required to have a fixed size. 
// You can use this attribute macro to override the default requirement on a specific storage item. 
// If you are a parachain developer, you can use this macro for storage items 
// that will never go into the Proof of Validity (PoV) blob.

// *#[pallet::hooks]
// *allows you to declare optional pallet hooks to implement pallet-specific logic at specific points in the block making process. 
// Within the #[pallet::hooks] macro, you can implement the Hooks trait to execute logic when 
// a block is being initialized or finalized, 
// before a runtime is upgraded, or after a runtime upgrade has been completed.
// #[pallet::hooks]
// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  // Hooks functions and logic goes here.
// }

// *#[pallet::call]
// *The #[pallet::call] is required to implement the functions that can be dispatched to the runtime for a pallet. 
// 
// Each function must:
// -define a weight with the #[pallet::weight($expr)] attribute
// -have its first argument as origin: OriginFor<T>
// -use compact encoding for arguments using #[pallet::compact]
// -return DispatchResultWithPostInfo or DispatchResult
// 
// Extrinsic requests coming into the runtime can use calls to trigger specific logic. 
// Calls can also be used in on-chain governance, demonstrated by the democracy pallet where calls can be voted on. 
// The #[pallet::call] aggregates all of the function call logic using the Call enum. 
// The aggregation enables FRAME to batch functions of the same type into a single runtime call. 
// The runtime then generates the associated items from the implementation defined in the impl code blocks.
// 

mod pallet{
    // *required to declare a pallet. 
    // This attribute macro is an attribute of the pallet module (mod pallet). 
        
    // *Within the pallet module, the #[pallet] macro serves as an entry point for additional #[pallet::*] macros 
    // *that describe the attributes used to identify the specific items the pallet requires. 
        
    #[pallet]
    pub mod pallet {
        ...
    }
}
mod config{
    // required to define the generic data types that the pallet uses

    #[pallet::config]
    pub trait Config: frame_system::Config + $optionally_some_other_supertraits
    $optional_where_clause {}
    
    
    #[pallet::config]
    #[pallet::disable_frame_system_supertrait_check]
    pub trait Config: pallet_timestamp::Config {}
}
mod constants{
    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant] // puts attributes in metadata
        type MyGetParam: Get<u32>;
    }
}
mod extra_constants{
    #[pallet::extra_constants]
    impl<T: Config> Pallet<T> {
      //Example function using extra_constants
      fn example_extra_constants() -> u128 { 4u128 }
    }
}
mod pallet_pallet{

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);
}
mod hooks{
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
      // Hooks functions and logic goes here.
    }
}
mod call{
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// $some_doc
        #[pallet::weight($ExpressionResultingInWeight)]
        pub fn $fn_name(
            origin: OriginFor<T>,
            $some_arg: $some_type,
            // or with compact attribute: #[pallet::compact] $some_arg: $some_type,
            ...
        ) -> DispatchResultWithPostInfo { // or `-> DispatchResult`
            ...
        }
        ...
    }
}
mod error{
    #[pallet::error]
    pub enum Error<T> {
        /// $some_optional_doc
        $SomeFieldLessVariant,
        /// $some_more_optional_doc
        $SomeVariantWithOneField(FieldType),
        ...
    }
}
mod event{
    #[pallet::event]
    #[pallet::generate_deposit($visibility fn deposit_event)] // Optional
    pub enum Event<$some_generic> $optional_where_clause {
        /// Some doc
        $SomeName($SomeType, $YetanotherType, ...),
        ...
    }
}
mod storage{
    #[pallet::storage]
    #[pallet::getter(fn $getter_name)] // optional
    $vis type $StorageName<$some_generic> $optional_where_clause
        = $StorageType<$generic_name = $some_generics, $other_name = $some_other, ...>;
}
mod type_values{
    // The #[pallet::type_value] macro enables you to define a struct that implements a Get trait for storage types. 
    // This attribute macro can be used multiple times in combination with the #[pallet::storage] macro 
    // to define default values in storage.

    #[pallet::type_value]
    fn MyDefault<T: Config>() -> T::Balance { 3.into() }
}
mod genesis_build{
    // allows you to define how a genesis configuration is built.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }
}
mod genesis_config{
    // allows you to define the genesis configuration of the pallet.
    // The macro can be defined as an enumeration or a struct, 
    // but must be public and implement trait the GenesisBuild with the #[pallet::genesis_build] macro.

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        _myfield: BalanceOf<T>,
    }
}
mod inherent{
    // allows the pallet to provide data in an unsigned inherent transaction.

    #[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        // ... regular trait implementation
    }
}
mod origin{
    // allows you to define an origin for the pallet.
    // The macro must be defined as a type alias, enumeration, or struct. The macro must be public.
    
    #[pallet::origin]
    pub struct Origin<T>(PhantomData<(T)>);
}
mod validate_unsigned{
    // allows the pallet to validate unsigned transactions.

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        // ... regular trait implementation
    }   
}
mod aggregation{
    // aggregation
    // Used in the context of FRAME, aggregation or pallet aggregation is the process of combining analogous types 
    // from multiple runtime modules into a single type. 
    // Pallet aggregation allows each module's analogous types to be represented. 
    // The call containing the aggregated types is sometimes referred to as an outer call or a call to an outer object. 
    
    // Currently, there are six data types that can be aggregated:
    // 
    // Call for published functions that can be called with a set of arguments.
    // Error for messages that indicate why a function invocation (Call) failed.
    // Event for pallet-emitted events that describe state changes.
    // Log for extensible header items.
    // Metadata for information that allows inspection of the above.
    // Origin for the source of a function invocation (Call).
}

mod custom_pallet_skeleton{

    // Most pallets are composed with some combination of the following sections:

    // Imports and dependencies
    // Pallet type declaration
    // Runtime configuration trait
    // Runtime storage
    // Runtime events
    // Hooks for logic that should be executed in a specific context
    // Function calls that can be used to execute transactions
    // For example, if you wanted to define a custom pallet, you might start with a skeleton structure for the pallet similar to the following:

    // Add required imports and dependencies
    pub use pallet::*;

    // Inside the module, the macro parses items with the attribute #[pallet::*]. 
    // Some #[pallet::*] attributes are mandatory and some are optional.
    #[frame_support::pallet]
    pub mod pallet { 
        // Import system-level types from the frame_support and frame_system crates automatically 
        // by using the pallet_prelude from those crates
        use frame_support::pallet_prelude::*;
        use frame_system::pallet_prelude::*;

        // Declare the pallet type
        // This is a placeholder to implement traits and methods.
        #[pallet::pallet]
        #[pallet::generate_store(pub(super) trait Store)]
        pub struct Pallet<T>(_);

        // Add the runtime configuration trait
        // All types and constants go here.
        #[pallet::config]
        pub trait Config: frame_system::Config { ... }

        // Add runtime storage to declare storage items.
        #[pallet::storage]
        #[pallet::getter(fn something)]
        pub type MyStorage<T: Config> = StorageValue<_, u32>;

        // Add runtime events
        #[pallet::event]
        #[pallet::generate_deposit(pub(super) fn deposit_event)]
        pub enum Event<T: Config> { ... }

        // Add hooks to define some logic that should be executed
        // in a specific context, for example on_initialize.
        #[pallet::hooks]
        impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { ... }

        // Add functions that are callable from outside the runtime.
        #[pallet::call]
        impl<T:Config> Pallet<T> { ... }
    }
}

