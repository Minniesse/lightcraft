//! The build without the `network` feature (KatWork's): every command that would fetch something from another
//! computer is unknown, and the status and cancel commands beside them say nothing is running.
//!
//!   cargo test -p lightcraft-engine --no-default-features --test katwork_profile
#![cfg(not(feature = "network"))]

use lightcraft_engine::{EngineError, Session};
use serde_json::json;

/// The commands that start a download: the denoise, face and SAM 3 models, the GeoNames place names and the
/// lensfun lens database.
const DOWNLOADS: &[&str] =
    &["denoise.models.download", "faces.models.download", "segment.model.download", "metadata.downloadPlaceNames", "lens.downloadDatabase"];

#[test]
fn downloads_are_unknown() {
    let mut s = Session::new();
    let listed: Vec<&str> = s.commands().iter().map(|c| c.id).collect();
    for id in DOWNLOADS {
        assert!(!listed.contains(id), "{id} is in the command list");
        // the parameters a real download takes, so only the missing command can refuse it
        let e = s.execute(id, &json!({"id": "yunet-2023mar", "acknowledged": true})).unwrap_err();
        assert!(matches!(e, EngineError::UnknownCommand(_)), "{id}: {e}");
    }
}

#[test]
fn the_download_status_commands_report_nothing_running() {
    let mut s = Session::new();
    let denoise = s.execute("denoise.models.downloads", &json!({})).unwrap();
    assert_eq!((denoise["running"].as_bool(), denoise["downloads"].as_array().map(Vec::len)), (Some(false), Some(0)), "{denoise}");
    let faces = s.execute("faces.models.downloads", &json!({})).unwrap();
    assert_eq!((faces["running"].as_bool(), faces["downloads"].as_array().map(Vec::len)), (Some(false), Some(0)), "{faces}");
    let lens = s.execute("lens.databaseStatus", &json!({})).unwrap();
    assert_eq!(lens["download"]["state"], "idle", "{lens}");
    assert_eq!(s.execute("segment.model.cancel", &json!({})).unwrap()["cancelled"], false);
}
