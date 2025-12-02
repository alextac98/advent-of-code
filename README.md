# Advent of Code

This is my [Advent of Code](https://adventofcode.com/) repository that I use to solve all the AoC challenges I come across. It is split up by year and day of the challenges. I use AoC to learn new skills, so this code should not be used for reference or learning purposes!

## Prerequisites

- [Bazel](https://bazel.build/) (or [Bazelisk](https://github.com/bazelbuild/bazelisk))

## New Day Setup

Use the generator script to create new day(s):

```bash
# Generate all 25 days for the current year
bazel run //:generate_days

# Generate specific year
bazel run //:generate_days -- -y 2024

# Generate specific number of days
bazel run //:generate_days -- -y 2025 -d 10
```

## Building and Running

```bash
# Build a specific day
bazel build //2025/01:main

# Run a specific day
bazel run //2025/01:main

# Build all targets
bazel build //...
```

## Regenerating the Python requirements.txt

After changing the `requirements.in` file, you'll need to regenerate the `requirements.txt` with this command:

```bash
bazel run //tools/python:generate_requirements_txt
```
