# KatWork fork of LightCraft

This repository is the KatWork fork of LightCraft (https://github.com/storytold/lightcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on LightCraft by the ArtCraft team (https://github.com/storytold/lightcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `ddc0eef377a771a51f3574f8c26ef75be12c7aab`
- Upstream `storytold/lightcraft` head at that time: `ea033fbe1c6dacdc7e1e99ee5a3e9fa0ce35d78c`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): network feature, on by default` | draft, not opened (in the fork PR body) |

## The KatWork build

KatWork builds `lightcraft-cli` with `--no-default-features --features denoise` and `lightcraft-web` (through
`cargo xtask web --no-default-features`) with no features. Cargo features, on by default so an upstream build
is unchanged, are then off:

- `network` (in `lightcraft-engine`, `lightcraft-segment` and `lightcraft-denoise`, forwarded by `lightcraft-mcp`,
  `lightcraft-ui-egui`, the CLI, the web crate and the desktop app): `lightcraft-fetch` (rustls, webpki-roots) leaves
  the build. The commands that start a download are unknown: `denoise.models.download`, `faces.models.download`,
  `segment.model.download`, `metadata.downloadPlaceNames` and `lens.downloadDatabase`; their status and cancel
  commands stay and report nothing running. The remote denoise client (`LIGHTCRAFT_DENOISE_URL`), the
  `denoise-serve` subcommand (unknown) and the SAM 3 remote worker's client and server are compiled out.

`denoise` stays on (the engine behaves like upstream apart from the network); `rawnind-model`, `heif` and `dhat-heap`
stay off (none is a default). `scripts/katwork-profile.sh` (CI job `katwork-profile`) checks the graphs and runs
`crates/engine/tests/katwork_profile.rs` with the gate off. The KatWork lock check must refuse `network` in its `features` and `web_features` columns.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/lightcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.lightcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The app icon's provenance (`assets/app-icon/LICENSE.txt`, `assets/app-icon/README.md`, `assets/ATTRIBUTION.md`): the
  owner drew the lynx in the ArtCraft drawing app; that is a fact of the licence and attribution record, not a mark.
- The flatpak permission for ArtCraft Launcher's version folder (`xdg-data/ArtCraft/versions`) and the generated copy of
  `craft-update`'s manifest in `packaging/linux/flatpak/cargo-sources.json`: they name a separate program (the upstream
  launcher) and a dependency's own description; KatWork builds no flatpak.
- An upstream engineering note (`docs/egui-037-migration-plan.md`) that names another upstream program, ArtCraft Native.
- Upstream CI configuration (`release.yml`'s secret name, `windows-arm64.yml`'s install path): unchanged; KatWork does not
  run these workflows.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.lightcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.lightcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/lightcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/lightcraft-macos-512.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/lightcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/lightcraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/lightcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `assets/app-icon/lightcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt`
- `crates/codecs/tests/fixtures/quad-444-gbr-lossless.avif`: kept: test fixture (four flat colour quadrants; viewed, no ArtCraft mark)
- `docs/images/ba-blue-marble.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/ba-earthrise.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/ba-migrant-mother.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/ba-tetons.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/bw-split.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/crop-tetons.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/curve-tetons.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/grading-migrant-mother.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/grid-demo.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/grid-pd.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/hero-tetons.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/info-earthrise.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/masking.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `docs/images/presets-mixer.jpg`: kept: screenshot of the upstream build (viewed: no ArtCraft mark; its top bar predates the Discord button)
- `packaging/macos/dmg/background.svg`: kept: original artwork (the macOS disk-image background: the app icon's lynx and the name "LightCraft" as text; viewed, no ArtCraft mark), credited in `assets/ATTRIBUTION.md`
- `packaging/macos/dmg/background.tiff`: kept: original artwork (rendered from `background.svg`; viewed, no ArtCraft mark), credited in `assets/ATTRIBUTION.md`
