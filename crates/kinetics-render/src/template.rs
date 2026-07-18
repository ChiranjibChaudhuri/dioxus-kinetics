//! Embedded Node.js scripts spawned by the PNG capture orchestrator.

pub const CAPTURE_CJS: &str = r#"// kinetics-render PNG capture script
//
// Usage: node capture.cjs <output-dir> [width] [height]
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
    const width = parseInt(process.argv[3] || '1280', 10);
    const height = parseInt(process.argv[4] || '720', 10);
    const framesDir = path.join(outDir, "frames");
    const pngDir = path.join(outDir, "png");
    fs.mkdirSync(pngDir, { recursive: true });

    const playwright = require("playwright");
    const browser = await playwright.chromium.launch({ headless: true });
    const page = await browser.newPage({ viewport: { width, height } });

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

/// `node pdf.cjs <output-dir> <width> <height>` — loads the highest-numbered
/// (settled) HTML frame and writes `<output-dir>/report.pdf` via Playwright
/// Chromium's `page.pdf()`. Prints backgrounds so the inlined CSS renders.
pub const PDF_CJS: &str = r#"// kinetics-render PDF report script
//
// Usage: node pdf.cjs <output-dir> <width> <height>
// Loads the highest-numbered frame in output-dir/frames and writes
// output-dir/report.pdf.

const path = require("path");
const fs = require("fs");

async function main() {
    const outDir = process.argv[2];
    if (!outDir) {
        console.error("Usage: node pdf.cjs <output-dir> [width] [height]");
        process.exit(2);
    }
    const width = parseInt(process.argv[3] || '1280', 10);
    const height = parseInt(process.argv[4] || '720', 10);
    const framesDir = path.join(outDir, "frames");

    const frames = fs
        .readdirSync(framesDir)
        .filter((f) => f.endsWith(".html"))
        .sort((a, b) => parseInt(a, 10) - parseInt(b, 10));
    if (frames.length === 0) {
        console.error("no HTML frames to print");
        process.exit(1);
    }
    const settled = frames[frames.length - 1];
    const fileUrl = "file://" + path.resolve(path.join(framesDir, settled));

    const playwright = require("playwright");
    const browser = await playwright.chromium.launch({ headless: true });
    const page = await browser.newPage();
    await page.goto(fileUrl, { waitUntil: "networkidle" });
    await page.pdf({
        path: path.join(outDir, "report.pdf"),
        width: width + "px",
        height: height + "px",
        printBackground: true,
        margin: { top: "0", right: "0", bottom: "0", left: "0" },
        preferCSSPageSize: false,
    });
    await browser.close();
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
"#;
