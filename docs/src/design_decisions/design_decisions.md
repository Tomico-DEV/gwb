# General Design Decisions

## Accessibility

Users are expected to only use operators to create solids instead of operating
on the underlying data directly, which can cause corrupted or degenerate states.

Because of this, only the operators are made public in the Topology module.

## Panics in low-level euler operators

This is hard decision. Do I return an None when the operation is invalid and therefore
will not be carried out? Or do I make the code panic because any high-level
operator that uses these operators should not provide invalid input to begin with?
Will the users be allowed to use these low-level operators?

As I'm not sure at the time of writing since I'm still developing this, I have decided
to make them panic instead. Hopefully this won't be too much of a pain to change
to Option<> if I find there is a need for it later on.
