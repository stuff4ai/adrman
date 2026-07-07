#!/usr/bin/env python3
"""Write a GitHub Actions job summary for adrman CI."""

from __future__ import annotations

import os
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

REPORT_DIR = Path("ci-report")
SUMMARY_PATH = os.environ.get("GITHUB_STEP_SUMMARY")
NEXTTEST_LOG = REPORT_DIR / "nextest.log"
JUNIT_PATH = REPORT_DIR / "junit.xml"
COVERAGE_SUMMARY = REPORT_DIR / "coverage-summary.txt"


def status_label(outcome: str | None) -> str:
    if outcome == "success":
        return "pass"
    if outcome == "skipped":
        return "skipped"
    return "fail"


def parse_nextest_summary(log_path: Path) -> dict[str, str] | None:
    if not log_path.is_file():
        return None

    pattern = re.compile(
        r"Summary\s+\[\s*([^\]]+)\]\s+(\d+)\s+tests run:\s+"
        r"(\d+)\s+passed,\s+(\d+)\s+failed,\s+(\d+)\s+skipped"
    )
    for line in reversed(log_path.read_text(encoding="utf-8", errors="replace").splitlines()):
        match = pattern.search(line)
        if match:
            duration, total, passed, failed, skipped = match.groups()
            return {
                "duration": duration.strip(),
                "total": total,
                "passed": passed,
                "failed": failed,
                "skipped": skipped,
            }
    return None


def parse_coverage_summary(summary_path: Path) -> dict[str, str] | None:
    if not summary_path.is_file():
        return None

    for line in summary_path.read_text(encoding="utf-8", errors="replace").splitlines():
        if line.startswith("TOTAL"):
            percentages = re.findall(r"([\d.]+%)", line)
            if len(percentages) >= 3:
                return {
                    "regions": percentages[0],
                    "functions": percentages[1],
                    "lines": percentages[2],
                }
            if len(percentages) == 1:
                return {"regions": percentages[0], "functions": "n/a", "lines": "n/a"}
    return None


def slowest_tests(junit_path: Path, limit: int = 5) -> list[tuple[str, float]]:
    if not junit_path.is_file():
        return []

    try:
        root = ET.parse(junit_path).getroot()
    except ET.ParseError:
        return []

    timings: list[tuple[str, float]] = []
    for testcase in root.iter("testcase"):
        name = testcase.attrib.get("name", "unknown")
        classname = testcase.attrib.get("classname", "")
        full_name = f"{classname}::{name}" if classname else name
        try:
            duration = float(testcase.attrib.get("time", "0"))
        except ValueError:
            duration = 0.0
        timings.append((full_name, duration))

    timings.sort(key=lambda item: item[1], reverse=True)
    return timings[:limit]


def artifact_note() -> str:
    run_url = (
        f"{os.environ.get('GITHUB_SERVER_URL', '')}/"
        f"{os.environ.get('GITHUB_REPOSITORY', '')}/actions/runs/"
        f"{os.environ.get('GITHUB_RUN_ID', '')}"
    )
    if run_url.endswith("/actions/runs/"):
        return "Download the `ci-report` artifact from this workflow run."
    return (
        f"Download the [`ci-report`]({run_url}) artifact from this workflow run "
        "(Artifacts section at the bottom of the run page)."
    )


def main() -> int:
    fmt = status_label(os.environ.get("FMT_OUTCOME"))
    clippy = status_label(os.environ.get("CLIPPY_OUTCOME"))
    tests = status_label(os.environ.get("TESTS_OUTCOME"))
    overall = "pass" if all(
        status == "pass" for status in (fmt, clippy, tests)
    ) else "fail"

    lines = [
        "# CI report",
        "",
        f"## Overall CI result: **{overall}**",
        "",
        "| Check | Result |",
        "| --- | --- |",
        f"| Formatting (`cargo fmt`) | {fmt} |",
        f"| Linting (`cargo clippy`) | {clippy} |",
        f"| Tests and coverage (`cargo llvm-cov nextest`) | {tests} |",
        "",
    ]

    test_stats = parse_nextest_summary(NEXTTEST_LOG)
    if test_stats:
        lines.extend(
            [
                "## Test statistics",
                "",
                f"- Duration: {test_stats['duration']}",
                f"- Total: {test_stats['total']}",
                f"- Passed: {test_stats['passed']}",
                f"- Failed: {test_stats['failed']}",
                f"- Skipped: {test_stats['skipped']}",
                "",
            ]
        )
    else:
        lines.extend(["## Test statistics", "", "_No nextest summary was captured._", ""])

    coverage = parse_coverage_summary(COVERAGE_SUMMARY)
    if coverage:
        lines.extend(
            [
                "## Coverage summary",
                "",
                f"- Regions: {coverage['regions']}",
                f"- Functions: {coverage['functions']}",
                f"- Lines: {coverage['lines']}",
                "",
            ]
        )
    else:
        lines.extend(["## Coverage summary", "", "_No coverage summary was generated._", ""])

    slow_tests = slowest_tests(JUNIT_PATH)
    if slow_tests:
        lines.extend(["## Slowest tests", ""])
        for name, duration in slow_tests:
            lines.append(f"- `{name}` ({duration:.3f}s)")
        lines.append("")

    lines.extend(
        [
            "## Artifacts",
            "",
            artifact_note(),
            "",
            "Uploaded files:",
            "",
            "- `junit.xml` — JUnit test report",
            "- `lcov.info` — LCOV coverage report",
            "- `coverage-html/` — HTML coverage report",
            "- `coverage-summary.txt` — text coverage summary",
            "- `nextest.log` — nextest console output",
            "",
        ]
    )

    report = "\n".join(lines)
    if SUMMARY_PATH:
        Path(SUMMARY_PATH).write_text(report, encoding="utf-8")
    else:
        print(report)

    return 0


if __name__ == "__main__":
    sys.exit(main())
