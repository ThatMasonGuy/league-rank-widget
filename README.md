# League Rank Widget

A small Tauri desktop overlay for tracking configured League of Legends ranks.

## Development

```powershell
npm install
npm run tauri dev
```

The widget gets its current rank data from `https://lol.mxn.au/ranks`. Its activity states are driven by the worker response: `recently_played` indicates a completed game in the last hour, while a recent confirmed `in_game` result shows **Active now**.

## Releases and in-app updates

Pushing a version tag in the form `vX.Y.Z` runs `.github/workflows/release.yml`. It builds the Windows NSIS installer, signs its updater package, creates a GitHub Release, and uploads the Tauri-generated `latest.json` metadata used by the app's Update button.

Keep `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml` on the same version before creating a release:

```powershell
git tag v1.0.0
git push origin v1.0.0
```

`TAURI_SIGNING_PRIVATE_KEY` is a GitHub Actions secret. Its public counterpart is intentionally embedded in `src-tauri/tauri.conf.json`; never rotate it after users have installed a release unless you are deliberately starting a new update channel. Back up the private key outside this repository.

The public `latest.json` endpoint is served from this repository's latest GitHub Release. Keep the repository and release assets public so the updater can download and validate new versions on users' machines.
