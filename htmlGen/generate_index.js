import { readFileSync, writeFileSync } from "fs";
import { marked } from "marked";
import { gfmHeadingId } from "marked-gfm-heading-id";

const template = readFileSync("index_template.html").toString();
const readmeMd = readFileSync("../README.md").toString();

marked.use(gfmHeadingId());

const result = template.replace("{{readme-html}}", await marked(readmeMd));

writeFileSync("../index.html", result);
