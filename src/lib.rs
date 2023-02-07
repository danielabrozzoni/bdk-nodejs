use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::database::MemoryDatabase;
use bdk::descriptor;
use bdk::descriptor::ExtendedDescriptor;
use bdk::descriptor::IntoWalletDescriptor;
use bdk::keys::bip39::Mnemonic;
use bdk::keys::KeyMap;
use bdk::wallet::AddressIndex;
use bdk::wallet::SyncOptions;
use bdk::blockchain::EsploraBlockchain;
use js_sys::Promise;
use std::rc::Rc;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub struct Wallet {
    // Rc is not thread safe, if you need thread safety use
    // Arc<Mutex<
    wallet: Rc<bdk::Wallet<MemoryDatabase>>,
}

#[wasm_bindgen]
pub struct Descriptor {
    desc: (ExtendedDescriptor, KeyMap),
}

#[wasm_bindgen]
pub struct Network {
    network: bdk::bitcoin::Network,
}

#[wasm_bindgen]
impl Network {
    #[wasm_bindgen(constructor)]
    pub fn new(network: &str) -> Self {
        Network {
            network: bdk::bitcoin::Network::from_str(network).unwrap(),
        }
    }
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(constructor)]
    pub fn new(descriptor: Descriptor, network: &Network) -> Wallet {
        let wallet = Rc::new(
            bdk::Wallet::new(
                descriptor.desc,
                None,
                network.network,
                MemoryDatabase::new(),
            )
            .unwrap(),
        );
        Wallet { wallet }
    }

    pub fn get_new_address(&self) -> String {
        self.wallet
            .get_address(AddressIndex::New)
            .unwrap()
            .address
            .to_string()
    }

    pub async fn sync(&self) -> Promise {
        let esplora_url = "https://blockstream.info/testnet/api/";
        let blockchain = EsploraBlockchain::new(esplora_url, 20);
        let wallet = Rc::clone(&self.wallet);
        future_to_promise(async move { wallet.sync(&blockchain, SyncOptions::default()).await.map(|_| JsValue::null()).map_err(|e| e.to_string().into()) })
    }

    pub fn get_total_balance(&self) -> u64 {
        self.wallet.get_balance().unwrap().get_total()
    }
}

#[wasm_bindgen]
pub fn generate_mnemonic() -> String {
    use bdk::keys::bip39::{Language, Mnemonic, WordCount};
    use bdk::keys::{GeneratableKey, GeneratedKey};
    use bdk::miniscript::Tap;

    let mnemonic: GeneratedKey<_, Tap> =
        Mnemonic::generate((WordCount::Words12, Language::English))
            .expect("Error generating mnemonic");

    mnemonic.to_string()
}

#[wasm_bindgen]
pub fn mnemonic_to_bip86_tap_descriptor(
    mnemonic: String,
    passphrase: Option<String>,
    network: &Network,
) -> Descriptor {
    let secp = Secp256k1::new();

    let mnemonic_with_passphrase = (Mnemonic::from_str(&mnemonic).unwrap(), passphrase);
    let external_path = DerivationPath::from_str("m/86h/0h/0h/0").unwrap();

    Descriptor {
        desc: descriptor!(tr((mnemonic_with_passphrase.clone(), external_path)))
            .unwrap()
            .into_wallet_descriptor(&secp, network.network)
            .unwrap(),
    }
}
