# Archive key rotation

Trindle's update archive (apt.trindle.org) is signed with one ed25519 key, fingerprint `DC23 9DC3 4F02 A6A6 D569 E83B 6D17 DFB7 011D D6AA` (`sh tools/apt/key.sh fingerprint`). It now expires two years after it was last renewed (`sh tools/apt/key.sh expires`; today 6 November 2028).

## Where it lives

- Public half: `system/apt/trindle-archive-keyring.gpg` in this repository. `tools/system-files.sh` installs it on every computer as `/usr/share/keyrings/trindle-archive-keyring.gpg`, and `system/apt/trindle.sources` names it in `Signed-By`.
- Private half: `"$TRINDLE_KEY_HOME"` in the project's private key folder (backed up with the other secrets). It is never in the repository.
- `tools/apt/build.sh` signs with it and `tools/apt/verify.sh` checks against the public half. `verify.sh` refuses to pass, and so `publish.sh` puts nothing up, once the key is within 60 days of expiry.

## Who and how often

The Trindle project session renews it, on the project owner's word, at least six months before the expiry date, and always in the release that comes nearest to that. Extending an expiry keeps the fingerprint, so nothing in `Signed-By` or in `key.sh` changes.

## Renewing (same key, new date)

1. `GNUPGHOME="$TRINDLE_KEY_HOME" gpg --quick-set-expire DC239DC34F02A6A6D569E83B6D17DFB7011DD6AA 2y`
2. `GNUPGHOME="$TRINDLE_KEY_HOME" gpg --export DC239DC34F02A6A6D569E83B6D17DFB7011DD6AA > system/apt/trindle-archive-keyring.gpg`
3. Run `sh tools/apt/key.sh expires`, build and verify the archive, commit, and publish a release.

## How computers get the new key

Computers update `/usr/share/keyrings/trindle-archive-keyring.gpg` through the `trindle` package, which the old key still signs. So a renewal must reach a published release before the old date, or computers that skipped it would reject the archive.

## If the key is lost or leaked

Make a new key in the same secrets folder, ship its public half in a `trindle` release signed by the old key (if the old key still works), and say so on the website. If the old key is gone for good, computers need the new keyring by hand: say how on the website first.
