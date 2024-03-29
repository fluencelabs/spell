#! /usr/bin/env node

const fs = require("fs").promises;
const path = require("path");

function printUsage() {
  console.log(
    `Usage: "ci bump-version %postfix%"`,
  );
}

let postfix;
const mode = process.argv[2];

function validateArgs() {
  switch (mode) {
    case "bump-version":
      postfix = process.argv[3];
      if (!postfix) {
        printUsage();
        return false;
      }
      return true;

    default:
      return false;
  }
}

const PATH_TO_PACKAGES = "./src/aqua/";

async function getPackageJsonsRecursive(currentPath) {
  return (
    await Promise.all(
      (await fs.readdir(currentPath, { withFileTypes: true }))
        .filter(
          (file) =>
            file.name !== "node_modules" &&
            file.name !== "integrations-tests" &&
            (file.isDirectory() || file.name === "package.json"),
        )
        .map((file) =>
          file.isDirectory()
            ? getPackageJsonsRecursive(path.join(currentPath, file.name))
            : Promise.resolve([
              path.join(process.cwd(), currentPath, file.name),
            ])
        ),
    )
  ).flat();
}

async function getVersion(file) {
  const content = await fs.readFile(file);
  const json = JSON.parse(content);
  return [json.name, json.version];
}

function processDep(obj, name, fn) {
  if (!obj) {
    return;
  }

  if (!obj[name]) {
    return;
  }

  fn(obj, obj[name]);
}
async function getVersionsMap(allPackageJsons) {
  return new Map(await Promise.all(allPackageJsons.map(getVersion)));
}

function getVersionForPackageOrThrow(versionsMap, packageName) {
  const version = versionsMap.get(packageName);
  if (!version) {
    console.log("Failed to get version for package: ", packageName);
    process.exit(1);
  }
  return version;
}

async function checkConsistency(file, versionsMap) {
  console.log("Checking: ", file);
  const content = await fs.readFile(file);
  const json = JSON.parse(content);

  for (const [name, versionInDep] of versionsMap) {
    const check = (x, version) => {
      if (version.includes("*")) {
        return;
      }

      if (versionInDep !== version) {
        console.log(
          `Error, versions don't match: ${name}:${version} !== ${versionInDep}`,
          file,
        );
        process.exit(1);
      }
    };
    processDep(json.dependencies, name, check);
    processDep(json.devDependencies, name, check);
  }
}

async function bumpVersions(file, versionsMap) {
  console.log("Updating: ", file);
  const content = await fs.readFile(file);
  const json = JSON.parse(content);

  // bump dependencies
  for (const [name, version] of versionsMap) {
    const update = (x) => (x[name] = `${version}-${postfix}`);
    processDep(json.dependencies, name, update);
    processDep(json.devDependencies, name, update);
  }

  // also bump version in package itself
  const version = getVersionForPackageOrThrow(versionsMap, json.name);
  json.version = `${version}-${postfix}`;

  const newContent = JSON.stringify(json, undefined, 4) + "\n";
  await fs.writeFile(file, newContent);
}

async function processPackageJsons(allPackageJsons, versionsMap, fn) {
  await Promise.all(allPackageJsons.map((x) => fn(x, versionsMap)));
}

async function run() {
  if (!validateArgs()) {
    printUsage();
    process.exit(1);
  }

  const packageJsons = await getPackageJsonsRecursive(PATH_TO_PACKAGES);
  const versionsMap = await getVersionsMap(packageJsons);

  // always check consistency
  console.log("Checking versions consistency...");
  await processPackageJsons(packageJsons, versionsMap, checkConsistency);
  console.log("Versions are consistent");

  if (mode === "bump-version") {
    console.log("Adding postfix: ", postfix);
    await processPackageJsons(packageJsons, versionsMap, bumpVersions);
    console.log("Done");
  }
}

run();
