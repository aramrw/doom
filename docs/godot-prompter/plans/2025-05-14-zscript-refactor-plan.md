# Refactor Plan: ZScript Parser Robustness (2025-05-14)

## Goal
Transform the ZScript parser from a strict validator into a resilient "data extractor" to handle varied Realm667 enemy definitions (like the Wicked) without failing the whole import.

## 1. Decouple `parse_state_frame`
Currently, `parse_state_frame` expects a strict format: `SpriteName` `Frames` `Duration` `[Modifiers]` `[Actions]`. This fails if actions are missing, arguments vary, or complex syntax is present.

- **New Logic:**
    - Parse `SpriteName` + `Frames` (The mandatory core).
    - Parse `Duration` (The mandatory core).
    - Attempt to parse `Actions` (Optional, loop until end of line).
    - If `Actions` parsing fails, log a warning and return the frame with an "Unknown" action status rather than returning a `nom::Err`.

## 2. Introduce `Action` Enum
- Instead of forcing `parse_function_call`, use an enum to distinguish between `KnownAction` (A_SpawnProjectile, A_PlaySound) and `UnknownAction` (the rest).
- The `resource_gen` will then be updated to handle `UnknownAction` by just warning the user instead of halting.

## 3. Implementation Steps
- [ ] Modify `parse_state_frame` to use `opt()` for action parsing.
- [ ] Update `parse_actor` loop to not panic on unknown properties, but log them as warnings.
- [ ] Ensure sprite frames remain 1:1 consistent with current logic.
- [ ] Verify Wicked enemy import after refactor.
