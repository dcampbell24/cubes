# drawCube(r) - Draw an axis-aligned cube with size 2r by 2r by 2r, centered
# on the local origin.
glfw = dlopen("/usr/lib/libglfw.so.2.6")
const GLFW_WINDOW     = 0x00010001
const GLFW_FULLSCREEN = 0x00010002

const GL_QUADS = int(0x7)

function glfwInit()
    ccall(dlsym(glfw, :glfwInit), Int, ())
end

function glNormal3f{T<:Number}(x::T, y::T, z::T)
    ccall(dlsym(glfw, :glNormal3f), Int, (Float32, Float32, Float32),
        float32(x), float32(y), float32(z))
end

function glVertex3f{T<:Number}(x::T, y::T, z::T)
    ccall(dlsym(glfw, :glNormal3f), Int, (Float32, Float32, Float32),
        float32(x), float32(y), float32(z))
end

function glBegin(mode::Int)
    ccall(dlsym(glfw, :glBegin), Int, (Int,), int(mode))
end

function glEnd()
    ccall(dlsym(glfw, :glEnd), Int, ())
end

function glfwOpenWindow(width::Int, height::Int, redbits::Int, greenbits::Int,
            bluebits::Int, alphabits::Int, depthbits::Int, stencilbits::Int, mode::Uint)
        ccall(dlsym(glfw, :glfwOpenWindow), Int,
            (Int, Int, Int, Int, Int, Int, Int, Int, Uint),
            width, height, redbits, greenbits, bluebits,
            alphabits, depthbits, stencilbits, mode)
end

function drawCube(r::Float32)
  glBegin(GL_QUADS)
    # +X face
    glNormal3f(1,0,0)
    glVertex3f(r,r,-r)
    glVertex3f(r,r,r)
    glVertex3f(r,-r,r)
    glVertex3f(r,-r,-r)
    # -X face
    glNormal3f(-1,0,0)
    glVertex3f(-r,r,r)
    glVertex3f(-r,r,-r)
    glVertex3f(-r,-r,-r)
    glVertex3f(-r,-r,r)
    # +Y face
    glNormal3f(0,1,0)
    glVertex3f(-r,r,r)
    glVertex3f(r,r,r)
    glVertex3f(r,r,-r)
    glVertex3f(-r,r,-r)
    # -Y face
    glNormal3f(0,-1,0)
    glVertex3f(-r,-r,r)
    glVertex3f(-r,-r,-r)
    glVertex3f(r,-r,-r)
    glVertex3f(r,-r,r)
    # +Z face
    glNormal3f(0,0,1)
    glVertex3f(-r,r,r)
    glVertex3f(-r,-r,r)
    glVertex3f(r,-r,r)
    glVertex3f(r,r,r)
    # -Z face
    glNormal3f(0,0,-1)
    glVertex3f(-r,r,-r)
    glVertex3f(r,r,-r)
    glVertex3f(r,-r,-r)
    glVertex3f(-r,-r,-r)
  glEnd()
end

function main()
    glfwInit()
    glfwOpenWindow(300, 300, 0, 0, 0, 0, 0, 0, GLFW_WINDOW)
    drawCube(float32(0.5))
end

#main()
