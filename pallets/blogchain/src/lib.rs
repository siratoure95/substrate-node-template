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
	pub(super) type AseetIDNFT<T: Config> = StorageValue<_, u64, ValueQuery>;
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
        type AssetId: Get<u64>;
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
	#[pallet::storage]
	#[pallet::getter(fn comment_votings)]
	pub(super) type 
	CommenterVote<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<<T as frame_system::Config>::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn blog_posts)]
	pub(super) type BlogPosts<T: Config> = StorageMap<_, Twox64Concat, T::Hash, BlogPost<T>>;

	#[pallet::storage]
	#[pallet::getter(fn blog_post_comments)]
	pub(super) type BlogPostComments<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Vec<BlogPostComment<T>>>; //CountedStorage
	
	#[pallet::storage]
	#[pallet::getter(fn comments_counter)]
	pub(super) type CounterComments<T: Config> = CountedStorageMap<_, Twox64Concat, T::Hash, Vec<<T as frame_system::Config>::AccountId>>; //CountedStorage

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

			let author = ensure_signed(origin.clone())?;
			println!("Author {:?}",author);
			println!("Content {:?}",content.len());
			ensure!(
					(content.len() as u32) > T::BlogPostMinBytes::get(),
					<Error<T>>::BlogPostNotEnoughBytes
			);

			ensure!(
					(content.len() as u32) < T::BlogPostMaxBytes::get(),
					<Error<T>>::BlogPostTooManyBytes
			);
			
			let blog_post = BlogPost { content: content.clone(), author: author.clone() };
			// println!("blog_post {:?}",blog_post);
			let blog_post_id = T::Hashing::hash_of(&blog_post);
			println!("blog_post_id {:?}",blog_post_id);
			let res = <BlogPosts<T>>::insert(blog_post_id, blog_post);
			let encoded_blogpost = BlogPosts::<T>::get(blog_post_id);
			let comments_vec: Vec<BlogPostComment<T>> = Vec::new();
			println!("2. blog_post_id {:?}",blog_post_id);
			<BlogPostComments<T>>::insert(blog_post_id, comments_vec);
			println!("1. blog_post_id {:?}",blog_post_id);
			
			
			if _asset_id == 0{
				//kitties
				println!("Create Kitt");
				let _result_kitty = pallet_template::Pallet::<T>::create_kitty(origin.clone());
				println!("_result_kitty {:?}",_result_kitty);
				let count = AseetIDNFT::<T>::get();
				AseetIDNFT::<T>::put(0);

			}
			else if _asset_id == 1 {
				//dog
				let _result_dog = pallet_template::Pallet::<T>::create_dog(origin.clone());
				println!("_result_dog {:?}",_result_dog);
				let count = AseetIDNFT::<T>::get();
				AseetIDNFT::<T>::put(1);

			}
			else{
				ensure!(
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisNotValidate
				);

			}		
			Self::deposit_event(Event::BlogPostCreated(content, author, blog_post_id));
			Ok(())
		}
		
		#[pallet::weight(5000)]
		pub fn create_blog_post_comment(
				origin: OriginFor<T>,
				content: Vec<u8>,
				blog_post_id: T::Hash,
		) -> DispatchResult{
			let mut kitties_counter_comment = 0;
			let mut dogs_counter_comment = 0;
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

			let _asset_id = AseetIDNFT::<T>::get();
			if _asset_id == 0{
				//kitties
				 let author = ensure_signed(origin.clone())?;
				 let _result_kitty = pallet_template::Pallet::<T>::create_kitty(origin.clone());
				 let mut kitty_comment_vec: Vec<<T as frame_system::Config>::AccountId> = Vec::new();
				 kitty_comment_vec.push(author);
				 <CommenterVote<T>>::insert(blog_post_id, kitty_comment_vec);
				 let author = ensure_signed(origin.clone())?;
				 let _result_kitty = pallet_template::Pallet::<T>::create_kitty(origin.clone());
				 let mut kitty_comment_vec: Vec<<T as frame_system::Config>::AccountId> = Vec::new();
				 kitty_comment_vec.push(author);
				 <CounterComments<T>>::insert(blog_post_id, kitty_comment_vec.clone());
			}
			else if _asset_id == 1 {
				//dogs
				let author = ensure_signed(origin.clone())?;
				let _result_dog = pallet_template::Pallet::<T>::create_dog(origin.clone());
				let mut dog_comment_vec: Vec<<T as frame_system::Config>::AccountId> = Vec::new();
				dog_comment_vec.push(author);
				<CommenterVote<T>>::insert(blog_post_id, dog_comment_vec);
				let author = ensure_signed(origin.clone())?;
				let _result_dog = pallet_template::Pallet::<T>::create_dog(origin.clone());
				let mut dog_comment_vec: Vec<<T as frame_system::Config>::AccountId> = Vec::new();
				dog_comment_vec.push(author);
				<CounterComments<T>>::insert(blog_post_id, dog_comment_vec.clone());


				//dog
				let _result_dog = pallet_template::Pallet::<T>::create_kitty(origin.clone());
			}
			else if _asset_id <  0 {
				ensure!(
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooLow
				);
			}
			else if _asset_id >  1 {
				ensure!(
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooHigh
				);
			}
			else{
				ensure!(
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisNotValidate
				);

			}

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
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooLow
				);
			}
			else if _asset_id >  1 {
				ensure!(
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisTooHigh
				);
			}
			else{
				ensure!(
					(_asset_id as u64) > T::AssetId::get(),
					<Error<T>>::AssetIDisNotValidate
				);

			}
			/*We are going to see which blogger gets to 5 votes first. 
				Which ever blogger gets to 10 comments votes first will win  
				or the blogger with the highest blog amount will win. If 
				blogger has 2 different comments then they get the oopposite NFT.
				If blogger minted a kity NFT then they would get the dog NFT.
				IF blogger minted a dog NFT then they would get the kitty NFTt*/
				let blogger_voters = CommenterVote::<T>::get(blog_post_id);
				let blogger_comment_voters = CounterComments::<T>::get(blog_post_id);
				for i in blogger_comment_voters {
					println!("{:?}", i);
					
				}


			Ok(())
		}

		#[pallet::weight(500)]
		pub fn tip_blog_post(
				origin: OriginFor<T>,
				blog_post_id: T::Hash,
				amount: <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance,
		) -> DispatchResult {
				let tipper = ensure_signed(origin)?;
				println!("tipper = {:?}",tipper);
				println!("blog_post_id = {:?}",blog_post_id);
				println!("amount = {:?}",amount);

				let blog_post = Self::blog_posts(&blog_post_id).ok_or(<Error<T>>::BlogPostNotFound)?;
				println!("blog_post_id = {:?}",blog_post_id);
				let blog_post_author = blog_post.author;
				println!("blog_post_author = {:?}",blog_post_author);
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

