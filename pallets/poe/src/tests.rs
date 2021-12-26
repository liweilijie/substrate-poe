use crate::{Error, mock::*, Proofs};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works() {
   new_test_ext().execute_with(|| {
       let claim = vec![0,1];
       assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
   })
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_exceeds_length() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2,3,4,5];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ExceedMaxClaimLength
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None);
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

//  测试: 不是拥有才撤销存证报错
#[test]
fn revoke_claim_failed_when_claim_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

//  测试: 转移存储功能
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((2, frame_system::Pallet::<Test>::block_number()))
        );
    })
}
//  测试: 转移存储不存在的错误
#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let claim2 = vec![1,2];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim2.clone()),
            Error::<Test>::ClaimNotExist
        );

    })
}

//  测试: 转移存储owner不对
#[test]
fn transfer_claim_failed_when_claim_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );

    })
}
