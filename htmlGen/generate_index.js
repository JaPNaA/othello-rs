import { readFileSync, writeFileSync } from "fs";
import { marked } from "marked";

const template = readFileSync("index_template.html").toString();
const readmeMd = readFileSync("../README.md").toString();

const result = template.replace("{{readme-html}}", await marked(readmeMd));

writeFileSync("../index.html", result);
