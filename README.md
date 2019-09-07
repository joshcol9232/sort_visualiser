# Sorting Visualiser

### Visualisations:
#### Circle:

Displays array in a circle:

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/circle_vis.png" alt="Circular Visualisation" width="475" height="474">

#### Bars:

Displays array as a row of bars:

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/bar_vis.png" alt="Bar Visualisation" width="475" height="474">

Coloring:

**Element** | **Colour**
--- | ---
Active | Blue
Secondary Active (used when comparing elements) | Blue
Pivot (Quicksort) | Purple

Colours may change in the future.

With Lomuto partitioning quicksort (default quicksort implemented), the two active elements show the area where the elements are collecting that are bigger than the pivot. Once it reaches the end of the partition it moves the pivot to before that area.

#### Dots:

Displays array as dots (looks good with quick sort):

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/dot_vis.png" alt="Dot Visualisation" width="475" height="474">

Exact same colouring as the bar visualisation.

#### Pixels:

Displays multiple arrays, spanning from the left to the right of the window. Each row of pixels is a seperate array.

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/pixel_vis.png" alt="Pixel Visualisation" width="475" height="474">

<img src="https://github.com/eggmund/sort_visualiser/blob/master/readme_images/pixel_vis_shuffled.png" alt="Pixel Visualisation Shuffled" width="475" height="474">

Does not display active elements etc because it would be a bit too cluttered.

### Controls:
#### Sorts:
**Key** | **Sort**
--- | ---
**1** | Bubble Sort.
**2** | Selection Sort.
**3** | Quicksort.
**4** | Shell Sort.
**5** | Comb Sort (very similar to shell sort).
**6** | Radix LSD Sort (Base 10).

#### Array functions:
**Key** | **Sort**
--- | ---
**S** | Shuffle.
**R** | Reset array.
**I** | Invert/reverse array.
**Q** | Cancel current sort.

#### Display modes:
**Key** | **Sort**
--- | ---
**C** | Circle.
**B** | Bars.
**D** | Dots.
**P** | Pixels.
