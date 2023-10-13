import esbuild from "esbuild";
const typescriptEntries = ["src/code/js/tesseract.ts", "src/code/js/sine.ts"];
const CSSEntries = ["src/code/css/tesseract.css", "src/code/css/proxima.css"];
export const entries = [...typescriptEntries, ...CSSEntries];

export const esBuildContext = {
  sourcemap: true,
    entryPoints: entries,
  bundle: true,
  minify: false,
  loader: {
    ".png": "dataurl",
    ".woff": "dataurl",
    ".woff2": "dataurl",
    ".eot": "dataurl",
    ".ttf": "dataurl",
    ".svg": "dataurl",
  },
  outdir: "src/out",
} as esbuild.BuildOptions;
