# Check your download

The Trindle image comes with two more files next to it:

- `SHA256SUMS`: the image's file name and its SHA-256, one line, in the form `<hash>  trindle-0.9.0-beta.1-amd64.iso`.
- `SHA256SUMS.sig`: a signature of that file, made with the Trindle Archive key.

The key's fingerprint is:

```text
DC23 9DC3 4F02 A6A6 D569  E83B 6D17 DFB7 011D D6AA
```

The same key signs Trindle's updates. Its public half is in `system/apt/trindle-archive-keyring.gpg` in the source, and on an installed Trindle at `/usr/share/keyrings/trindle-archive-keyring.gpg`.

## Check it

Put the three files in one folder, then:

```bash
gpg --import trindle-archive-keyring.gpg
gpg --fingerprint DC239DC34F02A6A6D569E83B6D17DFB7011DD6AA
gpg --verify SHA256SUMS.sig SHA256SUMS
sha256sum -c SHA256SUMS
```

Compare the fingerprint that `gpg --fingerprint` prints with the one above, character by character. `gpg --verify` must say `Good signature`, and `sha256sum -c` must say `trindle-0.9.0-beta.1-amd64.iso: OK`. If either says otherwise, do not use the image.

## Making them (for the project)

After the image is cleared, in the folder holding it:

```bash
sha256sum trindle-0.9.0-beta.1-amd64.iso > SHA256SUMS
GNUPGHOME="$TRINDLE_KEY_HOME" gpg --batch --detach-sign --local-user DC239DC34F02A6A6D569E83B6D17DFB7011DD6AA --output SHA256SUMS.sig SHA256SUMS
```

The private half stays where `docs/apt-key-rotation.md` says and is never put in a repository.
