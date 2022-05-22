# Running Rust in Jupyter notebooks

* Prepare a virtual environment (optional)
  ```shell
  python3 -m venv venv
  source ./venv/bin/activate
  pip3 install -r requirements.txt
  ```
* Install [Rust kernel for Jupyter](https://github.com/google/evcxr/blob/main/evcxr_jupyter/README.md)

  This should be something like:
  ```shell
  cargo install evcxr_jupyter
  evcxr_jupyter --install
  ```

* [Demo time](https://github.com/google/evcxr/blob/main/evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb)

  More about [evcxr tools](https://github.com/google/evcxr)

* Add a crate dependency
  In Jupyter notebook
   ```jupyter
  :dep rand="*"
  ```
  (it could be slow since it downloads/compiles/installs the crate)

  Then, use it as usual
  ```rust
  use rand::prelude::*;
  rand::random::<u32>()
  ```
  (when using REPL, it could require explicit typing since
  rustc cannot deduce type from affectation or return types. You should use the [*turbo-fish
  operator*](https://doc.rust-lang.org/1.60.0/reference/glossary.html#turbofish))

* Plots in *Jupyter Lab* with Rust using Plotly.rs

  [Alternatives from official package](https://github.com/google/evcxr/blob/main/evcxr_jupyter/README.md#3rd-party-integrations) 

  or [plotters](https://crates.io/crates/plotters) (plotters does not require jupyter lab extention)

  Plotly.rs seems to be the only maintained package

  ```shell
  jupyter labextension install jupyterlab-plotly@4.9.
  ```

  ```jupyter
  :dep plotly = { version = ">=0.7.0" }
  :dep itertools-num = "0.1.3"
  ```
 
  ```rust
  extern crate plotly;
  extern crate rand_distr;
  extern crate itertools_num;
  extern crate itertools;
  ```
  
  ```rust
  use itertools_num::linspace;
  use plotly::common::{
  ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
  };
  use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
  use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};
  use rand_distr::{Distribution, Normal, Uniform};
  ```
 
  ```rust
  let n: usize = 100;
  let t: Vec<f64> = linspace(0., 10., n).collect();
  let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
  
  let trace = Scatter::new(t, y).mode(Mode::Markers);
  let mut plot = Plot::new();
  plot.add_trace(trace);
  let layout = Layout::new().height(800);
  plot.set_layout(layout);
  plot.lab_display();
  ```
