# Contributing to Ghost (gst)

Thank you for your interest in contributing to *Ghost (gst)*! This document provides guidelines to help make the contribution process smooth and efficient for everyone involved. Please take a moment to review this document before contributing.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## Getting Started

1. **Fork** the repository on GitHub.
2. Clone your fork locally:

    ```bash
    git clone https://github.com/your-username/ghost-gst.git
    cd ghost-gst
    ```

3. Create a branch for your feature or bug fix:

    ```bash
    git checkout -b feature/your-feature-name
    ```

## Making Changes

1. Make your changes in your feature branch.
2. Add or update tests as necessary.
3. Ensure your code adheres to the existing style. I use *rustfmt* for formatting.
4. Run the tests (if there are any for that particular feature) to ensure nothing is broken:

    ```bash
    cargo test
    ```

## Committing Your Changes

1. Commit your changes with a descriptive commit message:

    ```bash
    git commit -m "Brief description of your changes"
    ```

2. Push your branch to GitHub:

    ```bash
    git push origin feature/your-feature-name
    ```

## Submitting a Pull Request

1. Go to the *Ghost (gst)* repository on GitHub.
2. Click the **"New pull request"** button.
3. Select your feature branch from the dropdown menu.
4. Add a title and description for your pull request.
5. Click **"Create pull request"**.

## Pull Request Guidelines

- Ensure the PR description clearly describes the problem and solution.
- Reference the relevant issue number if applicable.
- Update any relevant documentation, including comments.

## Coding Conventions

- Write clear, readable, and maintainable code.
- Follow *Rust* best practices and idioms.
- Use meaningful variable and function names.
- Comment your code where necessary, especially for complex logic.

## Reporting Bugs

1. Use the GitHub issue tracker to report bugs.
2. Describe the bug in detail, including steps to reproduce.
3. Include your environment details (OS, Rust version, etc.).

## Suggesting Enhancements

1. Use the GitHub issue tracker to suggest enhancements.
2. Provide a clear and detailed explanation of the feature and its benefits.

## Questions?

If you have any questions or need further clarification, please open an issue or reach out to the maintainer.

Thank you for contributing to *Ghost (gst)*!
