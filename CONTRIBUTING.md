# Contributing to *jot*

Thank you for your interest in contributing to ***jot***. This file otulines the guidelines for bug reports, feature requests, and code contributions.

## Reporting Bugs

If you encounter a bug, please [open a new issue](https://github.com/araekiel/jot/issues/new/choose) on GitHub. Please include a clear and detailed description of the bug, steps to reproduce it, and any relevant error messages or screenshots.

## Requesting Features
If you have an idea for a new feature, please [open a new issue](https://github.com/araekiel/jot/issues/new/choose) on GitHub. Please include a clear and detailed description of the feature, how it would benefit the project, and any other relevant details.

## Contributing Code

#### *Set up your fork:*

- [Fork the repository](https://github.com/araekiel/jot/fork) to your account.
- Clone the forked repository to your local machine.
- Build/compile the program:

```bash
$ cargo build 
```

- Run the program using the executable created in '***target/debug/***' or, run it directly (Pass in commands and arguments after '***--***'):

```bash
$ cargo run -- *args*
```

#### *Make your changes:*

- Create a new branch for your changes:

```bash
$ git checkout -b *my-branch*
```

- Make your changes, then format and lint your code:

```bash
$ cargo lint
$ cargo fmt
```

- When you are satisfied with your changes, commit them (Please follow [commit message guidelines](#commit-message-guidelines) when writing your commit messages):

```bash'
git commit -m "add: *description*"
```

#### *Open a pull request:*

- Push your changes to your forked repository:

```bash
$ git push origin *my-branch*
```

- On GitHub, open a pull request with your commit(s).

Please include a clear and detailed description of your changes in the pull request. If your changes are related to an issue, please reference the issue number in the pull request title or description.

## Commit Message Guidelines

These guidelines are to ensure a consistent commit history. Following them is highly appreciated!

Write your commits in the following structure: 

```bash
$ git commit -m "action: description"
```

There are 4 possible actions:

- ***`add:`*** When adding a feature or file to the project.
- ***`remove:`*** When removing a feature or file from the project.
- ***`fix:`*** When fixing a bug. Mention the issue tag in description.
- ***`update:`*** When your changes don't quite fall in the other categories.

Try to keep your commit description concise. Add other relevant information in the commit body.

## Contact

If you have any concerns about the project, feel free to [open an issue](https://github.com/araekiel/jot/issues/new/choose) or email me to get in touch. I should respond within 24 hours.

Thank you for taking the time to read these guidelines. I hope you enjoy contributing to ***jot***!
