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
