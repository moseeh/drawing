# Geometrical Shapes in Rust

## Overview

This project's goal is to create and manipulate various geometrical shapes in Rust using traits and modules. It simulate drawing, coloring, and displaying shapes with proper struct and trait definitions.

## Project Structure

The project ids structured as follows:

- drawing/src/
    - main.rs 
    - geometrical_shapes.rs


## Features

- `main.rs` contains the usage logic.
- `geometrical_shapes.rs` defines all traits, structs, and their implementations.


## 🛠 Implementation Details

### ✅ Traits

Two traits are defined in `geometrical_shapes.rs`:

#### `Drawable`

This trait provides methods for rendering and coloring shapes:

```rust
pub trait Drawable {
    fn draw(&self);
    fn color(&self);
}
```

#### `Displayable`

This trait is responsible for displaying shape-specific information:

```rust
pub trait Displayable {
    fn display(&self);
}
```

###  **Shapes** 

| Shape      | `new` Function Parameters                                     |
|------------|---------------------------------------------------------------|
| `Point`    | Two `i32` values (x, y)                                       |
| `Line`     | References to two different `Point`s                          |
| `Triangle` | References to three different `Point`s                        |
| `Rectangle`| References to two different `Point`s                          |
| `Circle`   | Reference to a `Point` (center) and an `i32` for the radius   |





## 🚀 How to Run

1. Fork the project from: `https://learn.zone01kisumu.ke/git/moonyango/drawing`

2. Make sure both `main.rs` and `geometrical_shapes.rs` are in the same project folder. In your `main.rs`, include the module like this:

```rust
mod geometrical_shapes;
```
3. In the terminal navigate to the folder containing the `main.rs` and `geometrical_shapes.rs`, the type `cargo run`.

4. Hit Enter. An image containing the geometrical shapes is generated.


### Authors

- Moses Onyango
- Shayo Victor
- Barrack Kope


## How to contribute 👷 

**1.** Fork [this](https://learn.zone01kisumu.ke/git//moonyango/drawing.git) repository.

**2.** Clone the forked repository.

```terminal
git clone https://learn.zone01kisumu.ke/git/moonyango/drawing.git
```

**3.** Navigate to the project directory.

```terminal
cd drawing
```

**4.**  Make a new folder with your project name inside.
<br>

**5.**  Also Add a README file in your project folder which consists of Description/screenshots about your project !          
 
<br>

**6.** Create a new branch.

```terminal
git checkout -b <your_branch_name>
```

**7.** Add & Commit your changes.

```terminal
  git add .
  git commit -m "<your_commit_message>"
```

  Push your local branch to the remote repository.

```terminal
git push -u origin <your_branch_name>
```

**8.** Create a Pull Request!
