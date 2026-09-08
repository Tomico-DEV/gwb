# External dependencies

For now, I will make use of the `nalgebra` library for numeric operations.
However, I will not call use the library directly in any code except
that of the lowest level, where all objects and calls will be wrapped around
code belonging to this project. This way, if the library is found to be inadequate
in the future, it can simply be changed out for a different library or an entirely new custom
implementation can be created instead.


# Types

Because the underlying representation of an integer or a real may very well be changed downstream,
I will make use of typealises to ensure that a transition is as clean as possible. These are located
under `core`.