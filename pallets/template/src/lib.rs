#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;



#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};
    
	#[cfg(feature = "std")]
	use serde::Serialize;
    #[cfg(feature = "std")]
	use serde::Deserialize;

    // The basis which we buil
    #[pallet::pallet]
    pub struct Pallet<T>(_);



    // Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

        
    // The Gender type used in the `Kitty` struct
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Gender {
        Male,
        Female,
    }

    // Struct for holding Dog information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[scale_info(skip_type_params(T))]
    pub struct Dog<T: Config> {
        // Using 16 bytes to represent a kitty DNA
        pub dna: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    /// Keeps track of the number of dogs in existence.
    #[pallet::storage]
    pub(super) type CountForDogs<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Maps the kitty struct to the kitty DNA.
    #[pallet::storage]
    pub(super) type Dogs<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Dog<T>>;

    /// Track the dogs owned by each account.
    #[pallet::storage]
    pub(super) type DogsOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaxKittiesOwned>,
        ValueQuery,
    >;



    //Kitty 
    // Struct for holding kitty information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        // Using 16 bytes to represent a kitty DNA
        pub dna: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    /// Keeps track of the number of kitties in existence.
    #[pallet::storage]
    pub(super) type CountForKitties<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Maps the kitty struct to the kitty DNA.
    #[pallet::storage]
    pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Kitty<T>>;

    /// Track the kitties owned by each account.
    #[pallet::storage]
    pub(super) type KittiesOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaxKittiesOwned>,
        ValueQuery,
    >;

    // Your Pallet's configuration trait, representing custom external types and interfaces.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the kitties pallet.
        type Currency: Currency<Self::AccountId>;

        /// The maximum amount of kitties a single account can own.
        #[pallet::constant]
        type MaxKittiesOwned: Get<u32>;

        /// The type of Randomness we want to specify for this pallet.
        type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;

        //Dog
        /// The maximum amount of dogs a single account can own.
        #[pallet::constant]
        type MaxDogsOwned: Get<u32>;

        /// The type of Randomness we want to specify for this pallet.
        type DogRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }


	// Our pallet's genesis configuration
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, [u8; 16], Gender)>,
	}

	// Required to implement default for GenesisConfig
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { kitties: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the DNA and Gender to be
			// supplied
			for (account, dna, gender) in &self.kitties {
				assert!(Pallet::<T>::mint(account, *dna, *gender).is_ok());
			}
		}
	}


    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Kitty
        /// A new kitty was successfully created.
        CreatedKitty { kitty: [u8; 16], owner: T::AccountId },
        /// A kitty was successfully transferred.
        TransferredKitty { from: T::AccountId, to: T::AccountId, kitty: [u8; 16] },
        /// The price of a kitty was successfully set.
        PriceSetKitty { kitty: [u8; 16], price: Option<BalanceOf<T>> },
        /// A kitty was successfully sold.
        SoldKitty { seller: T::AccountId, buyer: T::AccountId, kitty: [u8; 16], price: BalanceOf<T> },
        /// Dog
        /// A new dog was successfully created.
        CreatedDog { dog: [u8; 16], owner: T::AccountId },
        /// A dog was successfully transferred.
        TransferredDog { from: T::AccountId, to: T::AccountId, dog: [u8; 16] },
        /// The price of a dog was successfully set.
        PriceSetDog { dog: [u8; 16], price: Option<BalanceOf<T>> },
        /// A dog was successfully sold.
        SoldDog { seller: T::AccountId, buyer: T::AccountId, dog: [u8; 16], price: BalanceOf<T> },
    }

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {
        /// An account may only own `MaxKittiesOwned` kitties.
        TooManyOwned,
        /// This kitty already exists!
        DuplicateKitty,
        /// This kitty already exists!
        DuplicateDog,
        /// An overflow has occurred!
        Overflow,
        /// This kitty does not exist!
        NoKitty,
        /// This dog does not exist!
        NoDog,
        /// You are not the owner of this kitty/dog.
        NotOwner,
        /// Trying to transfer or buy a kitty/dog from oneself.
        TransferToSelf,
        /// Ensures that the buying price is greater than the asking price.
        BidPriceTooLow,
        /// This kitty/dog is not for sale.
        NotForSale,
    }

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique dogs.
        ///
        /// The actual dog creation is done in the `mint()` function.
        #[pallet::weight(0)]
        pub fn create_dog(origin: OriginFor<T>) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Generate unique DNA and Gender using a helper function
            let (dog_gen_dna, gender) = Self::gen_dna();

            // Write new kitty to storage by calling helper function
            Self::mint(&sender, dog_gen_dna, gender)?;

            Ok(())
        }
        /// Create a new unique kitty.
        ///
        /// The actual kitty creation is done in the `mint()` function.
        #[pallet::weight(0)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Generate unique DNA and Gender using a helper function
            let (kitty_gen_dna, gender) = Self::gen_dna();

            // Write new kitty to storage by calling helper function
            Self::mint(&sender, kitty_gen_dna, gender)?;

            Ok(())
        }

        /// Directly transfer a kitty to another recipient.
        ///
        /// Any account that holds a kitty can send it to another Account. This will reset the
        /// asking price of the kitty, marking it not for sale.
        #[pallet::weight(0)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            kitty_id: [u8; 16],
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let from = ensure_signed(origin)?;
            let kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            ensure!(kitty.owner == from, Error::<T>::NotOwner);
            Self::do_transfer_kitty(kitty_id, to)?;
            Ok(())
        }

        /// Set the price for a kitty.
        ///
        /// Updates kitty price and updates storage.
        #[pallet::weight(0)]
        pub fn set_price(
            origin: OriginFor<T>,
            kitty_id: [u8; 16],
            new_price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Ensure the kitty exists and is called by the kitty owner
            let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            ensure!(kitty.owner == sender, Error::<T>::NotOwner);

            // Set the price in storage
            kitty.price = new_price;
            Kitties::<T>::insert(&kitty_id, kitty);

            // Deposit a "PriceSet" event.
            Self::deposit_event(Event::PriceSetKitty { kitty: kitty_id, price: new_price });

            Ok(())
        }

        /// Buy a saleable kitty. The bid price provided from the buyer has to be equal or higher
        /// than the ask price from the seller.
        ///
        /// This will reset the asking price of the kitty, marking it not for sale.
        /// Marking this method `transactional` so when an error is returned, we ensure no storage
        /// is changed.
        #[pallet::weight(0)]
        pub fn buy_kitty(
            origin: OriginFor<T>,
            kitty_id: [u8; 16],
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let buyer = ensure_signed(origin)?;
            // Transfer the kitty from seller to buyer as a sale.
            Self::do_buy_kitty(kitty_id, buyer, bid_price)?;

            Ok(())
        }
    }

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {
        // Generates and returns DNA and Gender
        fn gen_dna() -> ([u8; 16], Gender) {
            // Create randomness
            let random = T::KittyRandomness::random(&b"dna"[..]).0;

            // Create randomness payload. Multiple kitties can be generated in the same block,
            // retaining uniqueness.
            let unique_payload = (
                random,
                frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
                frame_system::Pallet::<T>::block_number(),
            );

            // Turns into a byte array
            let encoded_payload = unique_payload.encode();
            let hash = frame_support::Hashable::blake2_128(&encoded_payload);

            // Generate Gender
            if hash[0] % 2 == 0 {
                (hash, Gender::Male)
            } else {
                (hash, Gender::Female)
            }
        }

        // Helper to mint a dog
        pub fn mint_dog(
            owner: &T::AccountId,
            dna: [u8; 16],
            gender: Gender,
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let dog = Dog::<T> { dna, price: None, gender, owner: owner.clone() };

            // Check if the dog does not already exist in our storage map
            ensure!(!Dogs::<T>::contains_key(&dog.dna), Error::<T>::DuplicateKitty);

            // Performs this operation first as it may fail
            let count = CountForDogs::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Append dog to DogsOwned
            DogsOwned::<T>::try_append(&owner, dog.dna)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new dog to storage
            Dogs::<T>::insert(dog.dna, dog);
            CountForDogs::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::CreatedDog { dog: dna, owner: owner.clone() });

            // Returns the DNA of the new dog if this succeeds
            Ok(dna)
        }

        // Helper to mint a kitty
        pub fn mint(
            owner: &T::AccountId,
            dna: [u8; 16],
            gender: Gender,
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let kitty = Kitty::<T> { dna, price: None, gender, owner: owner.clone() };

            // Check if the kitty does not already exist in our storage map
            ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

            // Performs this operation first as it may fail
            let count = CountForKitties::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Append kitty to KittiesOwned
            KittiesOwned::<T>::try_append(&owner, kitty.dna)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new kitty to storage
            Kitties::<T>::insert(kitty.dna, kitty);
            CountForKitties::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::CreatedKitty { kitty: dna, owner: owner.clone() });

            // Returns the DNA of the new kitty if this succeeds
            Ok(dna)
        }

        // Update storage to transfer kitty
        pub fn do_transfer_kitty(
            kitty_id: [u8; 16],
            to: T::AccountId,
        ) -> DispatchResult {
            // Get the kitty
            let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            let from = kitty.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = KittiesOwned::<T>::get(&from);

            // Remove kitty from list of owned kitties.
            if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoKitty.into())
            }

            // Add kitty to the list of owned kitties.
            let mut to_owned = KittiesOwned::<T>::get(&to);
            to_owned.try_push(kitty_id).map_err(|()| Error::<T>::TooManyOwned)?;

            // Transfer succeeded, update the kitty owner and reset the price to `None`.
            kitty.owner = to.clone();
            kitty.price = None;

            // Write updates to storage
            Kitties::<T>::insert(&kitty_id, kitty);
            KittiesOwned::<T>::insert(&to, to_owned);
            KittiesOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::TransferredKitty { from, to, kitty: kitty_id });

            Ok(())
        }

        // A helper function for purchasing a kitty
        pub fn do_buy_kitty(
            kitty_id: [u8; 16],
            to: T::AccountId,
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Get the kitty
            let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
            let from = kitty.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = KittiesOwned::<T>::get(&from);

            // Remove kitty from list of owned kitties.
            if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoKitty.into())
            }

            // Add kitty to the list of owned kitties.
            let mut to_owned = KittiesOwned::<T>::get(&to);
            to_owned.try_push(kitty_id).map_err(|()| Error::<T>::TooManyOwned)?;

            // Mutating state here via a balance transfer, so nothing is allowed to fail after this.
            if let Some(price) = kitty.price {
                ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
                // Transfer the amount from buyer to seller
                T::Currency::transfer(&to, &from, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;
                // Deposit sold event
                Self::deposit_event(Event::SoldKitty {
                    seller: from.clone(),
                    buyer: to.clone(),
                    kitty: kitty_id,
                    price,
                });
            } else {
                return Err(Error::<T>::NotForSale.into())
            }

            // Transfer succeeded, update the kitty owner and reset the price to `None`.
            kitty.owner = to.clone();
            kitty.price = None;

            // Write updates to storage
            Kitties::<T>::insert(&kitty_id, kitty);
            KittiesOwned::<T>::insert(&to, to_owned);
            KittiesOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::TransferredKitty { from, to, kitty: kitty_id });

            Ok(())
        }
        // A helper function for purchasing a dog
        pub fn do_buy_dog(
            dog_id: [u8; 16],
            to: T::AccountId,
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Get the dog
            let mut dog = Dogs::<T>::get(&dog_id).ok_or(Error::<T>::NoKitty)?;
            let from = dog.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = DogsOwned::<T>::get(&from);

            // Remove kitty from list of owned kitties.
            if let Some(ind) = from_owned.iter().position(|&id| id == dog_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoDog.into())
            }

            // Add kitty to the list of owned kitties.
            let mut to_owned = KittiesOwned::<T>::get(&to);
            to_owned.try_push(dog_id).map_err(|()| Error::<T>::TooManyOwned)?;

            // Mutating state here via a balance transfer, so nothing is allowed to fail after this.
            if let Some(price) = dog.price {
                ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
                // Transfer the amount from buyer to seller
                T::Currency::transfer(&to, &from, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;
                // Deposit sold event
                Self::deposit_event(Event::SoldKitty {
                    seller: from.clone(),
                    buyer: to.clone(),
                    kitty: dog_id,
                    price,
                });
            } else {
                return Err(Error::<T>::NotForSale.into())
            }

            // Transfer succeeded, update the kitty owner and reset the price to `None`.
            dog.owner = to.clone();
            dog.price = None;

            // Write updates to storage
            Dogs::<T>::insert(&dog_id, dog);
            DogsOwned::<T>::insert(&to, to_owned);
            DogsOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::TransferredKitty { from, to, kitty: dog_id });

            Ok(())
        }
    }
}