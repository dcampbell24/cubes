#include <stdio.h>   // Needed only for sprintf()
#include <math.h>    // Needed for sin(), cos() and M_PI
#include <GL/glfw.h> // Takes care of everything GL-related

/* Some global variables for convenience */
double t0 = 0.0;
int frames = 0;
char titlestring[200];
// X, Y, Z rotations.
double Xrot = 0.0;
double Yrot = 0.0;
double Zrot = 0.0;
double ROT_SPEED = 0.05;

// 3-bit color palette
float COLOR[8][3] = {{0,0,0},{1,0,0},{0,1,0},{1,1,0},{0,0,1},{1,0,1},{0,1,1},{1,1,1}};

int PCUBE[27] = {
	6,  6,  6,
	6,  1,  2,
	1,  1,  2,

	4,  6,  5,
	4,  4,  5,
	1,  2,  2,

	3,  3,  3,
	4,  3,  5,
	4,  5,  5,
};

/* showFPS() - Calculate and report frames per second
 *   (updated once per second) in the window title bar
 */
void showFPS() {

    double t, fps;
	// Number of seconds since glfwInit()
    t = glfwGetTime();
    // If one second has passed, or if this is the very first frame
    if((t-t0) > 1.0 || frames == 0) {
        fps = (double)frames / (t-t0);
        sprintf(titlestring, "Polycube (%.1f FPS)", fps);
        glfwSetWindowTitle(titlestring);
        t0 = t;
        frames = 0;
    }
    frames++;
}

/* setupCamera() - set up the OpenGL projection matrix */
void setupCamera() {
    int width, height;
    // Get window size. It may start out different from the requested
    // size, and will change if the user resizes the window.
    glfwGetWindowSize( &width, &height );
    if(height<=0) height=1; // Safeguard against iconified/closed window

    // Set viewport. This is the pixel rectangle we want to draw into.
    glViewport( 0, 0, width, height ); // The entire window

    // Select and setup the projection matrix.
    glMatrixMode(GL_PROJECTION); // "We want to edit the projection matrix"
    glLoadIdentity(); // Reset the matrix to identity
    // 65 degrees FOV, same aspect ratio as window, depth range 1 to 100
    gluPerspective( 65.0f, (GLfloat)width/(GLfloat)height, 1.0f, 100.0f );

    // Select and setup the modelview matrix.
    glMatrixMode( GL_MODELVIEW ); // "We want to edit the modelview matrix"
    glLoadIdentity(); // Reset the matrix to identity
    // Look from 0,-5,0 towards 0,0,0 with Z as "up" in the image
    gluLookAt( 0.0f, -10.0f, 0.0f,    // Eye position
               0.0f, 0.0f, 0.0f,   // View point
               0.0f, 0.0f, 1.0f );  // Up vector
}

/* drawCube(r) - Draw an axis-aligned cube with size 2r by 2r by 2r, centered
 * on the local origin.
 */
void drawCube(float r) {
  glBegin(GL_QUADS);
    // +X face
    glNormal3f(1,0,0);
    glVertex3f(r,r,-r);
    glVertex3f(r,r,r);
    glVertex3f(r,-r,r);
    glVertex3f(r,-r,-r);
    // -X face
    glNormal3f(-1,0,0);
    glVertex3f(-r,r,r);
    glVertex3f(-r,r,-r);
    glVertex3f(-r,-r,-r);
    glVertex3f(-r,-r,r);
    // +Y face
    glNormal3f(0,1,0);
    glVertex3f(-r,r,r);
    glVertex3f(r,r,r);
    glVertex3f(r,r,-r);
    glVertex3f(-r,r,-r);
    // -Y face
    glNormal3f(0,-1,0);
    glVertex3f(-r,-r,r);
    glVertex3f(-r,-r,-r);
    glVertex3f(r,-r,-r);
    glVertex3f(r,-r,r);
    // +Z face
    glNormal3f(0,0,1);
    glVertex3f(-r,r,r);
    glVertex3f(-r,-r,r);
    glVertex3f(r,-r,r);
    glVertex3f(r,r,r);
    // -Z face
    glNormal3f(0,0,-1);
    glVertex3f(-r,r,-r);
    glVertex3f(r,r,-r);
    glVertex3f(r,-r,-r);
    glVertex3f(-r,-r,-r);
  glEnd();
}

void drawPolyCube(int pts[27], int len, float r) {
	int i;
	for(i = 0; i < len; i++) {
		glPushMatrix();
		  glTranslatef(i%3, (i/3)%3, i/9);
		  glColor3f(COLOR[pts[i]][0], COLOR[pts[i]][1], COLOR[pts[i]][2]);
	  	  glTranslatef(-1.0f, -1.0f, -1.0f); // Center the cube.
		  drawCube(r);
		glPopMatrix();
	}
}

/* drawScene() - the actual drawing commands to render our scene.  */
void drawScene() {
    float t = (float)glfwGetTime(); // Get elapsed time
    glPushMatrix();
      glRotatef(90.0f*Xrot, 1.0f, 0.0f, 0.0f); // Spin around X
      glRotatef(90.0f*Yrot, 0.0f, 1.0f, 0.0f); // Spin around Y
      glRotatef(90.0f*Zrot, 0.0f, 0.0f, 1.0f); // Spin around Z
      drawPolyCube(&PCUBE, 27, 0.5);
    glPopMatrix();
}

/* main(argc, argv) - the standard C entry point for the program */
int main(int argc, char *argv[]) {

    int running = GL_TRUE; // Main loop exits when this is set to GL_FALSE
    glfwInit();
    if(!glfwOpenWindow(640, 480, 8,8,8,8, 32,0, GLFW_WINDOW)) {
        glfwTerminate();
        return 1;
    }
    // Enable back face culling and Z buffering
    glEnable(GL_CULL_FACE);
    glEnable(GL_DEPTH_TEST);
	//glfwDisable(GLFW_AUTO_POLL_EVENTS)
    // Use "transparent black" (R=G=B=A=0) for the background color
    glClearColor(0.3f, 0.3f, 0.5f, 0.0f);
    // Main loop
    while(running)
    {
        showFPS();
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        setupCamera();
        drawScene();
        // Swap buffers, i.e. display the image and prepare for next frame.
        glfwSwapBuffers();
        // Check if the ESC key was pressed or the window was closed.
        if(glfwGetKey(GLFW_KEY_ESC) || !glfwGetWindowParam(GLFW_OPENED)) {
          running = GL_FALSE;
		} else if(glfwGetKey(GLFW_KEY_DOWN)) {
			Xrot += ROT_SPEED;
		} else if(glfwGetKey(GLFW_KEY_UP)) {
			Xrot -= ROT_SPEED;
		} else if(glfwGetKey(GLFW_KEY_LEFT)) {
			Yrot += ROT_SPEED;
		} else if(glfwGetKey(GLFW_KEY_RIGHT)) {
			Yrot -= ROT_SPEED;
		} else if(glfwGetKey('Z')) {
			Zrot += ROT_SPEED;
		} else if(glfwGetKey('X')) {
			Zrot -= ROT_SPEED;
		}
    }
    glfwTerminate();
    return 0;
}
