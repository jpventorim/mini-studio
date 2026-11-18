# Foundry UI — paper design system

The design system takes its character from graph notebooks, printed reference
sheets, and handwritten session notes. It supports 25 base components. Light is
the active design target; the dark palette stays functional while its visual
review is deferred.

[All components](http://localhost:6006/?path=/story/design-system-all-components--default)
is the working system.

## Design purpose

These are controls for doing work and reading information. Familiar interaction,
clear labels, and dependable state changes take priority over decorative novelty.
Paper gives the composition a place; controls do not each need a material story.
Keep factual copy and existing component behavior intact during visual refinement.
Higher-level product features need their own requirements.

## Surfaces

- **Canvas:** pale graph paper with a quiet 24px square grid and 1px lines. It
  supplies the reference to notebooks. Keep the grid behind the work.
- **Working sheet:** warm white and plain. One thin edge and a shallow contact
  shadow establish a sheet. Text, menus, and overlays need quiet backgrounds.
- **Writing area:** a standalone box or an integrated baseline inside paper.
  Multiline ruling uses a 24px rhythm and stays below the text baseline.
- **Attachments:** one optional stationery clip on a Panel. It is decorative,
  does not intercept input, and never overlaps labels or focus indicators.

Do not repeat complete sheets around nested controls. Badge, Alert, Avatar, and
Tabs do not get sheet shadows or extra frames. Straight edges and crisp type do
most of the work. The default system uses no grain, photographic texture, random
noise, stains, or distressed text. Grids and ruling may disappear in forced colors
without removing information.

## Palette

Preserve these colors during the refinement pass. Blue ink is the default primary
action; semantic tones retain explicit text or symbols so color is never the only
state cue.

| Role                | Value     | Use                                    |
| ------------------- | --------- | -------------------------------------- |
| Canvas              | `#F1F0E9` | Graph-paper background                 |
| Sheet               | `#FFFEF8` | Reading and writing                    |
| Alternate sheet     | `#F6F4EC` | Quiet grouping                         |
| Primary ink         | `#282D32` | Text and strong outlines               |
| Secondary ink       | `#58616B` | Help and secondary labels              |
| Sheet edge          | `#C6C9C7` | Decorative sheet boundaries            |
| Control edge        | `#737F89` | Necessary interactive boundaries       |
| Grid line           | `#DCE4E6` | Decorative graph lines                 |
| Blue ink / sapphire | `#285A88` | Primary actions, selection, focus      |
| Red ink / ruby      | `#A13742` | Errors and destructive actions         |
| Green ink / emerald | `#2F684F` | Positive states                        |
| Ochre / gold        | `#E7C66B` | Restrained highlighting with dark text |

Tones are `sapphire`, `ruby`, `emerald`, `gold`, and `neutral`. Small status labels
can use pale tone tints and dark lettering. Gold text and symbols require dark
ink on pale surfaces. Decorative sheet edges and grid lines are not sufficient
interactive boundaries.

Measure actual states: 4.5:1 for ordinary text and 3:1 for large text and necessary
control/state indicators. Maintain legible disabled labels. Focus uses a distinct
3px ink outline, with space around the target and no clipping.

## Typography, spacing, and motion

Georgia provides short sheet and section headings; the system sans-serif handles
labels, controls, body text, and initials. Monospace is reserved for code and
measured values. Avoid added type styles merely to make controls look special.
Use a 15px body with comfortable line height, an 8px spacing rhythm, and 4px
subdivisions. Use normal UI density and let wrapping content grow vertically.
Do not add decorative kickers or sequence numbers above headings.

Visible marks can be small while their targets remain at least 44px. Radio marks,
slider thumbs, and switch tracks use larger invisible targets. State changes
should be immediate; brief transitions clarify movement rather than stage a
performance. Reduced motion removes transitions and looping decoration.

## Component vocabulary

| Component  | Current treatment                                                    |
| ---------- | -------------------------------------------------------------------- |
| Panel      | Shallow paper edge with an optional `clipped` variant                |
| Separator  | Fine printed rule in either orientation                              |
| Button     | Ink-filled primary, outlined secondary, quiet text action            |
| Badge      | Compact typeset label with a pale tone tint; no accent strip         |
| Avatar     | Circular portrait or white initials on blue ink; no outer frame      |
| Alert      | Clear status icon, title, and message; no accent rail or raised card |
| TextField  | Standalone writing box or baseline integrated in paper               |
| Textarea   | Resizable writing area with text-aligned ruling                      |
| Checkbox   | Printed square with a crisp ink check or mixed dash                  |
| Switch     | Small track and thumb; position and contrast show state              |
| RadioGroup | Stacked options with full-row focus and a filled selection mark      |
| Select     | Writing box or baseline with a check mark and inset menu focus       |
| Slider     | Continuous line and round thumb; no decorative tick marks            |
| Tabs       | Unframed text index, selected underline, and open content            |
| Accordion  | Printed section headings and rules revealing the same sheet          |
| Dialog     | Raised working sheet with a quiet scrim and clear close action       |
| Popover    | Small reference note and a directional pointer                       |
| Tooltip    | Compact high-contrast ink label                                      |
| Progress   | Printed track and ink fill with explicit value/status                |

The desktop canvas uses NumberField for dimensions and quantity, SelectableRow
for image entries, EmptyState for the clickable drop target and blank sheet,
Readout for print dimensions, Notice for feedback, and PreviewControls for
paging and zoom. Storybook shows the controls independently of the workspace.

The selected tab underline moves between labels in 160ms using the shared easing.
Reduced motion places it immediately. It carries selection without ornamenting
every tab.
Status icons differ by meaning. Switch labels name the setting; the switch's
accessible checked state supplies on/off information without printed words.

## Implementation contract

`src/theme/contracts.ts` owns scoped theme values and paper ink aliases;
`foundation.ts` owns shared rules; `materials.ts` owns the graph-paper background.
Keep Panda defaults aligned with scoped light values. Component-specific styles
stay beside each component. Reuse semantic surface, edge, ink, focus, and depth
roles instead of adding per-component palettes.

`ThemeProvider` preserves nested scopes and portal theme inheritance. Theme
switches do not change geometry or lose form values. Components on paper use
paper ink, including inside a dark canvas.

Panel is paper-only and accepts `clipped`. `TextField`, `Textarea`, and `Select`
retain `surface="auto"`, `"mounted"`, and `"integrated"`: auto integrates inside
paper and uses a writing box outside it; explicit values override this behavior.

## Accessibility and verification

Keep Base UI semantics, labels, linked help/errors, keyboard behavior, focus
visibility, and overlay behavior. Use native semantic elements for simple content.
Decorations stay out of the accessibility tree and do not intercept input.
Dialogs preserve focus trapping, Escape, and focus restoration; tooltips never
carry essential information alone.

Review empty/filled, selected, invalid, readonly, disabled, and open states where
supported. Include long labels, narrow layouts, keyboard input, reduced motion,
and forced colors. Run focused browser checks per component and the full suite
at integration milestones. Automated checks supplement rendered inspection;
screen-reader and product-context review remain necessary.

## Iteration

Keep changes small: translate a component, inspect its states and context, then
resolve details and verify. Use the existing palette and Button treatment as anchors when deciding
whether a change improves clarity. Record what was actually reviewed and tested
in the commit history. User design acceptance is separate from agent verification.
The palette and Button are the anchors for this pass. Continue dark visual design
after the light review. The desktop shell and new product features are outside
this design-system work.
