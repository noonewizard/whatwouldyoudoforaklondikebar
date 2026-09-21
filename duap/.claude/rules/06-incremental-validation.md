# Incremental validation

A subsystem lands in increments that each compile, test and are inspectable.

## Requirements

1. Every commit leaves `cargo test --workspace` passing. A commit that does
   not is a defect regardless of what follows it.
2. A new subsystem lands with its tests in the same commit. "Tests to
   follow" means the design has not been validated.
3. A commit that adds more than roughly a thousand lines of implementation
   without tests is split.
4. Each subsystem follows the same loop before it is considered done:

   ```
   implement -> compile -> test -> attack -> benchmark -> document -> integrate
   ```

   The `attack` step is `red-team-engineer`'s, and it is not optional.
5. Generated files are regenerated, not hand-edited, and the regeneration is
   checked in CI.

## Why the loop has `attack` before `benchmark`

Optimising code that is wrong wastes the optimisation, and a security fix
frequently changes the performance profile. Establish correctness against an
adversary first.
