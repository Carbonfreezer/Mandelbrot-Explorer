# Mandelbrot Explorer Offline Experimental

This is an experimental branch that generates images with the rug library for arbitrary precision. As it is by its nature 
much slower it generates a sequence of png images. Be warned you need a fast computer and a lot of patience. 
The autofocus system is also included here. It is best to run the program on a Linux server. To get it compiled
you need to have several non rust libraries installed.

```bash
sudo apt update
sudo apt install libgmp-dev libmpfr-dev
sudo apt install build-essential m4
```

Once the program terminated you can generate a move from the sequence of images:
```bash
ffmpeg -framerate 50 -i Image_%06d.png -c:v libx264 -crf 18 -preset slow -pix_fmt yuv420p fractal.mp4
```
I find the life version in the main branch much more interesting.

