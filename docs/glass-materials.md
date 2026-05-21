# Glass Materials

Glass is represented through tokenized material recipes.

Material axes:

- `GlassLevel`
- `GlassTone`
- `GlassDensity`
- `GlassPolicy`

Web, Desktop, and Mobile use `backdrop-filter` where supported. When
backdrop filtering is unavailable, the same recipe resolves to a solid
fallback. Native targets use the same recipe and map it to available
renderer capabilities.

Text and icons are validated against fallback surfaces, not only ideal
blurred surfaces.

## Native Material Names

The public material API uses functional names:

- `GlassLayer`
- `MaterialRequest`
- `MaterialTone`
- `MaterialDensity`
- `MaterialPolicy`
- `GlassDepth`

Existing glass recipe types remain available during the `0.1.x` transition,
but docs and examples prefer the functional material vocabulary.
