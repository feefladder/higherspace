import sqlite3
import bpy

id=3
con = sqlite3.connect("/home/feefladder/Git/higherspace/polydb.sqlite3")
cur = con.cursor()

def add_mesh(name, verts, faces, edges=None, col_name="Collection"):    
     if edges is None:
         edges = []
     mesh = bpy.data.meshes.new(name)
     obj = bpy.data.objects.new(mesh.name, mesh)
     col = bpy.data.collections[col_name]
     col.objects.link(obj)
     bpy.context.view_layer.objects.active = obj
     mesh.from_pydata(verts, edges, faces)

def pydata_from_db(id):
    verts = cur.execute("SELECT x,y,z FROM Vertex WHERE poly=?", (id,) ).fetchall()
    fids = [fid for fid, in cur.execute("SELECT face FROM Polygon WHERE poly=?", (id,) ).fetchall()]
    faces = [tuple(vert for vert, in cur.execute("SELECT vertex FROM Polygon WHERE poly=? AND face=?", (id, fid) ).fetchall()) for fid in fids]
    return verts, faces

def mesh_from_db(id):
     name, = cur.execute("SELECT longname FROM Polyhedron WHERE id=?", (id, )).fetchone()
     print(name)
     if name is None:
         return
     verts, faces = pydata_from_db(id)
     add_mesh(name, verts, faces)

mesh_from_db(id)
