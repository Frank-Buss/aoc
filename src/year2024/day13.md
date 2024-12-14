---
urlcolor: blue
author: 'Frank Buss'
...
# Advent of Code 2024, day 13
Part 1 can be solved trivially with brute-force. This is not possible for part 2 due to the much larger prize targets. The problem can be described as a system of linear equations:

\begin{equation}
\begin{aligned}
x_1b_1 + x_2b_2 &= p_x\\
y_1b_1 + y_2b_2 &= p_y
\end{aligned}
\end{equation}

This system has two unknowns $b_1$ and $b_2$, which must be integers. It can have exactly one solution, no solution, or infinite solutions. For a unique solution, the determinant must be non-zero:

\begin{equation}
\det\begin{pmatrix}
x_1 & x_2 \\
y_1 & y_2
\end{pmatrix} = x_1y_2 - x_2y_1 \neq 0
\end{equation}

This would be easy to solve, if the unknowns were rationals. But since they are integers, it is a system of linear [Diophantine equations](https://en.wikipedia.org/wiki/Diophantine_equation#Diophantine_geometry). In general it gets complicated, but analyzing the solutions from part 1 shows that there are either none or one solution, so first let's try to solve as usual, with Cramer's rule, which looks like this for our equation system, for calculating $b_1$:

\begin{equation}
b_1 = \frac{\det\begin{pmatrix}
p_x & x_2 \\
p_y & y_2
\end{pmatrix}}{d} = \frac{p_xy_2 - x_2p_y}{x_1y_2 - x_2y_1}
\end{equation}

With $b_1$, we can calculate $b_2$ a bit faster, by solving our first linear equation for $b_2$:

\begin{equation}
b_2 = \frac{p_x - x_1b_1}{x_2}
\end{equation}

Now to check if it is an integer solution, we just have to test if the numerators are divisible by the denominators, taking care to handle negative determinants properly. In Rust we can use the modulo operator, looks like this, including the cost function:

```rust
let det: i128 = (x1 * y2) as i128 - (x2 * y1) as i128;
if det != 0 {
    let num: i128 = (px * y2) as i128 - (x2 * py) as i128;
    if num.abs() % det.abs() == 0 {
        let b1 = num / det;
        let num: i128 = px as i128 - x1 as i128 * b1;
        if num % x2 as i128 == 0 {
            let b2 = num / x2 as i128;
            let cost = 3 * b1 + b2;
            solution2 += cost as u128;
        }
    }
}
```
