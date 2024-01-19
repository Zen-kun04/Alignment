# Alignment (Rust)
Tired of wasting your time trying to align your text to the center or left? I introduce to you: Alignment.

Alignment is a Rust library that lets you align your text to the center or left and optionally add padding on the "X" axis, always respecting the alignment (center or left).

## Usage

### _Simple print ?_
```rust
use align::{ print_center, print_right };

fn main() {
    print_center(&["This", "text", "will be centered", "in the CLI"], None); // Center the text with no X padding.
    print_center(&["This", "text", "will be centered", "in the CLI"], Some(20)); // Center the text with an extra 20 padding in the X axis.

    print_right(&["This", "text", "will be to", "the right", "in the CLI"], None); // Right align the text with no X padding.
    print_right(&["This", "text", "will be to", "the right", "in the CLI"], Some(20)); // Right align the text with an extra 20 padding in the X axis.
}

```

### _Stored in variable ?_
```rust
use align::{ center, right };

fn main() {
    let center_no_padding = center(&["This", "text", "will be centered", "in the CLI"], None); // Center the text with no X padding.
    let center_with_padding = center(&["This", "text", "will be centered", "in the CLI"], Some(20)); // Center the text with an extra 20 padding in the X axis.

    let right_no_padding = right(&["This", "text", "will be to", "the right", "in the CLI"], None); // Right align the text with no X padding.
    let right_with_padding = right(&["This", "text", "will be to", "the right", "in the CLI"], Some(20)); // Right align the text with an extra 20 padding in the X axis.
}
```

## Must know
I'm Zen-kun04, also known as Baguette. I started with Rust not long ago, so this is my first library (and Rust project).

I will, of course, maintain the library if I have some useful ideas and/or get suggestions from users.