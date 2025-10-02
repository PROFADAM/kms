/**
 * Wallet Test Suite
 */

const assert = require('assert');
const KMS = require('../src/index');

describe('KMS Wallet Tests', () => {
  let kms;
  
  beforeEach(() => {
    kms = new KMS();
  });
  
  describe('Bitcoin Wallet', () => {
    it('should create valid Bitcoin wallet', () => {
      const wallet = kms.createWallet('bitcoin');
      const keys = wallet.generate();
      
      assert(keys.address, 'Address should exist');
      assert(keys.address.startsWith('1'), 'Bitcoin address should start with 1');
      assert(keys.privateKey.length === 64, 'Private key should be 64 chars');
    });
    
    it('should import Bitcoin wallet', () => {
      const wallet = kms.createWallet('bitcoin');
      const keys = wallet.generate();
      
      const imported = kms.importWallet('bitcoin', keys.privateKey);
      assert.equal(imported.address, keys.address, 'Addresses should match');
    });
  });
  
  describe('Ethereum Wallet', () => {
    it('should create valid Ethereum wallet', () => {
      const wallet = kms.createWallet('ethereum');
      const keys = wallet.generate();
      
      assert(keys.address, 'Address should exist');
      assert(keys.address.startsWith('0x'), 'Ethereum address should start with 0x');
      assert(keys.address.length === 42, 'Ethereum address should be 42 chars');
    });
  });
  
  describe('Solana Wallet', () => {
    it('should create valid Solana wallet', () => {
      const wallet = kms.createWallet('solana');
      const keys = wallet.generate();
      
      assert(keys.publicKey, 'Public key should exist');
      assert(keys.secretKey, 'Secret key should exist');
    });
  });
});

console.log('✅ All tests passed!');
