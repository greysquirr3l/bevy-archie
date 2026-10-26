# Virtual keyboard: `EditableText` integration decision

> **Status:** Decided. Path C (skip the `EditableText` migration), with the grapheme-cluster
> bug the investigation surfaced fixed.
> **Scope:** [issue #37](https://github.com/greysquirr3l/bevy-archie/issues/37) ·
> **Module:** `src/virtual_keyboard.rs` (feature `virtual_keyboard`)

## Summary

Bevy 0.19+ ships `EditableText`, a component that owns a text buffer, cursor, and selection.
`bevy_archie`'s virtual keyboard has its own buffer and cursor, so the two overlap. This note
records the spike, the measurements, and the decision.

## What `EditableText` actually provides

From `bevy_text-0.20.0/src/editing.rs`:

| `EditableText` member                                                    | Virtual keyboard equivalent                               | Verdict                                                                                |
| ------------------------------------------------------------------------ | --------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `editor: PlainEditor<TextBrush>`                                         | `VirtualKeyboard::buffer` + `cursor`                      | Overlaps                                                                               |
| `pending_edits: Vec<TextEdit>`                                           | Direct mutation of the buffer                             | Overlaps                                                                               |
| `max_characters: Option<usize>`                                          | `VirtualKeyboard::max_length`                             | Overlaps                                                                               |
| `EditableTextFilter`                                                     | `allow` / `exclude`                                       | Overlaps, with different semantics                                                     |
| `TextEdit::Insert` / `Backspace` / `Left` / `Right`                      | `add_char` / `backspace` / `cursor_left` / `cursor_right` | Overlaps                                                                               |
| Clipboard, IME composition, multi-click selection, physical-key handling | —                                                         | **Orthogonal** — the virtual keyboard has no physical keyboard, no IME, and no pointer |

`TextEdit` has no "set text" variant; `EditableText::clear()` is the closest.

## The spike

Two spikes were run against `bevy_text-0.20.0`.

### Spike 1 — the current `VirtualKeyboard` with non-ASCII input

```rust
let mut kb = VirtualKeyboard::new("test");
kb.add_char('é');
kb.add_char('a'); // panics
```

```text
buffer after é: "é" cursor=1
panicked at src/virtual_keyboard.rs:171:21:
assertion failed: self.is_char_boundary(idx)
```

**`VirtualKeyboard` panicked on multi-byte input.** `add_char` inserted at `self.cursor`, which
was a *byte* offset, but advanced the cursor by one *character*. After typing a two-byte `é` the
cursor pointed into the middle of the character, and the next `String::insert` tripped the
char-boundary assertion. Any user typing a name with an accent in a controller-driven text field
hit a crash. The same flaw made `backspace` delete a single *byte* rather than a character.

### Spike 2 — `EditableText` with the same input

```rust
editable.queue_edit(TextEdit::Insert("é".into()));
editable.queue_edit(TextEdit::Insert("a".into()));
// => "éa"
```

`EditableText` handled it correctly, because `parley::PlainEditor` is grapheme-aware.

A second spike showed `EditableTextFilter` semantics differ from `allow`/`exclude`: a filter
rejects an entire `Insert` if **any** character in it fails the filter, whereas
`VirtualKeyboard::is_char_allowed` filters per character. `Insert("1a2")` with a digit-only filter
produced `""`, not `"12"`. Any migration would have to keep per-character filtering in the
keyboard and leave `EditableTextFilter` unused.

## Decision: Path C (skip), plus a fix

### Why Path A is not implementable as written

Path A assumed an `EditableText` on "the displayed text entity". **There is no such entity.**
The virtual keyboard has no rendering at all — no key grid, no text field, no cursor display, and
no example. It is a state machine (`VirtualKeyboard` resource) plus an input system
(`handle_keyboard_input`) that emits `VirtualKeyboardEvent`. A sync layer would copy a string into
a component nobody reads.

### Why Path B's cost outweighs its benefit today

Path B makes `EditableText` the source of truth. That requires:

1. **A text-field entity concept that does not exist yet** — the keyboard would need to know which
   entity holds the `EditableText`, which means a new public field and a new way to construct
   `ShowVirtualKeyboard`.
2. **A public API break** — `buffer` and `cursor` are public fields, and `value()`,
   `add_char()`, `backspace()`, `cursor_left()`, `cursor_right()`, and `clear()` are public
   methods with ~30 unit tests depending on them.
3. **Moving editing into Bevy's `PostUpdate` pipeline** — `apply_text_edits` runs in `PostUpdate`,
   so a value read in the same frame a key was pressed would lag one frame behind. Avoiding that
   would mean the keyboard system calling `apply_pending_edits` directly, which drags in
   `FontCx`, `LayoutCx`, and `Clipboard`.
4. **A dependency the module does not currently have** — `EditableText` needs `TextPlugin`,
   `AssetPlugin`, and Bevy's clipboard stack.

All of that is spent on a feature that currently renders nothing. The overlap that *does* matter
— caret rendering, selection, scrolling for long text, and grapheme-correct editing — only becomes
reachable once a text field exists to render.

### What was done instead

The grapheme-cluster model was adopted directly, so the module now behaves the way `EditableText`
does and a future migration is mechanical:

- `VirtualKeyboard::cursor` is now a **grapheme-cluster index**, not a byte offset. This is a
  change in meaning for a public field, documented on the field itself.
- `max_length` is counted in grapheme clusters, matching `EditableText::max_characters`.
- `add_char`, `backspace`, `cursor_left`, `cursor_right`, `clear`, and `with_value` are all
  grapheme-aware.
- `unicode-segmentation` was added as a dependency.

The panic is fixed and covered by six new regression tests, including ZWJ emoji sequences
(`👨‍👩` is one grapheme and is deleted as one unit).

### When to revisit

Path B becomes the right answer when the virtual keyboard gains a rendered text field. At that
point:

1. Add a `VirtualKeyboardTarget(Entity)` (or similar) so the keyboard knows where to write.
2. Replace the buffer mutation calls with `TextEdit::Insert` / `Backspace` / `Left` / `Right`.
3. Move `VirtualKeyboard::value()` out of the resource and into a system that reads the
   `EditableText`, so the value is read after `apply_text_edits` has run.
4. Keep the per-character `allow` / `exclude` filtering in the keyboard — `EditableTextFilter`
   has different semantics.
