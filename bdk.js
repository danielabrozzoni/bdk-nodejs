const bdk = require('./target/wasm32-unknown-unknown/release/nodejs_example.js')

// Re-enable this line if you want the mnemonic to be generated
// let mnemonic = bdk.generate_mnemonic()
let mnemonic = "vapor winter snake hand reason airport text develop enrich abandon oyster filter";
console.log(mnemonic)

let network = new bdk.Network("testnet")
let desc = bdk.mnemonic_to_bip86_tap_descriptor(mnemonic, null, network)
let wallet = new bdk.Wallet(desc, network)

// Printing an address
console.log(wallet.get_new_address())

// The balance is zero before syncing
console.log(wallet.get_total_balance())

// But it might change after you sync
wallet.sync().then(() => console.log(wallet.get_total_balance()))
