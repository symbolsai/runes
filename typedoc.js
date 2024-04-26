// typedoc.js
/**
 * @type {import('typedoc').TypeDocOptions}
 */
module.exports = {
  entryPoints: ["./pkg/runes.d.ts"],
  plugin: ["typedoc-plugin-markdown"],
  readme: "none",
  githubPages: false,
  gitRevision: "master",
};
