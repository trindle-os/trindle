# Contributing

Thank you for being interested in Trindle. It is free software and you are free to use it, change it and fork it under the terms of the [licence](LICENSE).

## Reporting a bug

Report bugs at [trindle.buildmorphic.com/report](https://trindle.buildmorphic.com/report) and ask questions on the [Buildmorphic community](https://trindle.buildmorphic.com).

A useful report says:

- what you did,
- what you expected,
- what happened instead,
- your Trindle version and the computer you use.

The ring has a **Report a problem** button that writes this up for you. It shows exactly what the report holds, blanks out your name and home folder, and sends nothing by itself.

## Changes to the code

Pull requests are welcome on GitHub. Please open an issue first, or say in the pull request what it fixes or adds, so nobody builds something that cannot be taken.

- Keep one change to a pull request, small enough to read in one sitting.
- Run `cargo test --release` and `cargo clippy --release -- -D warnings` before you send it. Each pull request is checked by them again.
- Sign off each commit (`git commit -s`): that says you wrote it, or have the right to give it, under the project's licence.
- The maintainers read your change, and when it is taken, put it into Trindle's own working copy and publish it from there, so the history here stays one line. Your change is credited to you in that commit with a Co-authored-by line.
- A change is taken when it matches how Trindle is built, described in [docs/how-trindle-is-built.md](docs/how-trindle-is-built.md). One that goes another way is declined with a reason, and forking Trindle for that is welcome.

## Sign-off

By signing off a commit (`git commit -s`) you say, under the Developer Certificate of Origin 1.1:

1. The contribution was created in whole or in part by you and you have the right to submit it under the project's open source licence; or
2. it is based on earlier work that is under a suitable open source licence and you have the right to submit it with changes under that licence; or
3. it was given to you by someone who certified (1) or (2) and you have not changed it; and
4. you understand the contribution and a record of it, including your name and email, is kept publicly and may be redistributed under the project's licence.

There is no other agreement to sign.

## Security problems

Please report these privately. See [SECURITY.md](SECURITY.md).

## Building and testing

The steps are in the "Building it" section of [README.md](README.md). Please run `cargo test --release` before you send anything.

## Contact

[distro@trindle.org](mailto:distro@trindle.org)
