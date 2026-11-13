# assumptions

Not everything is built on a well-defined, well-understood foundation; sometimes
you have to make assumptions about how something works, with the logic of your
program built implicitly on top of those assumptions. You can add comments that
attempt to document the assumptions you're making, but comments don't run; you
still need to handle the case where they turn out to be wrong. That's what this
crate is for: `assumptions` provides `Assumption`, a single error type
representing "an invariant about the external world which was violated."

## External Assumptions

Consider parsing a webtoon homepage on `webtoons.com`: The page contains a
stats block with views and subscribers:

```html
<ul class="grade_area">
  <li>
    <span class="ico_view">view</span>
    <em class="cnt">10.8M</em>
  </li>
  <li>
    <span class="ico_subscribe">subscribe</span>
    <em class="cnt">1.3M</em>
  </li>
</ul>
```

Both values share the same `em.cnt` selector, so any parser implicitly relies
on the order that they are assumed to be in: first occurrence is views, second
is subscribers.

This is invisible in the code. There's no type or structure enforcing it; the
number parser works identically for both. If `webtoons.com` ever reorders the block,
the program can silently return wrong data.

A comment above the call site tells you what was assumed, but it doesn't help
when the assumption breaks:

```rust,ignore
// First `em.cnt` is views, second is subscribers.
let views = html.select(&selector).next()...;
let subscribers = html.select(&selector).nth(1)...;
```

Instead, we can use `assumptions`:

```rust,ignore
fn views(html: &Html) -> Result<u64, WebtoonError> {
    let selector = Selector::parse("em.cnt")
        .expect("`em.cnt` should be a valid CSS selector");

    let element = html
        .select(&selector)
        .next()
        // If `webtoons.com` were to remove/rename the element, then this will return early (alerting you).
        .assumption("there should be at least one `em.cnt` element on a webtoon homepage")?;

    assume_eq!(
        element
            .prev_sibling_element()
            // We assume that `em.cnt` is a nested element; if this changes, we are now alerted.
            .assumption("views `em.cnt` should be preceded by a `span` label element")?
            .inner_html()
            .as_str(),
        // If the label (which we are now more sure that we are actually looking at)
        // is not "view," then our assumptions of the webpage are no longer valid.
        "view",
        "label preceding first `em.cnt` on webtoon homepage should be `view`"
    );

    let views = element.inner_html();
    // We assume that `em.cnt` will always be populated with a string that can be
    // parsed as a number, but maybe for new webtoons they change to show nothing
    // in the future? Now we check (and it is clear in the code to future readers).
    assume!(!views.is_empty(), "views `em.cnt` should never contain empty text");

    // ... parse and return
}
```

## Where this fits

| Choice | Use for | Actionable by caller |
|---|---|---|
| `assert!` / `debug_assert!` | Invariants about your own logic | No; panics |
| **`assume!` / `Assumption`** | **Invariants about an external, uncontrolled system** | **No; always a bug** |
| `thiserror` variants | Expected, recoverable failure modes | Yes; typed, matchable |
| `anyhow::Error` | Any error at the application boundary | No; opaque |

`assert!` and `debug_assert!` are for logic that must be true given your own
code; a failing assert is your bug, there is nothing to recover from at
runtime, so it panics.

`thiserror` variants are for failures that are expected and recoverable: A file
that might not exist, a network request that might time out. The caller gets a
typed error back they can match on and handle.

`anyhow` is for the outer boundary of an application, where you've stopped
caring about error types and just want to surface anything that went wrong.

`Assumption` fits between `assert!` and `thiserror`: Like `assert!`, a
violated assumption is always a bug, but the bug is triggered by an external
system changing rather than a flaw in your own logic (the previous logic may have
been correct but is now incorrect); like `thiserror`, it returns an error rather
than panicking, because an upstream change shouldn't bring down the whole process;
unlike `thiserror`, it isn't typed or recoverable.

Every `Assumption` means the same thing: Something you relied on changed.

The right place to use `Assumption` is exactly at the point where your code
crosses into something you don't control (in the same vein as "pushing if's up"):
parsing a response, reading a selector, interpreting a byte sequence.

If you're reaching for `Assumption` inside internal logic, `assert!` is the better fit.

## Recommended message style

Assumption messages are best written in the **"expect as precondition"** style,
as described in the standard library documentation for [`Option::expect`] and
[`Result::expect`]: state what *should* be true, not what went wrong.

```rust,ignore
// Preferred
assume!(!panels.is_empty(), "episode should have at least one panel in order to be published");
bin.starts_with(b"dissect").assumption("`.rec` binary should always start with `dissect` magic bytes")?;
assume_eq!(label.as_str(), "view", "label preceding first `em.cnt` should be `view`");

// Should avoid
assume!(!panels.is_empty(), "panels list is empty");
bin.starts_with(b"dissect").assumption("wrong magic bytes")?;
assume_eq!(label.as_str(), "view", "label wasn't `view`");
```

**Hint**: The right phrasing depends on which choice you are using. For the
condition-checking macros (`assume!`, `assume_eq!`, `assume_ne!`,
`assume_matches!`), imagine the message completes the sentence "assume
that…" and write it from there. For `assumption!` and the trait methods
(`.assumption()`, `.with_assumption()`), you're not completing a sentence
about a check, you're stating the invariant directly as a positive claim
about what should have been true at that point.
