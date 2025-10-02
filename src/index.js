/**
 * KMS - Key Management System
 * Main Entry Point
 */

const BitcoinWallet = require('./bitcoin/wallet');
const EthereumWallet = require('./ethereum/wallet');
const SolanaWallet = require('./solana/wallet');

class KMS {
  constructor(config = {}) {
    this.config = config;
    this.wallets = new Map();
  }

  /**
   * Create a new wallet for specified blockchain
   * @param {string} blockchain - bitcoin, ethereum, solana
   * @param {object} options - wallet options
   */
  createWallet(blockchain, options = {}) {
    switch (blockchain.toLowerCase()) {
      case 'bitcoin':
        return new BitcoinWallet(options);
      case 'ethereum':
        return new EthereumWallet(options);
      case 'solana':
        return new SolanaWallet(options);
      default:
        throw new Error(`Unsupported blockchain: ${blockchain}`);
    }
  }

  /**
   * Import existing wallet
   * @param {string} blockchain
   * @param {string} privateKey or mnemonic
   */
  importWallet(blockchain, credentials) {
    const wallet = this.createWallet(blockchain);
    wallet.import(credentials);
    return wallet;
  }

  /**
   * List all managed wallets
   */
  listWallets() {
    return Array.from(this.wallets.values());
  }
}

module.exports = KMS;
