#!/usr/bin/env python3
"""
Advent of Code Day Generator Script
Generates Bazel project directories for each day of Advent of Code
"""

import argparse
import os
import sys
from datetime import datetime
from pathlib import Path

from jinja2 import Environment, FileSystemLoader


def get_workspace_dir() -> Path:
    """Get the workspace directory, handling both direct execution and Bazel run."""
    # Check if running under Bazel
    build_workspace = os.environ.get("BUILD_WORKSPACE_DIRECTORY")
    if build_workspace:
        return Path(build_workspace)
    # Fallback: assume script is in workspace root
    return Path(__file__).parent.resolve()


def get_templates_dir() -> Path:
    """Get the templates directory, handling both direct execution and Bazel run."""
    # Check if running under Bazel with runfiles
    runfiles_dir = os.environ.get("RUNFILES_DIR")
    if runfiles_dir:
        return Path(runfiles_dir) / "advent_of_code" / "templates"
    # Fallback: assume templates are relative to script
    return Path(__file__).parent.resolve() / "templates"


def main():
    parser = argparse.ArgumentParser(
        description="Generate Advent of Code Bazel project directories.",
        epilog="""
Examples:
  %(prog)s                    # Generate all 25 days for current year
  %(prog)s -y 2024 -d 10      # Generate days 1-10 for 2024
  %(prog)s --year 2025        # Generate all 25 days for 2025

Build commands:
  bazel build //YEAR/DAY:main    # Build a specific day
  bazel run //YEAR/DAY:main      # Run a specific day
  bazel build //...              # Build all targets
        """,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "-y", "--year",
        type=int,
        default=datetime.now().year,
        help="Year for Advent of Code (default: current year)",
    )
    parser.add_argument(
        "-d", "--days",
        type=int,
        default=25,
        help="Total number of days to generate (default: 25)",
    )

    args = parser.parse_args()

    # Validate inputs
    if not (1000 <= args.year <= 9999):
        print("Error: Year must be a 4-digit number", file=sys.stderr)
        sys.exit(1)

    if not (1 <= args.days <= 25):
        print("Error: Days must be a number between 1 and 25", file=sys.stderr)
        sys.exit(1)

    # Setup paths
    workspace_dir = get_workspace_dir()
    templates_dir = get_templates_dir()
    year_dir = workspace_dir / str(args.year)

    # Setup Jinja2 environment
    env = Environment(
        loader=FileSystemLoader(templates_dir),
        keep_trailing_newline=True,
    )

    print(f"🎄 Advent of Code {args.year} - Generating {args.days} day(s)")
    print("=" * 50)

    # Create year directory if it doesn't exist
    if not year_dir.exists():
        year_dir.mkdir(parents=True)
        print(f"📁 Created year directory: {args.year}/")

        # Create year-level BUILD.bazel
        template = env.get_template("year_build.bazel.j2")
        (year_dir / "BUILD.bazel").write_text(template.render(year=args.year))

    # Generate each day
    created_count = 0
    skipped_count = 0

    for day in range(1, args.days + 1):
        day_padded = f"{day:02d}"
        day_dir = year_dir / day_padded

        if day_dir.exists():
            print(f"⏭️  Day {day_padded}: Already exists, skipping")
            skipped_count += 1
            continue

        print(f"🔨 Day {day_padded}: Creating project...")

        # Create directory structure
        (day_dir / "src").mkdir(parents=True)

        # Create BUILD.bazel
        template = env.get_template("day_build.bazel.j2")
        (day_dir / "BUILD.bazel").write_text(
            template.render(year=args.year, day=day_padded)
        )

        # Create main.rs
        template = env.get_template("main.rs.j2")
        (day_dir / "src" / "main.rs").write_text(
            template.render(year=args.year, day=day_padded)
        )

        # Create empty input.txt
        (day_dir / "input.txt").touch()

        created_count += 1

    print("=" * 50)
    print("✅ Done!")
    print(f"   Created: {created_count} day(s)")
    print(f"   Skipped: {skipped_count} day(s) (already existed)")


if __name__ == "__main__":
    main()
