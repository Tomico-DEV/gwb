# Half-Edge Data Structures

Finally, we can discuss the underlying data structure that will be used to represent
objects.

GWB makes use the half-edge data structure

Recall from the section on [plane models](plane_models.md), that a plane model
is essetially a planar directed graph of vertices, directed edges, and polygons,
and has special topology through the joining or identification of edges.

Our data structure must be capable of representing this.

## Graph Representation

To represent the graph of the plane model, the half-edge data structure contains the following nodes:

- Solid
    - Represents a single solid; owns edges, vertices, faces, and everything else
- Face
    - Represents a planar or flat face
    - In GWB, this is defined as a planar polygon whose interior is connected
    - Can have one outer loop and multiple rings (holes)
    - Belongs to a solid
- Loop
    - A connected boundary (a collection of connected edges that loops)
    - Belongs to a face
- HalfEdge
    - Basically an edge (a line segment)
    - Has one vertex vertex; instead of storing two vertices to represent an edge,
      it instead knows about the next halfedge and the previous half edge 
    - Belongs to a loop
- Vertex


Note that all of these nodes are implemented in a doubly-linked list; each node knows about
its next and previous neighbor (this is stated explicitly in the HalfEdge node). This allows
us to iterate through say, every vertex in a solid, or every solid in memory.

## Identification

To store information on identification (joining of edges), we will need to introduce new nodes
and extend existing ones as well.

- Edge
    - The reason we call edges "half-edges" above is from the property 
      of [surface subdivisions](plane_models.md#surface-subdivision), where each edge must be 
      identified with exactly one other edge. So a "complete" edge is really just two edges,
      therefore it makes sense to call edges half-edges instead since they're only considered complete with identified with each other.
    - An Edge node associates two HalfEdges together by storing pointers to the "left" and "right" half-edges 
- HalfEdge
    - We update the half-edge accordingly by including information about the edge it belongs to
- Vertex
    - We include information about one of the half-edge it belongs to


**Note: the words "belongs to" and "owns" used in this here is not in the context of Rust ownership;
that is discussed in a later section**


# Rust Implementation

As the half-edge data structure makes use of doubly-linked lists, they are... impossible to
implement "safely" in a way Rust wants. Rust ownership requires a tree-like structure but
features like identification just makes this impossible as one object may have multiple references.

Instead, we will make use of an "arena". `SlotMap`s seem to be the best suited for this.
One parent object owns all of the data in a model,
and they are referenced through IDs. Yes, this circumvents Rust's ownership mechanism as 
you can definitely cause dangling references through the removal of an object that still has references,
but this will not cause a traditional undefined behavior where the program becomes unsafe.

Instead, we can ensure this never happens by only allowing the creation and deletion of objects
through Euler operations, and if there is a bug or an oversight, we can handle cases where
a reference is missing without ever causing undefined behavior.

## Topology and Geometry

GWB makes a distinction between topology and geometry, and to the average math n00b (me),
they sound like the same thing. So I'll make this distinction clear here:

**Topology**: How things are *connected*

**Geometry**: Where things are

Now, the true math nerds of you out there will be ready to jump out of your chair and yell at
me saying I am wrong; there's no need, I know it is but for all intents and purposes, this is
good enough.

Under the geometry module, we store things that are coordinate-based, such as

- Point
- Vector
- Transform

Etc (still thinking about what we need)

and under the topology module, we will keep nodes like

- Vertex
- HalfEdge
- Edge
- Loop
- Face
- Solid

A vertex would not store its x, y, and z coordinates explicitely,
instead it would have a `Point` for its actual coordinate.

This distinction makes it more obvious which operations operate on
topology and which operations operate on geometry. I think. I haven't
made this yet so I can't say for sure haha.
