//! A build without the `network` and `self-update` features (KatWork's) shows no control for what it left out: no
//! Check for Updates, no download buttons for denoise or face models, no lens database download or offer.

use serde_json::json;

#[cfg(not(feature = "network"))]
use crate::headless::Headless;
#[cfg(not(feature = "self-update"))]
use crate::menubar::MenuNode;
use crate::{LightcraftApp, Services};

#[cfg(not(feature = "network"))]
const T: std::time::Duration = std::time::Duration::from_secs(20);

/// Every item id of the menu bar, submenus included.
#[cfg(not(feature = "self-update"))]
fn menu_ids(app: &LightcraftApp) -> Vec<String> {
    fn walk(nodes: &[MenuNode], out: &mut Vec<String>) {
        for n in nodes {
            match n {
                MenuNode::Item { id, .. } => out.push(id.clone()),
                MenuNode::Submenu { children, .. } => walk(children, out),
                MenuNode::Separator => {}
            }
        }
    }
    let mut out = Vec::new();
    for (_, items) in crate::menubar::menu_bar(app) {
        walk(&items, &mut out);
    }
    out
}

#[cfg(not(feature = "self-update"))]
#[test]
fn check_for_updates_is_not_offered() {
    let mut app = LightcraftApp::new(lightcraft_engine::Session::with_demo(), Services::default());
    assert!(menu_ids(&app).iter().any(|id| id == "app.about"), "the menu bar was read");
    assert!(!crate::menus::ui_commands().any(|c| c.0 == "app.checkForUpdates"), "a UI command");
    assert!(!menu_ids(&app).iter().any(|id| id == "app.checkForUpdates"), "a menu item");
    assert!(app.run("app.checkForUpdates", json!({})).is_err(), "it still runs");
}

/// The Settings window open on `tab`, drawn until it settled; the widgets it registered.
#[cfg(not(feature = "network"))]
fn settings_widgets(h: &mut Headless, tab: &str) -> Vec<String> {
    let r = h.request("engine.execute", json!({"command": "app.settings", "params": {"tab": tab}}), T);
    assert_eq!(r["ok"], true, "app.settings {tab}: {r}");
    h.settle(T);
    for _ in 0..4 {
        h.step();
    }
    h.app.widgets.iter().map(|(id, _)| id.clone()).collect()
}

#[cfg(not(feature = "network"))]
#[test]
fn settings_offer_no_download() {
    let mut h = Headless::new(LightcraftApp::new(lightcraft_engine::Session::with_demo(), Services::default()), [1600.0, 1000.0], 1.0);
    // a folder for face models, so the Faces tab lists them
    let models = std::env::temp_dir().join(format!("lc-katwork-models-{}", std::process::id()));
    h.app.session.face_models_dir = Some(models.clone());
    h.settle(T);
    let general = settings_widgets(&mut h, "general");
    assert!(general.iter().any(|w| w.starts_with("check:settings.")), "the General tab was drawn: {general:?}");
    for gone in ["button:settingsLensDatabase", "check:settings.lensDatabaseOffer"] {
        assert!(!general.iter().any(|w| w == gone), "{gone} is shown");
    }
    for (tab, prefix) in [("denoise", "denoise:"), ("faces", "faces:")] {
        let widgets = settings_widgets(&mut h, tab);
        let models: Vec<&String> = widgets.iter().filter(|w| w.starts_with(prefix)).collect();
        // (the face models are always listed; the denoise tab lists none unless a model is known to this build)
        assert!(tab == "denoise" || models.iter().any(|w| w.contains(":get:")), "the {tab} tab lists its models: {models:?}");
        assert!(!models.iter().any(|w| w.contains(":download:") || w.contains(":cancelDownload:")), "{tab}: {models:?}");
    }
    assert!(lightcraft_engine::lensfun_download::mirrors().is_empty(), "nowhere to download the lens database from");
    let _ = std::fs::remove_dir_all(&models);
}
