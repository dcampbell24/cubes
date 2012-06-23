#/usr/env python

import bpy
import os
import re

# Delete the cube that starts in the default scene.
bpy.ops.object.delete()
is_obj = re.compile("^.*\.obj$")
is_mtl = re.compile("^.*\.mtl$")

if bpy.ops.import_scene.obj:
    files = os.listdir()
    for f in files:
        if is_obj.match(f):
            bpy.ops.import_scene.obj(filepath=f)

#bpy.ops.anim.change_frame(frame=0)
#bpy.ops.wm.quit_blender()
