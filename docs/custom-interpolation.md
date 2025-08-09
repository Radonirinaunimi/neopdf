# Custom Interpolation Strategies

`neopdf` is built on a modular architecture that allows users to define and use their own custom interpolation strategies. This is an advanced feature for users who need to implement specialized interpolation algorithms beyond the ones provided by default.

This guide will walk you through the process of creating and using a custom 1D interpolation strategy. The same principles apply to 2D and 3D strategies.

## Core Concept: The `Strategy` Traits

The interpolation logic in `neopdf` is powered by the [`ninterp`](https://github.com/NREL/ninterp) crate. To create a custom interpolator, you must define a struct that implements one of the core `Strategy` traits from `ninterp`:

- `Strategy1D`: For 1-dimensional interpolation.
- `Strategy2D`: For 2-dimensional interpolation.
- `Strategy3D`: For 3-dimensional interpolation.

These traits require you to implement a few methods, the most important of which is `interpolate`.

### Required Methods

For any `Strategy` trait, you must implement:

- **`interpolate(&self, data, point)`**: This is where your core interpolation logic goes. It takes the grid data and a point to evaluate and should return the interpolated value.
- **`allow_extrapolate(&self) -> bool`**: This method should return `true` if your strategy supports extrapolation outside the grid boundaries, and `false` otherwise.

You can also optionally implement:

- **`init(&mut self, data)`**: This method is called once when the interpolator is created. It's useful for performing validation on the grid data or pre-computing values (like coefficients) to speed up the `interpolate` calls.

## Step-by-Step Guide to a Custom 1D Strategy

Let's create a simple `NearestNeighbor` interpolation strategy as an example. This strategy will find the grid point closest to the requested point and return its value.

### Step 1: Define the Strategy Struct

First, define an empty struct for your strategy. It can contain fields for configuration if needed.

```rust
use ninterp::data::InterpData1D;
use ninterp::error::{InterpolateError, ValidateError};
use ninterp::strategy::traits::Strategy1D;
use ndarray::Data;

/// A custom strategy that returns the value of the nearest grid point.
#[derive(Debug, Clone)]
pub struct NearestNeighbor;
```

### Step 2: Implement the `Strategy1D` Trait

Now, implement the `Strategy1D` trait for the `NearestNeighbor` struct.

```rust
impl<D> Strategy1D<D> for NearestNeighbor
where
    D: Data<Elem = f64>,
{
    /// Finds the closest grid point and returns its value.
    fn interpolate(
        &self,
        data: &InterpData1D<D>,
        point: &[f64; 1],
    ) -> Result<f64, InterpolateError> {
        let x = point[0];
        let x_coords = data.grid[0].as_slice().unwrap();
        let values = data.values.as_slice().unwrap();

        // Find the index of the grid point closest to `x`.
        let mut closest_idx = 0;
        let mut min_dist = f64::MAX;

        for (i, &grid_x) in x_coords.iter().enumerate() {
            let dist = (x - grid_x).abs();
            if dist < min_dist {
                min_dist = dist;
                closest_idx = i;
            }
        }

        Ok(values[closest_idx])
    }

    /// This simple strategy does not support extrapolation.
    fn allow_extrapolate(&self) -> bool {
        false
    }
}
```

### Step 3: Use the Custom Strategy

Once your strategy is defined, you can use it with the `ninterp` `Interpolator` to create a functioning interpolator object. `neopdf` is built on top of this, so the integration is seamless.

```rust
use ninterp::interpolator::Interpolator;
use ninterp::data::InterpData1D;
use ndarray::Array1;

fn main() {
    // 1. Create your custom strategy instance.
    let strategy = NearestNeighbor;

    // 2. Create your grid data.
    let x_coords = Array1::from(vec![1.0, 2.0, 3.0, 4.0]);
    let y_values = Array1::from(vec![10.0, 20.0, 30.0, 40.0]);
    let data = InterpData1D::new(x_coords, y_values).unwrap();

    // 3. Build the interpolator with your custom strategy.
    let interpolator = Interpolator::new(data, strategy);

    // 4. Interpolate a point.
    let point = [2.6]; // This is closer to 3.0 than 2.0
    let result = interpolator.interpolate(&point).unwrap();

    println!("Interpolated value at {} is: {}", point[0], result);
    // Expected output: Interpolated value at 2.6 is: 30
    assert_eq!(result, 30.0);

    let point = [2.4]; // This is closer to 2.0 than 3.0
    let result = interpolator.interpolate(&point).unwrap();
    println!("Interpolated value at {} is: {}", point[0], result);
    // Expected output: Interpolated value at 2.4 is: 20
    assert_eq!(result, 20.0);
}
```

This example demonstrates how the modular design allows you to inject any compatible strategy into the interpolation framework. While this example uses `ninterp` directly, the same `Strategy` objects can be integrated into the higher-level `neopdf` structures.
