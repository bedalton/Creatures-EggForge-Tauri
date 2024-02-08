#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const version =  fs.readFileSync(path.join(__dirname, 'version.txt'), 'utf-8').trim();

const packageJsonPath = path.join(__dirname, 'package.json');
/**
 * @type {{version:string}}
 */
const packageJSON = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));

// Load old tauri config
const tauriConfPath = path.join(__dirname, 'src-tauri', 'tauri.conf.json');
/**
 * @type {{package:{version:string}}}
 */
let tauriContents = JSON.parse(fs.readFileSync(tauriConfPath, 'utf-8'));

if (version !== tauriContents.package.version) {
    tauriContents.package.version = version;
    const jsonString = JSON.stringify(tauriContents, null, '  ');
    fs.writeFileSync(tauriConfPath, jsonString, 'utf-8');
    console.log('Update tauri.conf.json version to ' + version)
}

if (version !== packageJSON.version) {
    packageJSON.version = version;
    const jsonString = JSON.stringify(packageJSON, null, '  ');
    fs.writeFileSync(packageJsonPath, jsonString, 'utf-8');
    console.log('Updating package.JSON version to ' + version);
}


const cargoTomlPath = path.join(__dirname, 'src-tauri', 'Cargo.toml');
let cargoTomlText = fs.readFileSync(cargoTomlPath, 'utf-8');
const tomlVersionRegex  = RegExp(/^version\s*=\s*"([0-9.]+)"\n/gmi);
const cargoMatch = tomlVersionRegex.exec(cargoTomlText);

if (cargoMatch == null || cargoMatch[1] !== version) {
    cargoTomlText = cargoTomlText.replace(tomlVersionRegex, 'version = "' + version + '"\n');
    fs.writeFileSync(cargoTomlPath, cargoTomlText, 'utf-8');
    console.log('Update Cargo.toml version to ' + version);
}

