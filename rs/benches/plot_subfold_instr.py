#!/usr/bin/env python3
"""Trace les instructions par élément des 3 variantes de `subfold`.

Entrée : la sortie du balayage callgrind (cf benches/README.md), une ligne par mesure :

    <source> <n> <variante> <instructions>

Usage :
    python3 benches/plot_subfold_instr.py instr.txt [-o criterion_plots_by_case]
"""
import argparse
import collections
import os

import matplotlib.pyplot as plt

SIZE = 1 << 14  # nombre d'éléments par run (const SIZE de benches/subfold.rs)
REP = 20  # nombre de runs dans examples/subfold_instr.rs
# (style, marqueur) : pull_fold et push_fold se superposent souvent, d'où le trait pointillé
VARIANTS = {"pull_next": ("-", "o"), "pull_fold": ("-", "s"), "push_fold": ("--", "x")}
LABELS = {
    "pull_next": "pull — next() élément par élément",
    "pull_fold": "pull — fold() dans le bloc",
    "push_fold": "push — fold() dans le bloc + surcharge de fold()",
}


def read(path):
    """{(source, n): {variante: instructions}}"""
    data = collections.defaultdict(dict)
    with open(path) as f:
        for line in f:
            parts = line.split()
            if len(parts) != 4:
                continue
            source, n, variant, instructions = parts
            data[(source, int(n))][variant] = int(instructions.replace(",", ""))
    return data


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", help="sortie du balayage callgrind")
    parser.add_argument("-o", "--output-dir", default="criterion_plots_by_case")
    args = parser.parse_args()

    data = read(args.input)
    os.makedirs(args.output_dir, exist_ok=True)

    sources = sorted({source for source, _ in data})
    for source in sources:
        sizes = sorted(n for s, n in data if s == source)
        plt.figure(figsize=(7, 4.5))
        for variant, (style, marker) in VARIANTS.items():
            ys = []
            for n in sizes:
                row = data[(source, n)]
                # la baseline mesure la seule génération des données
                ys.append((row[variant] - row["baseline"]) / REP / SIZE)
            plt.plot(sizes, ys, style, marker=marker, markersize=8, label=LABELS[variant])

        plt.xscale("log", base=2)
        plt.xticks(sizes, [str(n) for n in sizes])
        plt.ylim(bottom=0)
        plt.xlabel("taille du sous-bloc n")
        plt.ylabel("instructions par élément")
        plt.title(f"subfold — source « {source} »")
        plt.grid(alpha=0.3)
        plt.legend(fontsize=8)
        plt.tight_layout()

        output = os.path.join(args.output_dir, f"subfold_{source}.png")
        plt.savefig(output, dpi=150)
        plt.close()
        print(f"Saved chart to: {output}")


if __name__ == "__main__":
    main()
