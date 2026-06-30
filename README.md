# assumptions

Assumptions about a world you don't control.

Not everything is built on a well-defined, well-understood foundation.
Sometimes you have to make assumptions about how something works, and the
logic of your program is implicitly built on top of those assumptions.

You can add comments that attempt to document the assumptions you're making,
but comments don't run. You still need to handle the case where they turn out
to be wrong, either now or in the future when something upstream changes
without warning. And when that happens, a stale comment is worse than no
comment: it tells you what used to be true, not what is.

That's what this crate is for. `assumptions` provides [`Assumption`], a single
error type representing "an invariant about the external world which was
violated." The macros and trait methods let you document and enforce implicit
assumptions right at the boundary between the outside world you don't control
and the logic that depends on its assumed shape. The check becomes the
documentation, but it can't go stale, and when it fires, it tells you exactly
where and what went wrong.

## The problem

Consider parsing a webtoon homepage on `webtoons.com`. The page contains a
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
on the *order*: first occurrence is views, second is subscribers.

This is invisible in the code, and there's no type or structure enforcing it; the
number parser works identically for both. If Webtoons ever reorders the block,
both parsers return wrong data silently, with no error.

A comment above the call site tells you what was assumed, but it doesn't help
when the assumption breaks:

```rust,ignore
// First em.cnt is views, second is subscribers.
let views = html.select(&selector).next()...;
let subscribers = html.select(&selector).nth(1)...;
```

## The solution

Make the assumption executable, checked at the point where the order is relied
upon:

```rust,ignore
fn views(html: &Html) -> Result<u64, WebtoonError> {
    let selector = Selector::parse("em.cnt")
        .expect("`em.cnt` should be a valid CSS selector");

    let element = html
        .select(&selector)
        .next()
        .assumption("there should be at least one `em.cnt` element on webtoon homepage")?;

    // The selector matches both views and subscribers by order alone.
    // Verify the preceding label so a reordering fails loudly rather than
    // silently returning the subscriber count as views.
    let label = element
        .prev_sibling_element()
        .assumption("views `em.cnt` should be preceded by a `span` label element")?
        .inner_html();

    assume_eq!(
        label.as_str(), "view",
        "label preceding first `em.cnt` on webtoon homepage should be `view`"
    );

    let views = element.inner_html();
    assume!(!views.is_empty(), "views `em.cnt` should never contain empty text");

    // ... parse and return
}
```

Now if Webtoons reorders the block, the label check fires immediately:

```text
internal assumption violated at src/webtoon/page.rs:42:5: label preceding first `em.cnt` on webtoon homepage should be `view`: left = "subscribe", right = "view"
```

## Where this fits

| Tool | Use for | Actionable by caller |
|---|---|---|
| `assert!` / `debug_assert!` | Invariants about your own logic | No; panics |
| **`assume!` / `Assumption`** | **Invariants about an external, uncontrolled system** | **No; always a bug** |
| `thiserror` variants | Expected, recoverable failure modes | Yes; typed, matchable |
| `anyhow::Error` | Any error at the application boundary | No; opaque |

`assert!` and `debug_assert!` are for logic that must be true given your own
code. A failing assert is your bug, there is nothing to recover from at
runtime, so it panics.

`thiserror` variants are for failures that are expected and recoverable: a file
that might not exist, a network request that might time out. The caller gets a
typed error back they can match on and handle.

`anyhow` is for the outer boundary of an application, where you've stopped
caring about error types and just want to surface anything that went wrong.

`Assumption` fits between `assert!` and `thiserror`. Like `assert!`, a
violated assumption is always a bug, but the bug is triggered by an external
system changing rather than a flaw in your own logic. Like `thiserror`, it
returns an error rather than panicking, because an upstream change shouldn't
bring down the host process. Unlike `thiserror`, it isn't typed or recoverable;
every `Assumption` means the same thing: something you relied on changed, go
find it and fix it.

The right place to use `Assumption` is exactly at the point where your code
crosses into something you don't control: parsing a response, reading a
selector, interpreting a byte sequence. If you're reaching for `Assumption`
inside internal logic, `assert!` is the better fit.

## Recommended message style

Assumption messages are best written in the **"expect as precondition"** style,
as described in the standard library documentation for [`Option::expect`] and
[`Result::expect`]: state what *should* be true, not what went wrong.

```rust,ignore
// Good: States the precondition. Immediately clear what was expected.
assume!(!panels.is_empty(), "episode should have at least one panel in order to be published");
bin.starts_with(b"dissect").assumption("`.rec` binary should always start with `dissect` magic bytes")?;
assume_eq!(label.as_str(), "view", "label preceding first `em.cnt` should be `view`");

// Avoid: Describes the failure. Forces the reader to infer the assumed invariant.
assume!(!panels.is_empty(), "panels list is empty");
bin.starts_with(b"dissect").assumption("wrong magic bytes")?;
assume_eq!(label.as_str(), "view", "label wasn't `view`");
```

**Hint**: The right phrasing depends on which tool you're using. For the
condition-checking macros (`assume!`, `assume_eq!`, `assume_ne!`,
`assume_matches!`), imagine the message completes the sentence "assume
that..." and write it from there. For `assumption!` and the trait methods
(`.assumption()`, `.with_assumption()`), you're not completing a sentence
about a check — you're stating the invariant directly as a positive claim
about what should have been true at that point.
