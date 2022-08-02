#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::traits::{Currency, Randomness};

    // The basis which we build
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Allows easy access our Pallet's `Balance` type. Comes from `Currency` interface.
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // The Gender type used in the `Cat` struct
    #[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Gender {
        Male,
        Female,
    }

    // Struct for holding Cat information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[scale_info(skip_type_params(T))]
    pub struct Cat<T: Config> {
        // Using 16 bytes to represent a Cat DNA
        pub dna: [u8; 16],
        // `None` assumes not for sale
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: T::AccountId,
    }

    /// Keeps track of the number of Cats in existence.
    #[pallet::storage]
    pub(super) type CountForCats<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Maps the Cat struct to the Cat DNA.
    #[pallet::storage]
    pub(super) type Cats<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Cat<T>>;

    /// Track the Cats owned by each account.
    #[pallet::storage]
    pub(super) type DogsOwned<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BoundedVec<[u8; 16], T::MaxDogsOwned>,
        ValueQuery,
    >;
	/// Configuring Your Pallet
	/// //Buliding pallets include somt custom configs which will allow pallets to gain access 
	/// to oustide interfaces such as :
	/// Being able to controlling user balances. 
    // Your Pallet's configuration trait, representing custom external types and interfaces.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Currency: 
        /// A trait interfaces to access 
        /// and control user balances. Also gives you 
        /// access to the Balance type.
        type Currency: Currency<Self::AccountId>;

        /// Maximum Cats Owned:
        /// Max number of cats owned in an account
        #[pallet::constant]
        type MaxDogsOwned: Get<u32>;

        /// The type of Randomness we want to specify for this pallet.
        type CatRandomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    // Your Pallet's events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new Cat was successfully created.
        Created { Cat: [u8; 16], owner: T::AccountId },
        /// A Cat was successfully transferred.
        Transferred { from: T::AccountId, to: T::AccountId, Cat: [u8; 16] },
        /// The price of a Cat was successfully set.
        PriceSet { Cat: [u8; 16], price: Option<BalanceOf<T>> },
        /// A Cat was successfully sold.
        Sold { seller: T::AccountId, buyer: T::AccountId, Cat: [u8; 16], price: BalanceOf<T> },
    }

    // Your Pallet's error messages.
    #[pallet::error]
    pub enum Error<T> {
        /// An account may only own `MaxDogsOwned` Cats.
        TooManyOwned,
        /// This Cat already exists!
        DuplicateCat,
        /// An overflow has occurred!
        Overflow,
        /// This Cat does not exist!
        NoCat,
        /// You are not the owner of this Cat.
        NotOwner,
        /// Trying to transfer or buy a Cat from oneself.
        TransferToSelf,
        /// Ensures that the buying price is greater than the asking price.
        BidPriceTooLow,
        /// This Cat is not for sale.
        NotForSale,
    }

    // Your Pallet's callable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new unique Cat.
        ///
        /// The actual Cat creation is done in the `mint()` function.
        #[pallet::weight(0)]
        pub fn create_Cat(origin: OriginFor<T>) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Generate unique DNA and Gender using a helper function
            let (Cat_gen_dna, gender) = Self::gen_dna();

            // Write new Cat to storage by calling helper function
            Self::mint(&sender, Cat_gen_dna, gender)?;

            Ok(())
        }

        /// Directly transfer a Cat to another recipient.
        ///
        /// Any account that holds a Cat can send it to another Account. This will reset the
        /// asking price of the Cat, marking it not for sale.
        #[pallet::weight(0)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            Cat_id: [u8; 16],
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let from = ensure_signed(origin)?;
            let Cat = Cats::<T>::get(&Cat_id).ok_or(Error::<T>::NoCat)?;
            ensure!(Cat.owner == from, Error::<T>::NotOwner);
            Self::do_transfer(Cat_id, to)?;
            Ok(())
        }

        /// Set the price for a Cat.
        ///
        /// Updates Cat price and updates storage.
        #[pallet::weight(0)]
        pub fn set_price(
            origin: OriginFor<T>,
            Cat_id: [u8; 16],
            new_price: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let sender = ensure_signed(origin)?;

            // Ensure the Cat exists and is called by the Cat owner
            let mut Cat = Cats::<T>::get(&Cat_id).ok_or(Error::<T>::NoCat)?;
            ensure!(Cat.owner == sender, Error::<T>::NotOwner);

            // Set the price in storage
            Cat.price = new_price;
            Cats::<T>::insert(&Cat_id, Cat);

            // Deposit a "PriceSet" event.
            Self::deposit_event(Event::PriceSet { Cat: Cat_id, price: new_price });

            Ok(())
        }

        /// Buy a saleable Cat. The bid price provided from the buyer has to be equal or higher
        /// than the ask price from the seller.
        ///
        /// This will reset the asking price of the Cat, marking it not for sale.
        /// Marking this method `transactional` so when an error is returned, we ensure no storage
        /// is changed.
        #[pallet::weight(0)]
        pub fn buy_Cat(
            origin: OriginFor<T>,
            Cat_id: [u8; 16],
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Make sure the caller is from a signed origin
            let buyer = ensure_signed(origin)?;
            // Transfer the Cat from seller to buyer as a sale.
            Self::do_buy_Cat(Cat_id, buyer, bid_price)?;

            Ok(())
        }
    }

    // Your Pallet's internal functions.
    impl<T: Config> Pallet<T> {
        // Generates and returns DNA and Gender
        fn gen_dna() -> ([u8; 16], Gender) {
            // Create randomness
            let random = T::CatRandomness::random(&b"dna"[..]).0;

            // Create randomness payload. Multiple Cats can be generated in the same block,
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

        // Helper to mint a Cat
        //https://docs.substrate.io/reference/how-to-guides/basics/mint-basic-tokens/
        pub fn mint(
            owner: &T::AccountId,
            dna: [u8; 16],
            gender: Gender,
        ) -> Result<[u8; 16], DispatchError> {
            // Create a new object
            let Cat = Cat::<T> { dna, price: None, gender, owner: owner.clone() };

            // Check if the Cat does not already exist in our storage map
            ensure!(!Cats::<T>::contains_key(&Cat.dna), Error::<T>::DuplicateCat);

            // Performs this operation first as it may fail
            let count = CountForCats::<T>::get();
            let new_count = count.checked_add(1).ok_or(Error::<T>::Overflow)?;

            // Append Cat to DogsOwned
            DogsOwned::<T>::try_append(&owner, Cat.dna)
                .map_err(|_| Error::<T>::TooManyOwned)?;

            // Write new Cat to storage
            Cats::<T>::insert(Cat.dna, Cat);
            CountForCats::<T>::put(new_count);

            // Deposit our "Created" event.
            Self::deposit_event(Event::Created { Cat: dna, owner: owner.clone() });

            // Returns the DNA of the new Cat if this succeeds
            Ok(dna)
        }

        // Update storage to transfer Cat
        pub fn do_transfer(
            Cat_id: [u8; 16],
            to: T::AccountId,
        ) -> DispatchResult {
            // Get the Cat
            let mut Cat = Cats::<T>::get(&Cat_id).ok_or(Error::<T>::NoCat)?;
            let from = Cat.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = DogsOwned::<T>::get(&from);

            // Remove Cat from list of owned Cats.
            if let Some(ind) = from_owned.iter().position(|&id| id == Cat_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoCat.into())
            }

            // Add Cat to the list of owned Cats.
            let mut to_owned = DogsOwned::<T>::get(&to);
            to_owned.try_push(Cat_id).map_err(|()| Error::<T>::TooManyOwned)?;

            // Transfer succeeded, update the Cat owner and reset the price to `None`.
            Cat.owner = to.clone();
            Cat.price = None;

            // Write updates to storage
            Cats::<T>::insert(&Cat_id, Cat);
            DogsOwned::<T>::insert(&to, to_owned);
            DogsOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::Transferred { from, to, Cat: Cat_id });

            Ok(())
        }

        // A helper function for purchasing a Cat
        pub fn do_buy_Cat(
            Cat_id: [u8; 16],
            to: T::AccountId,
            bid_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Get the Cat
            let mut Cat = Cats::<T>::get(&Cat_id).ok_or(Error::<T>::NoCat)?;
            let from = Cat.owner;

            ensure!(from != to, Error::<T>::TransferToSelf);
            let mut from_owned = DogsOwned::<T>::get(&from);

            // Remove Cat from list of owned Cats.
            if let Some(ind) = from_owned.iter().position(|&id| id == Cat_id) {
                from_owned.swap_remove(ind);
            } else {
                return Err(Error::<T>::NoCat.into())
            }

            // Add Cat to the list of owned Cats.
            let mut to_owned = DogsOwned::<T>::get(&to);
            to_owned.try_push(Cat_id).map_err(|()| Error::<T>::TooManyOwned)?;

            // Mutating state here via a balance transfer, so nothing is allowed to fail after this.
            if let Some(price) = Cat.price {
                ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
                // Transfer the amount from buyer to seller
                T::Currency::transfer(&to, &from, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;
                // Deposit sold event
                Self::deposit_event(Event::Sold {
                    seller: from.clone(),
                    buyer: to.clone(),
                    Cat: Cat_id,
                    price,
                });
            } else {
                return Err(Error::<T>::NotForSale.into())
            }

            // Transfer succeeded, update the Cat owner and reset the price to `None`.
            Cat.owner = to.clone();
            Cat.price = None;

            // Write updates to storage
            Cats::<T>::insert(&Cat_id, Cat);
            DogsOwned::<T>::insert(&to, to_owned);
            DogsOwned::<T>::insert(&from, from_owned);

            Self::deposit_event(Event::Transferred { from, to, Cat: Cat_id });

            Ok(())
        }
    }
}