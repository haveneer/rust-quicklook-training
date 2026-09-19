#!/usr/bin/env python3
"""Generate the slide figures from measurements, driven by `benches/figures.json`.

Two kinds of figures, two data sources:

* `bars`   — horizontal bars, readable when projected, read from
             `target/criterion/<case>-<variant>/new/estimates.json` (`cargo bench`)
* `vbars`  — vertical bars + standard deviation, one bar per bench addressed by its
             `target/criterion` path; or, with `data`, from a `<name> <value>` text report
             (gungraun instruction counts, which do not go through criterion)
* `curves` — instructions-per-element curves, read from the text report produced by
             callgrind (`<source> <n> <variant> <instructions>`, see `benches/README.md`)

    venv/bin/python benches/plot_figures.py                      # every figure listed in figures.json
    venv/bin/python benches/plot_figures.py --only adapter       # only those whose path contains "adapter"
    venv/bin/python benches/plot_figures.py --discover           # inspection view: one chart per criterion case

The `--discover` mode needs no configuration: it scans every case found in `target/criterion`
and renders vertical bars with standard deviation, to review raw results.
Figures versioned in `images/benchmarks/` go through `figures.json` instead.
"""
import argparse
import collections
import glob
import json
import os

import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
CONFIG = os.path.join(HERE, "figures.json")


# ------------------------------------------------------------------- reading measurements

def read_criterion_case(criterion_dir, case):
    """{variant: (mean_ns, std_dev_ns)} for a `<case>-<variant>` case"""
    found = {}
    for path in glob.glob(f"{criterion_dir}/{case}-*/new/estimates.json"):
        variant = os.path.basename(os.path.dirname(os.path.dirname(path)))[len(case) + 1:]
        with open(path) as f:
            data = json.load(f)
        found[variant] = (data["mean"]["point_estimate"], data["std_dev"]["point_estimate"])
    return found


def read_callgrind_sweep(path):
    """{(source, n): {variant: instructions}} from the text report"""
    out = collections.defaultdict(dict)
    with open(path) as f:
        for line in f:
            parts = line.split()
            if len(parts) != 4:
                continue
            source, n, variant, instructions = parts
            out[(source, int(n))][variant] = int(instructions.replace(",", ""))
    return out


def normalize(name):
    """Loose key: `push/pop-on Vec ` and `push_pop-on Vec` name the same measurement"""
    return "_".join(
        part for part in "".join(c if c.isalnum() else "_" for c in name.lower()).split("_") if part
    )


def read_criterion_bench(criterion_dir, bench):
    """(mean_ns, std_dev_ns) for one bench, addressed by its `target/criterion` path

    `bench` is the directory criterion created, e.g. `create-box` for a plain
    `bench_function("create-box")` or `allocations/Heap allocation (Vec)` inside a
    `benchmark_group("allocations")`. Criterion sanitizes ids: `/` becomes `_` and
    trailing spaces are dropped, so `push/pop-on Vec ` lands in `push_pop-on Vec`.
    """
    path = os.path.join(criterion_dir, bench, "new", "estimates.json")
    if not os.path.exists(path):
        # criterion keeps what it is given: `bench_function("push/pop-on Vec ")` lands in
        # `push_pop-on Vec ` — trailing space included. Match on a normalized form too.
        wanted = normalize(bench)
        for candidate in glob.glob(f"{criterion_dir}/**/new/estimates.json", recursive=True):
            name = os.path.relpath(os.path.dirname(os.path.dirname(candidate)), criterion_dir)
            if normalize(name) == wanted:
                path = candidate
                break
    if not os.path.exists(path):
        available = sorted(
            os.path.relpath(os.path.dirname(os.path.dirname(p)), criterion_dir)
            for p in glob.glob(f"{criterion_dir}/**/new/estimates.json", recursive=True)
        )
        raise SystemExit(
            f"no measurement for '{bench}': run `cargo bench` first.\n"
            f"available benches:\n  " + "\n  ".join(available or ["(none)"])
        )
    with open(path) as f:
        data = json.load(f)
    return data["mean"]["point_estimate"], data["std_dev"]["point_estimate"]


def read_value_table(path):
    """{name: value} from a two-column text report (`<name> <value>`, thousands separators ok)

    Used for gungraun/callgrind instruction counts, which do not go through criterion.
    """
    out = {}
    with open(path) as f:
        for line in f:
            parts = line.split()
            if len(parts) != 2:
                continue
            out[parts[0]] = float(parts[1].replace(",", "").replace("_", ""))
    return out


# Scale factors for a mean expressed in nanoseconds.
UNITS = {"ns": 1, "us": 1e3, "µs": 1e3, "ms": 1e6, "s": 1e9}


def pick_unit(max_value):
    """Largest time unit keeping the biggest bar >= 1"""
    for name in ("s", "ms", "µs"):
        if max_value >= UNITS[name]:
            return name, UNITS[name]
    return "ns", 1


# -------------------------------------------------------------------------------- plotting

def series_color(key, cfg, roles):
    """One colour per measured entity, stable across figures.

    `series_colors` maps a bench to its colour, either by full name or by the part after the
    first `-`, so `create-box`, `clone-box` and `access-box` all get the colour of `box`.
    Falls back to the narrative palette (`roles`: slow / reference) when nothing matches.
    """
    series = cfg.get("series_colors", {})
    if key in series:
        return series[key]
    _, _, suffix = key.partition("-")
    if suffix and suffix in series:
        return series[suffix]
    return cfg["palette"][roles.get(key, "default")]


def draw_bars(fig_cfg, cfg, output):
    case = fig_cfg["case"]
    measures = read_criterion_case(cfg["criterion_dir"], case)
    if not measures:
        raise SystemExit(f"no measurement for '{case}': run `cargo bench` first")

    variants = fig_cfg.get("variants") or sorted(measures)
    missing = [v for v in variants if v not in measures]
    if missing:
        raise SystemExit(f"{case}: variants missing from measurements: {', '.join(missing)}")

    labels = {**cfg.get("labels", {}), **fig_cfg.get("labels", {})}
    roles = fig_cfg.get("roles", {})
    palette = cfg["palette"]

    divisor, suffix = (1000, "µs") if fig_cfg.get("unit", "us") == "us" else (1, "ns")
    names = [labels.get(v, v) for v in variants]
    values = [measures[v][0] / divisor for v in variants]
    colors = [series_color(v, cfg, roles) for v in variants]

    fig, ax = plt.subplots(figsize=(6.4, 0.46 * len(variants) + 0.95))
    bars = ax.barh(names[::-1], values[::-1], color=colors[::-1], height=0.68)
    ax.bar_label(bars, fmt=f"%.1f {suffix}", padding=4, fontsize=10)
    ax.set_xlim(0, max(values) * 1.28)
    # No x-axis caption: the bar labels carry the unit and the slide gives the context.
    ax.set_title(fig_cfg["title"], fontsize=12)
    ax.tick_params(labelsize=10)
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)
    fig.tight_layout()
    fig.savefig(output, dpi=170)
    plt.close(fig)


def draw_curves(fig_cfg, cfg, output):
    data_path = fig_cfg.get("data", cfg.get("data"))
    if not os.path.exists(data_path):
        raise SystemExit(
            f"{data_path} is missing: produce the callgrind report (see benches/README.md)"
        )
    sweep = read_callgrind_sweep(data_path)

    source = fig_cfg["source"]
    sizes = sorted(n for s, n in sweep if s == source)
    if not sizes:
        raise SystemExit(f"no measurement for source '{source}' in {data_path}")

    labels = {**cfg.get("labels", {}), **fig_cfg.get("labels", {})}
    styles = cfg["curve_styles"]
    elements = fig_cfg.get("elements", cfg.get("elements"))
    repeats = fig_cfg.get("repeats", cfg.get("repeats", 1))
    baseline = fig_cfg.get("baseline", "baseline")

    fig, ax = plt.subplots(figsize=(7, 4.5))
    for variant in fig_cfg["series"]:
        style, marker = styles.get(variant, ["-", "o"])
        ys = [
            (sweep[(source, n)][variant] - sweep[(source, n)][baseline]) / repeats / elements
            for n in sizes
        ]
        ax.plot(sizes, ys, style, marker=marker, markersize=8, label=labels.get(variant, variant))

    ax.set_xscale("log", base=2)
    ax.set_xticks(sizes, [str(n) for n in sizes])
    ax.set_ylim(bottom=0)
    ax.set_xlabel(fig_cfg.get("x_label", "block size n"))
    ax.set_ylabel(fig_cfg.get("y_label", "instructions per element"))
    ax.set_title(fig_cfg["title"])
    ax.grid(alpha=0.3)
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(output, dpi=150)
    plt.close(fig)


def draw_vbars(fig_cfg, cfg, output):
    """Vertical bars, one per bench, with a standard-deviation whisker when available.

    Two possible sources: criterion means (`benches`, times in ns) or a two-column text
    report (`data` + `series`, e.g. gungraun instruction counts).
    """
    labels = {**cfg.get("labels", {}), **fig_cfg.get("labels", {})}
    roles = fig_cfg.get("roles", {})
    palette = cfg["palette"]

    if "data" in fig_cfg:
        if not os.path.exists(fig_cfg["data"]):
            raise SystemExit(
                f"{fig_cfg['data']} is missing: produce the instruction report "
                f"(see benches/README.md)"
            )
        table = read_value_table(fig_cfg["data"])
        keys = fig_cfg["series"]
        missing = [k for k in keys if k not in table]
        if missing:
            raise SystemExit(
                f"{fig_cfg['file']}: missing from {fig_cfg['data']}: {', '.join(missing)}"
            )
        values = [table[k] for k in keys]
        errors = None
        suffix, divisor = "", 1
        y_label = fig_cfg.get("y_label", "instructions")
    else:
        keys = fig_cfg["benches"]
        measures = [read_criterion_bench(cfg["criterion_dir"], k) for k in keys]
        unit = fig_cfg.get("unit", "auto")
        if unit == "auto":
            suffix, divisor = pick_unit(max(m for m, _ in measures))
        else:
            suffix, divisor = unit, UNITS[unit]
        values = [m / divisor for m, _ in measures]
        errors = [s / divisor for _, s in measures]
        y_label = fig_cfg.get("y_label", f"Mean time ({suffix})")

    # Default label: the function name, without the criterion group prefix
    # (`accesses/Sequential access on stack (array)` shows as the part after the slash).
    names = [labels.get(k, k.rpartition("/")[2] or k) for k in keys]
    colors = [series_color(k, cfg, roles) for k in keys]

    fig, ax = plt.subplots(figsize=(7, 4.2))
    bars = ax.bar(range(len(keys)), values, yerr=errors, capsize=5, color=colors, width=0.62)
    # Direct labels: the fills alone sit below 3:1 contrast on white. Sit them above the
    # error whisker, not the bar, or a noisy measurement hides its own figure.
    fmt = "%.0f" if max(values) >= 100 else "%.2f"
    tops = [v + (errors[i] if errors else 0) for i, v in enumerate(values)]
    for rect, value, top in zip(bars, values, tops):
        ax.annotate(
            fmt % value,
            (rect.get_x() + rect.get_width() / 2, top),
            textcoords="offset points",
            xytext=(0, 3),
            ha="center",
            fontsize=9,
        )
    ax.set_xticks(range(len(keys)), names, rotation=fig_cfg.get("rotation", 20), ha="right")
    ax.set_ylim(0, max(tops) * 1.12)
    # Chart text is rendered into the French slides, hence the French wording here.
    ax.set_ylabel(y_label.format(unit=suffix) if "{unit}" in y_label else y_label, fontsize=9)
    if fig_cfg.get("title"):  # optional: several slides already carry their own heading
        ax.set_title(fig_cfg["title"], fontsize=12)
    ax.tick_params(labelsize=9)
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)
    ax.grid(axis="y", alpha=0.25)
    ax.set_axisbelow(True)
    fig.tight_layout()
    # 125 dpi keeps these ~875 px wide, the size the slides were laid out for
    fig.savefig(output, dpi=125)
    plt.close(fig)


DRAW = {"bars": draw_bars, "curves": draw_curves, "vbars": draw_vbars}


# ------------------------------------------------------------ inspection view (no configuration)

def discover(cfg):
    """One chart per criterion case: vertical bars + standard deviation, to review raw results"""
    by_case = collections.defaultdict(dict)
    for path in glob.glob(f"{cfg['criterion_dir']}/**/estimates.json", recursive=True):
        name = os.path.basename(os.path.dirname(os.path.dirname(path)))
        case, _, variant = name.rpartition("-")
        if not case:
            case, variant = "default", name
        with open(path) as f:
            data = json.load(f)
        by_case[case][variant] = (
            data["mean"]["point_estimate"],
            data["std_dev"]["point_estimate"],
        )

    output_dir = cfg["discover_dir"]
    os.makedirs(output_dir, exist_ok=True)
    cmap = plt.get_cmap("tab10")
    all_variants = sorted({v for d in by_case.values() for v in d})
    color_of = {v: cmap(i % 10) for i, v in enumerate(all_variants)}

    for case, measures in by_case.items():
        variants = [v for v in all_variants if v in measures]
        plt.figure(figsize=(8, 5))
        plt.bar(
            range(len(variants)),
            [measures[v][0] for v in variants],
            yerr=[measures[v][1] for v in variants],
            capsize=5,
            color=[color_of[v] for v in variants],
        )
        plt.xticks(range(len(variants)), variants, rotation=45, ha="right")
        if case != "default":
            plt.title(f"Benchmarks for case '{case}'")
        plt.ylabel("Mean time (ns)")
        plt.tight_layout()
        path = os.path.join(output_dir, f"{case}.png")
        plt.savefig(path)
        plt.close()
        print(f"Saved chart to: {path}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default=CONFIG)
    parser.add_argument("--only", help="only plot figures whose path contains this")
    parser.add_argument("--discover", action="store_true", help="inspection view, no config needed")
    parser.add_argument("--data", help="callgrind report to use (default: the one from the config)")
    parser.add_argument(
        "--skip-missing",
        action="store_true",
        help="skip figures whose measurements are absent instead of failing "
             "(a full run needs every bench to have been run)",
    )
    args = parser.parse_args()

    with open(args.config) as f:
        cfg = json.load(f)
    if args.data:
        cfg["data"] = args.data

    if args.discover:
        discover(cfg)
        return

    skipped = []
    for fig_cfg in cfg["figures"]:
        if args.only and args.only not in fig_cfg["file"]:
            continue
        output = os.path.join(cfg["output_root"], fig_cfg["file"])
        os.makedirs(os.path.dirname(output), exist_ok=True)
        try:
            DRAW[fig_cfg["type"]](fig_cfg, cfg, output)
        except SystemExit as missing:
            if not args.skip_missing:
                raise
            skipped.append((fig_cfg["file"], str(missing).splitlines()[0]))
            continue
        print(f"Saved chart to: {output}")

    for file, reason in skipped:
        print(f"Skipped {file}: {reason}")


if __name__ == "__main__":
    main()
