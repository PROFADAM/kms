\# 📚 API Documentation



\## KMS Class



\### Constructor



```javascript

const kms = new KMS(config);


const kms = new KMS({

&nbsp; network: 'mainnet',

&nbsp; encryption: true

});


const wallet = kms.createWallet('ethereum');

const keys = wallet.generate();

console.log(keys.address);


const wallet = kms.importWallet('bitcoin', 'YOUR\_PRIVATE\_KEY');


const btcWallet = kms.createWallet('bitcoin');

const keys = btcWallet.generate();


const CryptoUtils = require('./src/utils/crypto');

const encrypted = CryptoUtils.encrypt('secret', 'password123');


try {

&nbsp; const wallet = kms.createWallet('ethereum');

&nbsp; const keys = wallet.generate();

} catch (error) {

&nbsp; console.error('Error:', error.message);

}





