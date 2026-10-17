# Contributing

Read [ARCHITECTURE.md](ARCHITECTURE.md) first; it explains how the pieces fit and why some code looks odd.

## Setting up

You need [Rust](https://rustup.rs), [Bun](https://bun.sh) and the [Tauri prerequisites for Linux](https://tauri.app/start/prerequisites/#linux) (WebKitGTK 4.1). Then:

```sh
bun install
bun run tauri dev -- -- /path/to/config.seb
```

`bun install` also installs the Git hooks from `lefthook.yml`. Skip them for one commit with `git commit --no-verify`.

## Building packages

```sh
bun run tauri build
```

The `.deb` and `.rpm` packages end up in `src-tauri/target/release/bundle/`. Installing either one registers pantomim for `.seb` files and `seb://` / `sebs://` links.

## Testing against Moodle

You can make your own SEB quiz on the public [Moodle Sandbox](https://sandbox.moodledemo.net). Its login page lists the demo accounts. Log in as `admin`, create a course, enrol the `student` account, and add a quiz with at least one question. In the quiz's Safe Exam Browser settings, choose "Configure manually". Then log in as `student`, open the quiz and click "Launch Safe Exam Browser". The sandbox wipes itself every hour on the hour.

Or let `scripts/sandbox_setup.py` set one up. It only needs Python and takes a few minutes, since the sandbox is slow:

```sh
python3 -I scripts/sandbox_setup.py
```

It creates the course, the quiz and four questions, then prints the command that opens the quiz in pantomim with the demo student already logged in. That login lives in a separate profile in `/tmp`, so your own pantomim cookies stay untouched. Add `--quit-url` or `--quit-password` to try those settings too.

## Before you open a pull request

The hooks check formatting, types, clippy and the tests on commit and push. They can't see the UI, so if you changed something the user sees, run it and look at it: a green test run doesn't show a dialog at the wrong size.

## Changing behavior

pantomim copies SEB for Windows, so start there: find the C# code for what you're changing in [seb-win-refactoring](https://github.com/SafeExamBrowser/seb-win-refactoring) and do what it does.

Anything the exam server can see belongs in the matching `docs/spec/*.md`, together with a test in `src-tauri/tests/`. Take the expected values from SEB, from its tests, or from what Moodle accepts. A test whose expected value came from pantomim's own output proves nothing.

UI texts come word for word from SEB's `id.xml` and `en.xml`.

## Code style

`cargo fmt` formats the Rust code and Biome formats `src/`. Files carry no license header; the license lives in `LICENSE` and `NOTICE`. Write a comment only when the code can't explain itself, usually to say why or to name the SEB code it follows.

Keep commits small. Titles follow [Conventional Commits](https://www.conventionalcommits.org), such as `feat:` or `fix:`, and the commit-msg hook checks them. A few bullet points underneath say what changed.

## License

pantomim is licensed under the [Mozilla Public License 2.0](LICENSE). By contributing, you agree that your contribution is licensed under it as well.
