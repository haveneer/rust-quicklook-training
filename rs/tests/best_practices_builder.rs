/// Grid to build: the size is required, the rest is optional.
#[derive(Debug, Clone)]
pub struct Grid {
    nx: usize,
    ny: usize,
    spacing: f64,
    label: String,
}

impl Grid {
    fn new(nx: usize, ny: usize) -> Self {
        let label = String::new();
        Grid {
            nx,
            ny,
            spacing: 1.0,
            label,
        }
    }
    fn check(&self) -> Result<(), &'static str> {
        if self.nx < 2 || self.ny < 2 {
            return Err("at least 2 nodes per direction");
        }
        Ok(())
    }
}

/// Variant 1: consuming builder, `self -> Self`.
mod consuming {
    use super::Grid;

    #[must_use]
    pub struct GridBuilder(Grid);

    impl Grid {
        /// Entry point; the required parameters are given here.
        pub fn builder(nx: usize, ny: usize) -> GridBuilder {
            GridBuilder(Grid::new(nx, ny))
        }
    }

    impl GridBuilder {
        pub fn spacing(mut self, h: f64) -> Self {
            self.0.spacing = h;
            self
        }
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.0.label = label.into();
            self
        }
        /// Consumes the builder: the grid is moved, not cloned.
        pub fn build(self) -> Result<Grid, &'static str> {
            self.0.check()?;
            Ok(self.0)
        }
    }
}

/// Variant 2: non-consuming builder, `&mut self -> &mut Self`.
mod borrowing {
    use super::Grid;

    #[must_use]
    pub struct GridBuilder(Grid);

    impl GridBuilder {
        pub fn new(nx: usize, ny: usize) -> Self {
            GridBuilder(Grid::new(nx, ny))
        }
        pub fn spacing(&mut self, h: f64) -> &mut Self {
            self.0.spacing = h;
            self
        }
        pub fn label(&mut self, label: impl Into<String>) -> &mut Self {
            self.0.label = label.into();
            self
        }
        /// Keeps the builder, which can build again: the grid is cloned.
        pub fn build(&self) -> Result<Grid, &'static str> {
            self.0.check()?;
            Ok(self.0.clone())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Variant 1: a single chained expression
    let wing = Grid::builder(64, 32).spacing(0.01).label("wing").build()?;
    println!("{wing:?}");

    // Variant 2: configured in several steps, then built several times
    let refine = true;
    let mut builder = borrowing::GridBuilder::new(64, 32);
    builder.label("flap");
    if refine {
        builder.spacing(0.005);
    }
    let coarse = builder.build()?;
    let fine = builder.spacing(0.001).build()?;
    println!("{coarse:?}\n{fine:?}");
    Ok(())
}

#[test]
fn test() {
    main().unwrap()
}
