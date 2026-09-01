# rust-dither

Simple, terminal-based rust program to perform dithering on images.

Supports:
- Threshold
- Random
- Bayer (2x, 4x)
- Floyd Steinberg
- HalfTone (in-progress)

Algorithms to be added:
- Bayer (8x) (along with an arg for the matrix size ?)
- Void and Cluster
- Atkinson

### Examples (bayer_4 & random)
<img src="examples/bayer_4_example.jpg" alt="bayer_4_screenshot" width="350"/> <img src="examples/fs_example.jpg" alt="floyd_steinberg_screenshot" width="350"/>

### Installation

Clone the repo:
```
git clone https://github.com/luca-thompson/dither.git
```

Install Dependancies and Compile:
```
cargo build
```

### Usage

After having run cargo build,

```
cargo run --f-in <F_IN> --f-out <F_OUT> --algorithm <ALGORITHM>
```

| Arg         | Purpose                                 |
| ----------- | --------------------------------------- |
| --f-in      | file in (image to dither)               |
| --f-out     | file out (dithered image save location) |
| --algorithm | algorithm to use                        |
