use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
/*

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

*/


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let content: Vec<u8> = "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		assert_ok!(BlogModule::create_blog_post(Origin::signed(1), content,0));
		// Read pallet storage and assert an expected result.
		// assert_eq!(BlogModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(BlogModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}
