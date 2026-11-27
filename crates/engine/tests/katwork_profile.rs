//! The build without the `network` feature (KatWork's): every command that would fetch something from another
//! computer, or only watches or stops such a fetch, is unknown, and no result or command text offers a download.
//!
//!   cargo test -p lightcraft-engine --no-default-features --test katwork_profile
#![cfg(not(feature = "network"))]

use lightcraft_engine::lensfun_download::{self, Offer, Start};
use lightcraft_engine::{EngineError, Session};
use serde_json::{Value, json};

/// The commands that start a download (the denoise, face and SAM 3 models, the GeoNames place names, the lensfun
/// lens database), and the ones that only watch or stop a download. `denoise.models.downloads` stays: it also
/// reports the models being installed from a file.
const GONE: &[&str] = &[
    "denoise.models.download",
    "denoise.models.downloadCancel",
    "faces.models.download",
    "faces.models.downloads",
    "faces.models.downloadCancel",
    "segment.model.download",
    "segment.model.cancel",
    "metadata.downloadPlaceNames",
    "lens.downloadDatabase",
];

#[test]
fn downloads_are_unknown() {
    let mut s = Session::new();
    let listed: Vec<&str> = s.commands().iter().map(|c| c.id).collect();
    assert!(listed.contains(&"denoise.models.downloads"), "the install progress stays");
    for id in GONE {
        assert!(!listed.contains(id), "{id} is in the command list");
        // the parameters a real download takes, so only the missing command can refuse it
        let e = s.execute(id, &json!({"id": "yunet-2023mar", "acknowledged": true})).unwrap_err();
        assert!(matches!(e, EngineError::UnknownCommand(_)), "{id}: {e}");
    }
}

/// The keys of `v` (an object) that a panel would read to offer or show a download.
fn download_keys(v: &Value, keys: &[&str]) -> Vec<String> {
    keys.iter().filter(|k| v.get(**k).is_some()).map(|k| (*k).to_string()).collect()
}

#[test]
fn no_result_or_text_offers_a_download() {
    let mut s = Session::new();
    // no command names a command this build left out (`faces.detect` and Find Blemishes pointed to the download)
    for c in s.commands() {
        for gone in GONE {
            assert!(!c.params.contains(gone) && !c.label.contains(gone), "{} names {gone}: {}", c.id, c.params);
        }
    }
    // the model lists: a row with `downloadHost` gets a Download button
    for list in ["denoise.models.list", "faces.models.list"] {
        let v = s.execute(list, &json!({})).unwrap();
        let rows = v["models"].as_array().cloned().unwrap_or_default();
        // (the face models are always listed; denoise lists none unless a model is known to this build)
        assert!(!rows.is_empty() || list == "denoise.models.list", "{list} lists the known models");
        for row in rows {
            assert!(download_keys(&row, &["downloadHost"]).is_empty(), "{list}: {row}");
        }
    }
    let status = |s: &mut Session, id: &str| s.execute(id, &json!({})).unwrap();
    let sam = status(&mut s, "segment.model.status");
    assert_eq!(download_keys(&sam, &["download", "mirrors", "remote"]), Vec::<String>::new(), "{sam}");
    let lens = status(&mut s, "lens.databaseStatus");
    assert_eq!(download_keys(&lens, &["download"]), Vec::<String>::new(), "{lens}");
    let places = status(&mut s, "metadata.placeNamesStatus");
    assert_eq!(download_keys(&places, &["downloading", "lastDownload", "lastError"]), Vec::<String>::new(), "{places}");
    let denoise = status(&mut s, "denoise.models.downloads");
    assert_eq!((denoise["running"].as_bool(), denoise["downloads"].as_array().map(Vec::len)), (Some(false), Some(0)), "{denoise}");
    // a file the denoiser does not know: the answer does not point to a download
    let file = std::env::temp_dir().join(format!("lc-katwork-unknown-{}.bin", std::process::id()));
    std::fs::write(&file, b"not a model").unwrap();
    let e = s.execute("denoise.models.inspect", &json!({"path": file})).unwrap_err().to_string();
    let _ = std::fs::remove_file(&file);
    assert!(!GONE.iter().any(|g| e.contains(g)), "{e}");
    // the lens database is never offered or fetched at start
    let dir = std::env::temp_dir().join("lc-katwork-lensfun");
    assert_eq!(lensfun_download::decide(true, false, Some(dir), false, Offer::Ask), Start::Nothing);
    assert!(lensfun_download::mirrors().is_empty());
}
