#!/usr/bin/env python3
"""Barres lisibles *en projection* pour un cas criterion donné.

`plot_benches.py` produit une vue d'inspection (barres verticales, libellés pivotés, titres
techniques, nanosecondes) : parfait pour lire ses résultats, illisible sur un vidéoprojecteur.
Ce script lit les *mêmes* données (`target/criterion/<cas>-<variante>/new/estimates.json`) et en
tire une barre horizontale par variante, annotée de sa valeur, avec un titre lisible.

    python3 benches/plot_slide_bars.py adapter_filter_sum \\
        --title "Source Filter, consommée par sum()" \\
        --output-dir ../../images/benchmarks/adapter --name filter_sum

Sans `--title`/`--name`, le nom du cas sert de titre et de nom de fichier.
"""
import argparse
import glob
import json
import os

import matplotlib.pyplot as plt

# Libellés de projection pour les variantes rencontrées dans ces bancs
LABELS = {
    "next_only": "next() seul",
    "with_fold": "+ fold",
    "with_fold_find": "+ fold + find",
    "std_map": ".map() de la std",
    "pull_next": "pull — next() par élément",
    "pull_fold": "pull — fold() dans le bloc",
    "push_fold": "push — + surcharge de fold()",
    "chunks_ref": "chunks() (référence)",
}
# Une variante « lente » en rouge, la référence en gris, le reste en vert
SLOW = {"next_only", "pull_next"}
REFERENCE = {"std_map", "chunks_ref"}
# Ordre de lecture (du naïf vers la référence), et non l'ordre alphabétique des dossiers
ORDER = [
    "next_only",
    "pull_next",
    "with_fold",
    "pull_fold",
    "with_fold_find",
    "push_fold",
    "std_map",
    "chunks_ref",
]


def read_case(case):
    """[(variante, moyenne_ns)] dans l'ordre de lecture d'ORDER"""
    out = []
    for path in glob.glob(f"target/criterion/{case}-*/new/estimates.json"):
        variant = os.path.basename(os.path.dirname(os.path.dirname(path)))[len(case) + 1 :]
        with open(path) as f:
            out.append((variant, json.load(f)["mean"]["point_estimate"]))
    rank = {v: i for i, v in enumerate(ORDER)}
    out.sort(key=lambda kv: (rank.get(kv[0], len(ORDER)), kv[0]))
    return out


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("case", help="nom du cas criterion, sans la variante")
    parser.add_argument("--title", help="titre du graphe (défaut: le nom du cas)")
    parser.add_argument("--name", help="nom du fichier PNG (défaut: le nom du cas)")
    parser.add_argument("--output-dir", default="criterion_plots_by_case")
    parser.add_argument("--unit", choices=["us", "ns"], default="us")
    parser.add_argument("--elements", type=int, default=16384, help="éléments par itération")
    args = parser.parse_args()

    measures = read_case(args.case)
    if not measures:
        raise SystemExit(f"aucune mesure pour « {args.case} » : lancer `cargo bench` d'abord")

    divisor = 1000 if args.unit == "us" else 1
    suffix = "µs" if args.unit == "us" else "ns"
    names = [LABELS.get(v, v) for v, _ in measures]
    values = [m / divisor for _, m in measures]
    colors = [
        "#c0392b" if v in SLOW else "#7f8c8d" if v in REFERENCE else "#27ae60"
        for v, _ in measures
    ]

    fig, ax = plt.subplots(figsize=(6.4, 0.46 * len(measures) + 0.95))
    bars = ax.barh(names[::-1], values[::-1], color=colors[::-1], height=0.68)
    ax.bar_label(bars, fmt=f"%.1f {suffix}", padding=4, fontsize=10)
    ax.set_xlim(0, max(values) * 1.28)
    ax.set_xlabel(
        f"temps pour {args.elements} éléments ({suffix}, plus bas = mieux)", fontsize=9
    )
    ax.set_title(args.title or args.case, fontsize=12)
    ax.tick_params(labelsize=10)
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)
    fig.tight_layout()

    os.makedirs(args.output_dir, exist_ok=True)
    path = os.path.join(args.output_dir, f"{args.name or args.case}.png")
    fig.savefig(path, dpi=170)
    plt.close(fig)
    print(f"Saved chart to: {path}")


if __name__ == "__main__":
    main()
