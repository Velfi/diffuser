# Diffuser

A common theme in visualizations I create is their dependence on performing operations in parallel on a grid of values.
I write my visualizations in Rust. Because of Rust's ownership rules, you can't iterate over some in parallel while reading
and writing to it. This can generally be solved by copying the data grid and then reading from the copy in order to write to
the original. Diffuser was a program that I wrote to explore how I wanted to do that.

It works like this:

- The user can paint on the screen by clicking and dragging. Because mouse position updates aren't continuous, I use a line drawing algorithm to smooth the drawing out. When you draw on a cell, it can deposit more "ink" than the cell can "hold".
- Once per tick, I check each cell to see if it has too much ink. If it does, then I divide the ink between the original cell and its 8 immediate neighboring cells.
- Once per tick, I subtract a small amount of ink from any cell that contains ink.
- Lastly, I iterate over the grid and a pixel framebuffer in parallel, translating grid values into RGB colors

These rules together give the appearance of drawing with a liquid. They can look similar to Rorschach ink blots.

![diffuser example](./diffuser-example-1.png)
