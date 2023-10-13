import esbuild from "esbuild";
import { esBuildContext } from "./esbuild-config";

async function server() {
  const ctx = await esbuild.context(esBuildContext);
  const { host, port } = await ctx.serve({
    servedir: "src",
    port: 8080,
  });

  console.log(`Server running on http://${host}:${port}`);
}

void server();
