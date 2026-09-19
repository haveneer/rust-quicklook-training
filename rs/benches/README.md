# Benchmarks

* `cargo bench` runs the criterion benches (wall-clock time)
* `cargo bench -- --test` (ou `cargo test --benches`) exécute chaque bench une fois: c'est ainsi
  que `unix_pipeline()` (la démonstration « commande Unix » de `subfold.rs`) est vérifiée (un bench `harness = false`
  n'exécute aucun `#[test]`)
* `cargo bench --features iai` runs the gungraun (ex iai-callgrind) benches (instruction counts via Valgrind);
  benches with a gungraun variant: `iai_demo` (always gungraun), `dispatch`, `block_parser`'s `parser`

## Figures

Toutes les figures passent par un seul script, `benches/plot_figures.py`, décrit par
`benches/figures.json` (type de graphe, cas criterion ou relevé callgrind, ordre des variantes,
libellés, titres, couleurs, fichier de sortie) :

```shell
venv/bin/python benches/plot_figures.py                  # toutes les figures des slides
venv/bin/python benches/plot_figures.py --only adapter   # un sous-ensemble (filtre sur le chemin)
venv/bin/python benches/plot_figures.py --skip-missing   # ignore les figures sans mesures
venv/bin/python benches/plot_figures.py --discover       # vue d'inspection, sans configuration
```

Trois types de figures :

| type | source | utilisé par |
|---|---|---|
| `bars` | criterion, cas `<case>-<variante>` | `adapter/` (barres horizontales, lisibles en projection) |
| `vbars` | criterion (`benches`: chemins sous `target/criterion`) ou relevé `<nom> <valeur>` (`data`) | `allocations/`, `pointers/`, `refcell/`, `dispatch/` |
| `curves` | relevé callgrind `<source> <n> <variante> <instructions>` | `subfold/` |

Les couleurs suivent `series_colors` : **une teinte par structure mesurée, réutilisée d'une figure
à l'autre** (`box`, `rc`, `arc`… gardent la leur dans `create`, `clone` et `access`). La clé est le
nom du bench, ou la partie après le premier `-`. Toutes les figures puisent dans la même gamme
(tab10), y compris `adapter/`. À défaut de correspondance, la figure retombe sur `palette` +
`roles` (rouge « lent », gris « référence »).

Un `vbars` adresse chaque bench par **le répertoire que criterion a créé** : `create-box` pour un
`bench_function("create-box")`, `allocations/Heap allocation (Vec)` dans un
`benchmark_group("allocations")`. Criterion assainit les identifiants (`/` → `_`, espaces de fin
supprimés) : `push/pop-on Vec ` devient `push_pop-on Vec`. En cas d'erreur, le script liste les
benchs réellement disponibles.

Une exécution complète suppose que **tous** les benchs ont tourné :

```shell
cargo bench --bench adapter_internal_iteration   # adapter/
cargo bench --bench allocations_inner_loop       # allocations/*_stack_vs_heap
cargo bench --bench allocations_many_small       # allocations/*_many_small_objects
cargo bench --bench smart_pointers               # pointers/
cargo bench --bench refcell                      # refcell/
# subfold/ et dispatch/ : relevés callgrind, voir plus bas
```

`dispatch/` porte deux figures : `static_vs_dynamic_time.png` (criterion) et
`static_vs_dynamic_instructions.png` (gungraun, compte d'instructions). **Ce sont les
instructions que les slides montrent** : les variantes les plus rapides tournent à ~0,5 ns par
appel, régime où criterion rapporte ~20 % d'écart-type — allonger la boucle (`ROUNDS`) n'y change
rien, la dispersion suit le temps par appel, pas la longueur de la boucle.
Le relevé d'instructions, lui, rejoue à l'instruction près.

Produire `dispatch_instr.txt` (deux colonnes `<variante> <instructions>`) depuis la sortie du
bench, dans le conteneur décrit ci-dessous :

```shell
cargo bench --bench dispatch --features iai |
  awk '/iai_/ { name = $0; sub(/.*iai_/, "", name); sub(/[^a-z_].*/, "", name) }
       /Instructions/ { v = $2; sub(/\|.*/, "", v); gsub(/[^0-9]/, "", v);
                        if (name != "" && v != "") { print name, v; name = "" } }' \
  > dispatch_instr.txt
venv/bin/python benches/plot_figures.py --only dispatch
```

`--discover` remplace l'ancien `plot_benches.py` : il balaie `target/criterion` et sort un graphe
par cas dans `criterion_plots_by_case/` (barres verticales, écart-type), pour lire ses résultats.
Les figures versionnées dans `images/benchmarks/`, elles, sont décrites dans `figures.json` — voir
aussi `images/benchmarks/subfold/README.md` et `images/benchmarks/adapter/README.md`.

`matplotlib` est requis. Un virtualenv est déjà en place dans `code/rs/venv` (non versionné,
ignoré par `.gitignore`) — d'où le `venv/bin/python` ci-dessus. Pour le recréer :

```shell
python3 -m venv venv && venv/bin/pip install -r requirements.txt
```

## Running gungraun benches with Docker

gungraun needs Valgrind and `gungraun-runner` (same version as the `gungraun` crate), provided by
[environment/Dockerfile](environment/Dockerfile). From `code/rs`:

```shell
docker build -t rust-quicklook-bench benches/environment
docker run --rm --security-opt seccomp=unconfined \
  -v "$PWD":/work -v rq-target:/target -v rq-cargo:/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/target -w /work rust-quicklook-bench \
  bash -c 'cargo bench --bench iai_demo &&
           cargo bench --bench dispatch --features iai &&
           cargo bench -p block_parser --bench parser --features iai'
```

* `--security-opt seccomp=unconfined` is required: gungraun disables ASLR (`setarch`), which Docker's default
  seccomp profile forbids (`setarch: failed to set personality ...: Operation not permitted`)
* `rq-target` and `rq-cargo` volumes keep build artifacts and crates between runs; a new run is compared to the
  previous one (first run shows `N/A`)
* instruction counts depend on the architecture (aarch64 on Apple Silicon vs x86_64)

## `subfold` : pull vs push (benches/subfold.rs)

Trois écritures du même adaptateur `subfold` (toutes dans `benches/subfold.rs`, qui porte aussi
le `RandomGenerator`, le trait d'extension `SubFoldable` et la démonstration « commande Unix »
montrés en cours), sur 4 sources (`slice`, `filter`, `chain4`, `flatmap`) et
3 tailles de bloc (n = 2, 8, 64) :

* `pull_next` : `next()` tire la source élément par élément (itération externe)
* `pull_fold` : `next()` replie le bloc avec `fold()` (itération interne dans le bloc)
* `push_fold` : idem + surcharge de `Iterator::fold` (itération interne de bout en bout)

`cargo bench --bench subfold` donne les temps, **mais les écarts sont ici du même ordre que le
bruit et changent de 2x selon le codegen** (ajouter un bench voisin suffit à inverser un
classement). Pour des chiffres reproductibles, compter les instructions avec callgrind :

```shell
docker build -t rust-quicklook-bench benches/environment   # si l'image n'existe pas
docker run --rm --security-opt seccomp=unconfined \
  -v "$PWD":/work -v rq-target:/target -v rq-cargo:/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/target -w /work rust-quicklook-bench bash -c '
    cargo build --release --example subfold_instr
    for s in slice filter chain4 flatmap; do
      for n in 2 4 8 16 64; do
        for var in baseline pull_next pull_fold push_fold; do
          ir=$(valgrind --tool=callgrind --callgrind-out-file=/dev/null \
                 /target/release/examples/subfold_instr $s $var $n 2>&1 |
               grep "refs:" | tr -d " ," | cut -d: -f2)
          echo "$s $n $var $ir"
        done
      done
    done' > instr.txt
python3 benches/plot_figures.py --only subfold   # -> images/benchmarks/subfold/*.png
```

`baseline` mesure la seule génération des données et se déduit des autres mesures.

Résultats (instructions par élément, rustc 1.90, aarch64) :

| source    | n  | pull_next | pull_fold | push_fold |
|-----------|----|-----------|-----------|-----------|
| `slice`   | 8  | 8.38      | 8.25      | 9.00      |
| `slice`   | 64 | 8.05      | 8.03      | 9.00      |
| `filter`  | 8  | 7.01      | 6.45      | 6.39      |
| `chain4`  | 2  | 16.88     | 25.00     | 25.01     |
| `chain4`  | 8  | 17.35     | 14.50     | 14.51     |
| `chain4`  | 64 | 17.48     | 11.44     | 11.44     |
| `flatmap` | 8  | 9.56      | 14.02     | 13.89     |

Temps criterion correspondants (µs pour 16384 éléments, Apple M-series, machine peu chargée) :

| source    | n  | pull_next | pull_fold | push_fold | chunks (réf.) |
|-----------|----|-----------|-----------|-----------|---------------|
| `slice`   | 8  | 7.06      | 7.02      | 6.97      | 7.37          |
| `chain4`  | 8  | 17.06     | **7.67**  | 15.26     | 8.78          |
| `flatmap` | 8  | 10.99     | **7.24**  | 7.18      | 7.26          |
| `filter`  | 8  | 36.12     | 29.71     | 27.15     | 7.41          |

À retenir :

* l'écriture élément par élément (`pull_next`) coûte x1.5 à x2.1, et seulement face à une source
  portant un état (`chain4`) ; l'écart croît avec n (1.53x à n = 8, 2.08x à n = 1024) puisque la
  ré-entrée dans le `fold` de la source s'amortit du côté `pull_fold`
* sur une source triviale (`slice`), les trois variantes produisent le même code : LLVM efface
  la différence ; `push_fold` est même légèrement moins bon (le 1er élément traité à part
  empêche une boucle unique)
* le gain vient de l'**itération interne dans le bloc** face à une source dont `next()` porte un
  état (`chain4` : -34 % d'instructions à n = 64) — et il faut n assez grand pour amortir la
  ré-entrée dans le `fold` de la source (croisement vers n = 5 sur `chain4`)
* si cette ré-entrée est chère (`flatmap` : `Flatten::fold` doit réarmer l'itérateur interne à
  chaque bloc), l'itération interne devient **perdante** (+47 % à n = 8)
* la surcharge de `fold()` (`pull_fold` -> `push_fold`) ne change **rien** en instructions :
  `next()` délègue déjà le gros du travail à `fold`, il ne reste que le coût par bloc, pas par
  élément. En temps elle peut même coûter cher (`chain4`, n = 8 : 15.3 µs contre 7.7 µs à nombre
  d'instructions identique) — la boucle écrite à la main dans `fold` n'est pas déroulée/vectorisée
  comme celle générée par la version par défaut
* corollaire méthodologique : **instructions et temps ne classent pas pareil**
  (`flatmap` n = 8 : la voie `fold` exécute +47 % d'instructions mais tourne 1.5x plus vite).
  Le compte d'instructions dit ce que le code *fait*, pas ce que le CPU *coûte* : montrer les deux

## Itération interne vs externe (benches/iteration_style.rs, benches/adapter_internal_iteration.rs)

Deux bancs qui répondent à la question posée par l'article de Veedrac, là où `benches/subfold.rs`
ne montrait qu'un effet marginal.

### 1. Pipelines `std` : `for` contre `fold` (`iteration_style.rs`)

Même pipeline, deux consommations : `for x in it { acc += x }` (externe, « pull ») contre
`it.fold(0, add)` (interne, « push »). Temps criterion, 16384 éléments :

| pipeline    | `for` (externe) | `fold` (interne) | gain      |
|-------------|-----------------|------------------|-----------|
| `map`       | 0.99 µs         | 0.98 µs          | x1.0      |
| `zip`       | 1.01 µs         | 1.01 µs          | x1.0      |
| `chain`     | 16.13 µs        | 1.39 µs          | **x11.6** |
| `filter`    | 51.64 µs        | 1.89 µs          | **x27.3** |
| `flat_map`  | 5.95 µs         | 7.56 µs          | x0.8      |
| `zip(chain)`| 27.52 µs        | 74.03 µs         | x0.4      |

Les adaptateurs porteurs d'état (`Chain`, `Filter`) ont un `fold`/`try_fold` spécialisé : la
boucle interne se vectorise, la boucle de `next()` non. Les deux derniers cas rappellent que ce
n'est pas une loi : `Flatten::fold` doit réarmer l'itérateur interne, et `Zip` ne peut pas
spécialiser quand sa source n'est pas à accès aléatoire.

### 2. Adapter écrit à la main (`adapter_internal_iteration.rs`)

La `std` a appliqué la leçon, **mais la chaîne casse dès qu'on insère son propre adapter** : son
`fold` par défaut est une boucle de `next()`, donc la source repasse en itération externe.
Le même adapter `Scale` (un `map` spécialisé) est écrit deux fois — `next()` seul, puis
`next()` + `fold` délégué à la source :

```rust
fn fold<B, F>(self, init: B, mut f: F) -> B where F: FnMut(B, u32) -> B {
    let k = self.k;
    self.iter.fold(init, |acc, x| f(acc, scale(x, k)))   // 3 lignes
}
```

Temps criterion (µs) et instructions par élément (callgrind) :

| source   | consommateur | `next()` seul | `+ fold`   | gain temps | instr/élém. `next()` seul → `+ fold` |
|----------|--------------|---------------|------------|------------|--------------------------------------|
| `slice`  | `sum()`      | 1.34          | 1.35       | x1.0       | 1.50 → 1.50                          |
| `filter` | `sum()`      | 40.28         | **2.32**   | **x17.4**  | 7.35 → **2.25**                      |
| `chain`  | `sum()`      | 21.31         | **2.72**   | **x7.9**   | 22.0 → **3.00**                      |
| `slice`  | `find()`     | 8.99          | 10.32      | x0.9       | 7.00 → 7.00                          |
| `filter` | `find()`     | 10.30         | 10.38      | x1.0       | 8.99 → 8.00                          |
| `chain`  | `find()`     | 22.83         | 26.82      | x0.9       | 23.0 → 21.0                          |

À retenir :

* sur une source triviale (`slice`), aucune différence — le gain n'existe que si la source a
  quelque chose à spécialiser
* dès que la source porte un état (`Filter`, `Chain`), **3 lignes de `fold` valent un facteur 8
  à 17** : sans elles, l'adapter annule le travail fait par la `std` en amont
* les consommateurs **court-circuitants** (`find`, `any`, `position`, `try_for_each`) ne
  profitent de rien : ils passent par `try_fold`, or `std::ops::Try` est encore instable
  (issue #84277) — un adapter écrit sur stable ne peut pas le surcharger

Comptage d'instructions (même procédure Docker que pour `subfold`) :

```shell
docker run --rm --security-opt seccomp=unconfined \
  -v "$PWD":/work -v rq-target:/target -v rq-cargo:/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/target -w /work rust-quicklook-bench bash -c '
    cargo build --release --example adapter_instr
    for s in slice filter chain; do for cons in sum find; do
      for var in baseline next_only with_fold; do
        ir=$(valgrind --tool=callgrind --callgrind-out-file=/dev/null \
               /target/release/examples/adapter_instr $s $var $cons 2>&1 |
             grep "refs:" | tr -d " ," | cut -d: -f2)
        echo "$s $cons $var $ir"
      done; done; done'
```
