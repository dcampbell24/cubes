cc `pkg-config --cflags libglfw` -o prog $1 -lglut -lGLU -lm `pkg-config --static --libs libglfw`
