mod utils;

use bitcoin::consensus::deserialize;
use bitcoin::Transaction;
use ordinals::{Artifact, Runestone};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

fn runestone_from_tx(tx: &[u8]) -> Result<Artifact, JsValue> {
    if let Ok(tx) = deserialize::<Transaction>(&tx) {
        let runestone = Runestone::decipher(&tx);

        return match runestone {
            Some(parsed) => Ok(parsed),
            None => Err(JsValue::from_str("No runestone found")),
        };
    } else {
        return Err(JsValue::from_str("Invalid transaction"));
    }
}

/** Parse a TX and return a runestone message. Throws if the TX does not contain a runestone.  */
#[wasm_bindgen(js_name = parseRunestone)]
pub fn parse_runestone(tx: &[u8]) -> Result<JsValue, JsValue> {
    set_panic_hook();

    match runestone_from_tx(tx) {
        Ok(runestone) => match runestone {
            Artifact::Cenotaph(_) => Err(JsValue::from_str("Cenotaph found")),
            Artifact::Runestone(runestone) => Ok(serde_wasm_bindgen::to_value(&runestone)?),
        },
        Err(e) => Err(e),
    }
}

/** Verify if a TX is valid if:
 * TX does not contain a runestone OR
 * TX contains a runestone and contains no cenotaphs
 */
#[wasm_bindgen(js_name = verifyRunestone)]
pub fn verify_runestone(tx: &[u8]) -> bool {
    set_panic_hook();

    match runestone_from_tx(tx) {
        Ok(parsed) => match parsed {
            Artifact::Cenotaph(c) => {
                println!("Cenotaph found: {} ", c.flaw.unwrap().to_string());
                false
            }

            Artifact::Runestone(_) => true,
        },
        Err(_) => true,
    }
}

// tests
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn parse_runestone_valid_runestone() {
        let tx = hex::decode("010000000001016f0ea3ff3f1587c148d160e8e86a230fc7c671b4913daa2ce0f425265817d0f94901000000ffffffff0300000000000000000a6a5d07149bc30b14de012202000000000000225120effe107f3ba5cd2843309e105839fcaa6531bd9428f77bdd35eea99db9c7d7f1c209030000000000225120effe107f3ba5cd2843309e105839fcaa6531bd9428f77bdd35eea99db9c7d7f101402f189011fb5f7176a657593a6db6d4f009e34a0e1f0a3eb0def9682e8adfebc22f7b6b0320971718e764c975322abfdb36934b45ae40c0b00e4e6df0f3cd57a600000000").unwrap();

        let sut = parse_runestone(&tx);

        assert_eq!(sut.is_ok(), true);
    }

    #[wasm_bindgen_test]
    fn parse_runestone_invalid_tx() {
        let tx = hex::decode("0000").unwrap();

        let sut = parse_runestone(&tx);

        assert_eq!(sut.is_err(), true);
    }

    #[wasm_bindgen_test]
    fn parse_runestone_valid_tx_missing_runestone() {
        let tx = hex::decode("02000000000101e5de17ac68e642fdd493955c9e9c239c6199d2a62f433c0d8c1c7a1a4a0330f89000000000fdffffff01facb0100000000001976a914cf0d811541c6dfab3b2839f75cc318174930aaa788ac024730440220576670f8f7c687e832bf20e2e00232b13b91bc8e5a0be661ba9c8a911b056027022049a0d9d606c2b2da5aa63731dc42c9d38254c8f10832eab3517d63c7da30e3bf012103bb85addd0cb568e5282a1908b785f8f985a6df9e8a311b4fa62fb1261cb991e600000000").unwrap();

        let sut = parse_runestone(&tx);

        assert_eq!(sut.is_err(), true);
    }

    #[wasm_bindgen_test]
    fn verify_runestone_valid_tx_with_runestone() {
        let tx = hex::decode("010000000001016f0ea3ff3f1587c148d160e8e86a230fc7c671b4913daa2ce0f425265817d0f94901000000ffffffff0300000000000000000a6a5d07149bc30b14de012202000000000000225120effe107f3ba5cd2843309e105839fcaa6531bd9428f77bdd35eea99db9c7d7f1c209030000000000225120effe107f3ba5cd2843309e105839fcaa6531bd9428f77bdd35eea99db9c7d7f101402f189011fb5f7176a657593a6db6d4f009e34a0e1f0a3eb0def9682e8adfebc22f7b6b0320971718e764c975322abfdb36934b45ae40c0b00e4e6df0f3cd57a600000000").unwrap();

        let sut = verify_runestone(&tx);

        assert_eq!(sut, true);
    }

    #[wasm_bindgen_test]
    fn verify_runestone_valid_tx_missing_runestone() {
        let tx = hex::decode("02000000000101e5de17ac68e642fdd493955c9e9c239c6199d2a62f433c0d8c1c7a1a4a0330f89000000000fdffffff01facb0100000000001976a914cf0d811541c6dfab3b2839f75cc318174930aaa788ac024730440220576670f8f7c687e832bf20e2e00232b13b91bc8e5a0be661ba9c8a911b056027022049a0d9d606c2b2da5aa63731dc42c9d38254c8f10832eab3517d63c7da30e3bf012103bb85addd0cb568e5282a1908b785f8f985a6df9e8a311b4fa62fb1261cb991e600000000").unwrap();

        let sut = verify_runestone(&tx);

        assert_eq!(sut, true);
    }

    #[wasm_bindgen_test]
    fn verify_runestone_valid_tx_with_cenotaph() {
        let tx = hex::decode("02000000000104456b97046dfd590379e9fbbf451ef155d397c210cd2a832614fc90052aa8925bd802000000ffffffff994acd03cfa10845d92c91a34ca43ef05d903d8c7216960ba4484ef5c9fd735900000000171600145fc724fa1533446104055c72334fede398e1fbc6fffffffffc39802c95b86129850e8d3b655a5bd7357136a0cd135173ffbd3bf94497938400000000171600145fc724fa1533446104055c72334fede398e1fbc6ffffffff975aeb26bc06ea0e43142b7460183b490ce83e9d138f0f035d24bb49deb5631d01000000171600145fc724fa1533446104055c72334fede398e1fbc6ffffffff05a0e9a00200000000225120b7f6d8928f0784629acfdee4afb4e6bb3540fc396231b19d5607707cf4a8b85122020000000000002251201da3ab6b81807b1f0da0042ed3df1fa4015b6710463b49a393d3208325298e6600000000000000000c6a5d0900c0a2331ca0968001545d03000000000016001491ccbeeccd6af6db1c446ff21f97f28ccd803dc3f60999000000000017a914afe1816123d06b2d132a325df11534c7e67035ec870141588aa0b080d511a5a2c68d119c67a100b529dc145ba160d97b6039b91fc35443d5db0414a4bf935ca238b103b3e4652ea3c0d238a2c8da1cf19a78fa4682991383024830450221008d46493336d40ae39934fe31b0d4ecec1ed89e1cac207ac53a23aac7495975d402204056f38a796f064617ae9625407dda1dc062688c59b175c5549019475672dc25012103310d72aaf5486e8d15e7c4d9f8f7e07c8928727ae573c4eb51983ed48ff7f08a0247304402203fdc48d000521c864c3e4edb7c84bcb0815e3390974b567a015cae8147628e3c02202338e41ea2eed20dda0bc2cb1f1e722a0042bec9f87ad94334630aa7badf22c7012103310d72aaf5486e8d15e7c4d9f8f7e07c8928727ae573c4eb51983ed48ff7f08a0247304402202b55c918e290e565520c19545d2f9e59d72bf3be2c385947604f4033bb925123022079872ffb38eab92aee7b72e5c7946b939a454030c60b84ad00149c10ebae4139012103310d72aaf5486e8d15e7c4d9f8f7e07c8928727ae573c4eb51983ed48ff7f08a00000000").unwrap();

        let sut = verify_runestone(&tx);

        assert_eq!(sut, false);
    }
}
