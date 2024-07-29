// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;
contract UserRecords {
struct User {
uint8 userId;
uint8 userAge;
uint16 userCreditBalance;
bool userIsMember;
    }
uint randomCounter;
mapping (address => User) public users;
function addUserEntry(uint8 _userId, uint8 _userAge, uint16 _userCreditBalance, bool _userIsMember) public returns (bool) {
require(users[msg.sender].userId == 0, "User already exists!");
        users[msg.sender] = User(_userId, _userAge, _userCreditBalance, _userIsMember);
return true;
    }
function removeUserEntry() public returns (bool) {
require(users[msg.sender].userId != 0, "User does not exist!");
delete users[msg.sender];
return true;
    }
function updateUserEntry(uint8 _userId, uint8 _userAge, uint16 _userCreditBalance, bool _userIsMember) public returns (bool) {
require(users[msg.sender].userId != 0, "User does not exist!");
        users[msg.sender] = User(_userId, _userAge, _userCreditBalance, _userIsMember);
return true;
    }
function addOneToRandomCounter() public returns (bool) {
        randomCounter += 1;
return true;
    }
}