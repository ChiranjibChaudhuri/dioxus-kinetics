//! Embedded Node.js scripts spawned by the PNG capture orchestrator.

pub const CAPTURE_CJS: &str = r#"// kinetics-render PNG capture script
//
// Usage: node capture.cjs <output-dir>
// Iterates output-dir/frames/*.html and writes output-dir/png/<i>.png
// via Playwright Chromium.

const path = require("path");
const fs = require("fs");

async function main() {
    const outDir = process.argv[2];
    if (!outDir) {
        console.error("Usage: node capture.cjs <output-dir>");
        process.exit(2);
    }
    const framesDir = path.join(outDir, "frames");
    const pngDir = path.join(outDir, "png");
    fs.mkdirSync(pngDir, { recursive: true });

    const playwright = require("playwright");
    const browser = await playwright.chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });

    const frames = fs
        .readdirSync(framesDir)
        .filter((f) => f.endsWith(".html"))
        .sort((a, b) => {
            const ai = parseInt(a, 10);
            const bi = parseInt(b, 10);
            return ai - bi;
        });

    for (const frame of frames) {
        const idx = parseInt(frame, 10);
        const fileUrl = "file://" + path.resolve(path.join(framesDir, frame));
        await page.goto(fileUrl, { waitUntil: "networkidle" });
        await page.screenshot({ path: path.join(pngDir, idx + ".png") });
    }

    await browser.close();
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
"#;
