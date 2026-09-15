Here are some general design decisions I have made
during the development of GWB.

# Accessibility

Users are expected to only use operators to create solids instead of operating
on the underlying data directly, which can cause corrupted or degenerate states.

Because of this, only the operators are made public in the Topology module.

# Panics in low-level euler operators

This is hard decision. Do I return an None when the operation is invalid and therefore
will not be carried out? Or do I make the code panic because any high-level
operator that uses these operators should not provide invalid input to begin with?
Will the users be allowed to use these low-level operators?

As I'm not sure at the time of writing since I'm still developing this, I have decided
to make them panic instead. Hopefully this won't be too much of a pain to change
to Option<> if I find there is a need for it later on.

# Mutability

Solid used to own both the getter, setters, and topology convenience functions such as `get_he_twin`, but it became
cluttered and awkward to use fast. I've decided to move
these topology functions to `Topology`, a view into the solid.

One major problem here is mutability. Sometimes we'll only want
to *read* our solid, and other times we may want to *write* to it.
Both of these comes with different considerations, and making
two types of views (eg. `Topology` and `TopologyMut`) is not ideal.

This is where I made use of generics and traits. A `Topology` holds
a reference to a solid which through the use of generics, could be a `&Solid` or a `&mut Solid`. Then, two traits `TopologyRead` and
`TopologyWrite` were created, grouping functions that don't modify
the solid and functions that do respectively.

So to actually do stuff with a solid, you would first create a solid, then get a `Topology` context through some method and
operate on the solid. Never operate directly on a solid!!
