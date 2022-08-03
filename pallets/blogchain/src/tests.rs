use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use codec::{Codec, Decode, Encode};
use frame_system::ensure_signed;
use sp_runtime::traits::{Bounded, Hash};
use sp_core::H256;
use crate::BlogPosts;
use crate::BlogPost;
// use crate as pallet_template;
use crate as pallet_blogchain;

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
//Creates a Blog correctly
fn it_works_for_blog_post() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		assert_ok!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0));
		let content = string_message.encode();
		
		let hash_blog_post = BlogPost::<Test>{ content: content.clone(), author: 1 };
		let blog_post_id = <Test as frame_system::Config>::Hashing::hash_of(&string_message.encode());
		
		let encoded_blogpost = BlogPosts::<Test>::get(blog_post_id);
		let decoded_blogpost = encoded_blogpost.encode();
		let decoded_blogpost1 =vec![0];

		let final_blog_post = BlogPost::<Test>{ content: content.clone(), author: 1 };
		assert_eq!(decoded_blogpost,decoded_blogpost1);
		
		// let decoded_blogpost: BlogPost!
		
		// println!("{:?)", encoded_blogpost);
// {12, 139, 191, 57, 48, 132, 14, 117, ...}
		
	});
}
//Creates a Blog wrongly: BlogPostNotEnoughBytes
#[test]
fn correct_error_blog_too_small_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "Bitcoin Whitepaper";
		assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}
//Creates a Blog wrongly: BlogPostMaxBytes
#[test]
fn correct_error_blog_too_big_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes";
		assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostTooManyBytes);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}


//Create Blog comment correctly
fn it_works_for_blog_comment() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		assert_ok!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0));
		let content = string_message.encode();
		
		let hash_blog_post = BlogPost::<Test>{ content: content.clone(), author: 1 };
		let blog_post_id = <Test as frame_system::Config>::Hashing::hash_of(&string_message.encode());
		
		let encoded_blogpost = BlogPosts::<Test>::get(blog_post_id);
		let decoded_blogpost = encoded_blogpost.encode();
		let decoded_blogpost1 =vec![0];

		let final_blog_post = BlogPost::<Test>{ content: content.clone(), author: 1 };
		assert_eq!(decoded_blogpost,decoded_blogpost1);

		///
		let hash = H256::zero();
		assert_ok!(BlogModule::create_blog_post_comment(Origin::signed(1), string_message.encode(),blog_post_id));
	});
}

//Creates a Blog comment wrongly: BlogPostMaxBytes
#[test]
fn correct_error_blog_comment_too_big_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes";
		let hash = H256::zero();
		assert_noop!(BlogModule::create_blog_post_comment(Origin::signed(1), string_message.encode(),hash), Error::<Test>::BlogPostCommentTooManyBytes);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}

//Creates a Blog Comment wrongly: BlogPostMaxBytes
#[test]
fn correct_error_blog__comment_too_small_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "";
		let hash = H256::zero();
		assert_noop!(BlogModule::create_blog_post_comment(Origin::signed(1), string_message.encode(),hash), Error::<Test>::BlogPostCommentNotEnoughBytes);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}
//Creates a Blog wrongly: BlogPostNotFound
#[test]
fn correct_error_blog__comment_not_valdiate_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		let hash = H256::zero();
		assert_noop!(BlogModule::create_blog_post_comment(Origin::signed(1), string_message.encode(),hash), Error::<Test>::BlogPostNotFound);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}
//Creates a Blog wrongly: TipperIsAuthor
#[test]
fn correct_blog_good_Tip_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		assert_ok!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0));
		let content = string_message.encode();
		
		let hash_blog_post = BlogPost::<Test>{ content: content.clone(), author: 1 };
		let blog_post_id = <Test as frame_system::Config>::Hashing::hash_of(&string_message.encode());
		
		let encoded_blogpost = BlogPosts::<Test>::get(blog_post_id);
		let decoded_blogpost = encoded_blogpost.encode();
		let decoded_blogpost1 =vec![0];

		let final_blog_post = BlogPost::<Test>{ content: content.clone(), author: 1 };
		assert_eq!(decoded_blogpost,decoded_blogpost1);

		let result = BlogModule::tip_blog_post(Origin::signed(1), blog_post_id,20);
		// assert_ok!(result);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}
//Creates a Blog wrongly: TipperIsAuthor 
#[test]
fn correct_error_blog__not_good_Tip_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		let hash = H256::zero();
		assert_noop!(BlogModule::tip_blog_post(Origin::signed(1), hash,20), Error::<Test>::BlogPostNotFound);
		// assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),0), Error::<Test>::BlogPostNotEnoughBytes);
	});
}

//Create Blog comment correctly Asset  ID wrong, Invalid 
#[test]
fn it_works_for_blog_comment_asset_id() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let string_message =  "Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services Bitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and servicesBitcoin Whitepaper - Written by Satoshi Nakamoto in 2008, it describes the original plan and protocol for Bitcoin. BitPay - BitPay is a payment processing company and software that allows merchants such as eBay, Amazon and other online shopping channels to accept bitcoin as payment for its goods and services";
		/// Asset id  = 3,  asset high 
		assert_noop!(BlogModule::create_blog_post(Origin::signed(1), string_message.encode(),10), Error::<Test>::AssetIDisNotValidate);
	});
}
