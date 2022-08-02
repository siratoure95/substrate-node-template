#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, inherent::Vec, traits::Currency, sp_runtime::traits::Hash, transactional, traits::ExistenceRequirement};
	use frame_system::pallet_prelude::*;


	/// Keeps track of the number of kitties in existence.
	#[pallet::storage]
	pub(super) type CountForKittiesVotes<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Keeps track of the number of dogs in existence.
	#[pallet::storage]
	pub(super) type CountForDogsVotes<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Keeps track of the number of total animal votes in existence.
	#[pallet::storage]
	pub(super) type CountTotalVotes<T: Config> = StorageValue<_, u64, ValueQuery>;
	
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	// pub trait Config: frame_system::Config + pallet_template::Config {
	pub trait Config: frame_system::Config + pallet_template::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>; 
        type AssetId: Get<u8>;
		/// The maximum amount of kitties a single account can own.
		 #[pallet::constant]
        type MaxKittiesVotesOwned: Get<u32>;
		/// The maximum amount of dogs a single account can own.
		 #[pallet::constant]
        type MaxDogsVotesOwned: Get<u32>;
		#[pallet::constant]
		type MaxTotalVotes: Get<u32>;
		#[pallet::constant]
        type BlogPostMinBytes: Get<u32>;
        #[pallet::constant]
        type BlogPostMaxBytes: Get<u32>;
        #[pallet::constant]
        type BlogPostCommentMinBytes: Get<u32>;
        #[pallet::constant]
        type BlogPostCommentMaxBytes: Get<u32>; 
	}

	#[derive(Encode, Decode,TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct BlogPost<T: Config> {
			pub content: Vec<u8>,//BoundedVec
			pub author: <T as frame_system::Config>::AccountId,
	}

	#[derive( Encode, Decode,TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct BlogPostComment<T: Config> {
			pub content: Vec<u8>,
			pub blog_post_id: T::Hash,
			pub author: <T as frame_system::Config>::AccountId,
	}

	#[derive( Encode, Decode, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct CommentVoting<T: Config> {
		//Vec of the blog_post_id with the commenter id
		// pub blog_comment_id_identity : vec![T::AccountId],
		pub blog_comment_id_identity : Vec<T::AccountId>,
		pub blog_post_id_identity : Vec<T::Hashing>,
		//Asset number aka if its a Kitty or Dog
		pub blog_asset_number_vec: Vec<u8>,
		//Vector of the blog_id
		pub blog_id_post_vec :  Vec<T::Hashing>,
		pub blog_id_post_comment_vote: Vec<u8>,
		//owner of blog
		pub owner: T::AccountId,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// /// Maps the kitty struct to the kitty DNA.
	// #[pallet::storage]
	// #[pallet::getter(fn comment_votings)]
	// pub(super) type CommentVotings<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], CommentVoting<T>>;

	#[pallet::storage]
	#[pallet::getter(fn blog_posts)]
	pub(super) type BlogPosts<T: Config> = StorageMap<_, Twox64Concat, T::Hash, BlogPost<T>>;

	#[pallet::storage]
	#[pallet::getter(fn blog_post_comments)]
	pub(super) type BlogPostComments<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Vec<BlogPostComment<T>>>; //CountedStorage

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// SomethingStored(u32, T::AccountId),
		BlogPostCreated(Vec<u8>, T::AccountId, T::Hash),
        BlogPostCommentCreated(Vec<u8>, T::AccountId, T::Hash),
        Tipped(T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// An account may only own `MaxKittiesVotesOwned//MaxDogsVotesOwned` kitties and dogs.
		TooManyOwned,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		BlogPostNotEnoughBytes, 
        BlogPostTooManyBytes,
        BlogPostCommentNotEnoughBytes,
        BlogPostCommentTooManyBytes,
        BlogPostNotFound,
        TipperIsAuthor,
		AssetIDisTooHigh,
		AssetIDisTooLow,
		AssetIDisNotValidate,
		TooManyVotes,
		Not5CommentsNotFromAuthor,
		NotTheBlogPoster,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10000)]
		#[transactional]
		pub fn create_blog_post(origin: OriginFor<T>, content: Vec<u8>,_asset_id : u8) -> DispatchResult {

			let author = ensure_signed(origin)?;
			// let commentervoter = CommentVotings::<T>::get(&_asset_id).ok_or(Error::<T>::NotTheBlogPoster)?;
			// ensure!(commentervoter.owner == author, Error::<T>::NotTheBlogPoster);
			// CommentVoting::blog_id_post_vec.push(author);
			ensure!(
					(content.len() as u32) > T::BlogPostMinBytes::get(),
					<Error<T>>::BlogPostNotEnoughBytes
			);

			ensure!(
					(content.len() as u32) < T::BlogPostMaxBytes::get(),
					<Error<T>>::BlogPostTooManyBytes
			);

			let blog_post = BlogPost { content: content.clone(), author: author.clone() };

			let blog_post_id = T::Hashing::hash_of(&blog_post);

			<BlogPosts<T>>::insert(blog_post_id, blog_post);

			let comments_vec: Vec<BlogPostComment<T>> = Vec::new();
			<BlogPostComments<T>>::insert(blog_post_id, comments_vec);

			Self::deposit_event(Event::BlogPostCreated(content, author, blog_post_id));
			
			
			if _asset_id == 0{
				//kitties
				let _result_kitty = pallet_template::Pallet::<T>::create_kitty(origin);
				// let kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
				// ensure!(kitty.owner == from, Error::<T>::NotOwner);
				let blog_author = CommentVoting::<T>::blog_comment_id_identity.push(author);
				// ensure!(
				// 		(blog_comment_id_identity.len() as u32) > T::NotTheBlogPoster::get(),
				// 		<Error<T>>::BlogPostCommentNotEnoughBytes
				// );
			}
			else if _asset_id == 1 {
				//dog
				let _result_dog = pallet_template::Pallet::<T>::create_kitty(origin);
			}
			else if _asset_id <  0 {
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooLow
				);
			}
			else if _asset_id >  1 {
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooHigh
				);
			}
			else{
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisNotValidate
				);

			}		

			Ok(())
		}

		#[pallet::weight(5000)]
		pub fn create_blog_post_comment(
				origin: OriginFor<T>,
				content: Vec<u8>,
				blog_post_id: T::Hash,
				//assset to select dog and cat
				_asset_id : u8,
		) -> DispatchResult {
			let comment_author = ensure_signed(origin.clone())?;

			ensure!(
					(content.len() as u32) > T::BlogPostMinBytes::get(),
					<Error<T>>::BlogPostCommentNotEnoughBytes
			);

			ensure!(
					(content.len() as u32) < T::BlogPostMaxBytes::get(),
					<Error<T>>::BlogPostCommentTooManyBytes
			);

			let blog_post_comment = BlogPostComment {
					author: comment_author.clone(),
					content: content.clone(),
					blog_post_id: blog_post_id.clone(),
			};

			<BlogPostComments<T>>::mutate(blog_post_id, |comments| match comments {
					None => Err(()),
					Some(vec) => {
							vec.push(blog_post_comment);
							Ok(())
					},
			})
			.map_err(|_| <Error<T>>::BlogPostNotFound)?;

			Self::deposit_event(Event::BlogPostCommentCreated(
					content,
					comment_author,
					blog_post_id,
			));
						
			pub const MAX_TOTAL_VOTES: u64 = 10;
			pub const MAX_TOTAL_KITTIES_VOTES: u64 = 5;
			pub const MAX_TOTAL_DOGS_VOTES: u64 = 5;
			if _asset_id == 0{
				//kitties
				 let _result_kitty = pallet_template::Pallet::<T>::create_kitty(origin);
			}
			else if _asset_id == 1 {
				//dog
				let _result_dog = pallet_template::Pallet::<T>::create_kitty(origin);
			}
			else if _asset_id <  0 {
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooLow
				);
			}
			else if _asset_id >  1 {
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooHigh
				);
			}
			else{
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisNotValidate
				);

			}
			let mut kitties_counter_comment = 0;
			let mut dogs_counter_comment = 0;
			if _asset_id == 0{
				//kitties
				// Performs this operation first as it may fail
				// Create a new object
				let count = CountForKittiesVotes::<T>::get();
				let new_count = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
				if new_count >= MAX_TOTAL_KITTIES_VOTES {
					ensure!(
						(new_count as u32) > T::MaxKittiesVotesOwned::get(),
						<Error<T>>::TooManyVotes
					);
				}
				kitties_counter_comment = new_count;
				// Write new kitty vote to storage
				CountForKittiesVotes::<T>::put(new_count);
				// Vote_Kitties = Vote_Kitties +1;
				// Vote_Total = Vote_Total + 1;
				// Performs total vote counter
				let count = CountTotalVotes::<T>::get();
				let new_count = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
				// check to see if there is MAX_TOTAL_VOTES total votes 
				if new_count >= MAX_TOTAL_VOTES {
					ensure!(
						(new_count as u32) > T::MaxTotalVotes::get(),
						<Error<T>>::TooManyVotes
					);
				}

				// Write total vote to storage
				CountTotalVotes::<T>::put(new_count);
			}
			else if _asset_id == 1 {
				//dog
				// Performs this operation first as it may fail
				let count = CountForDogsVotes::<T>::get();
				let new_count = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
				// check to see if there is 30 kitty votes 
				if new_count >= MAX_TOTAL_DOGS_VOTES {
					ensure!(
						(new_count as u32) > T::MaxDogsVotesOwned::get(),
						<Error<T>>::TooManyVotes
					);
				}
				dogs_counter_comment = new_count;
				// Write new kitty vote to storage
				CountForDogsVotes::<T>::put(new_count);
				// Vote_Dogs = Vote_Dogs + 1;
				// Performs total vote counter
				let count = CountTotalVotes::<T>::get();
				let new_count = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
				// check to see if there is 60 total votes 
				if new_count >= MAX_TOTAL_VOTES {
					ensure!(
						(new_count as u32) > T::MaxTotalVotes::get(),
						<Error<T>>::TooManyVotes
					);
				}
				// Write total vote to storage
				CountTotalVotes::<T>::put(new_count);
			}

			else if _asset_id <  0 {
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooLow
				);
			}
			else if _asset_id >  1 {
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooHigh
				);
			}
			else{
				ensure!(
					(_asset_id as u8) > T::AssetId::get(),
					<Error<T>>::AssetIDisNotValidate
				);

			}

			// If blogger has 2 different comments then they get the oopposite NFT
			// IF blogger minted a kity NFT then they would get the dog NFT
			// IF blogger minted a dog NFT then they would get the kitty NFT
			// kitties_counter_comment
			// dogs_counter_comment
			// if(kitties_counter_comment == 2){
			// 	mint(
			// 		owner: &T::AccountId,
			// 		dna: [u8; 16],
			// 		gender: Gender,
			// 	) 

			// }


			Ok(())
		}

		#[pallet::weight(500)]
		pub fn tip_blog_post(
				origin: OriginFor<T>,
				blog_post_id: T::Hash,
				amount: <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance,
		) -> DispatchResult {
				let tipper = ensure_signed(origin)?;

				let blog_post = Self::blog_posts(&blog_post_id).ok_or(<Error<T>>::BlogPostNotFound)?;
				let blog_post_author = blog_post.author;

				ensure!(tipper != blog_post_author, <Error<T>>::TipperIsAuthor);

				<T as Config>::Currency::transfer(
						&tipper,
						&blog_post_author,
						amount,
						ExistenceRequirement::KeepAlive,
				)?;

				Self::deposit_event(Event::Tipped(tipper, blog_post_id));

				Ok(())
		}

	}
    //add pallet-hooks wth integrate testing 
}


