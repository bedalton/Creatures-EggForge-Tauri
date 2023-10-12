#!/usr/bin/env node
const {exit} = require('node:process');
const os = require('node:os');
const fs = require('node:fs');
const _fs = require('fs-extra');
const path = require('node:path');

if (os.platform() === 'darwin') {
    const {exec} = require('node:child_process');
    exec('sh compile-icons.command',
        (error, stdout, stderr) => {
            console.log(stdout);
            console.log(stderr);
            if (error !== null) {
                console.log(`exec error: ${error}`);
            }
        });

}

const dist = path.join(__dirname, 'dist');

/**
 * Makes sure this directory exists in the dist folder
 * @param {string} directory relative path in ./dist
 */
const ensureDirectory = (directory) => {
    const absolutePath = path.join(dist, directory);
    if (!fs.existsSync(absolutePath)) {
        fs.mkdirSync(absolutePath, {recursive: true});
    } else if (!fs.statSync(absolutePath).isDirectory()) {
        console.error('.dist/' + directory + " is a file not a folder");
        exit(1);
    }
}

/**
 * Checks if file is a unix style hidden file
 * @param {string} src
 * @return {boolean}
 */
const isUnixHidden = (src) => {
    if (src == null) {
        return false;
    }
    const filename = src
        .split('/')
        .flatMap((item) => item.split('\\')).pop();
    return filename.indexOf('.') === 0
}


// Extensions to never copy
const noCopyExtensions = ['.ds_store', '.less'];

/**
 * Checks if a file should be copied
 * @param {string} src file path of file to copy
 * @return {boolean}
 */
const shouldCopyFile = (src) => {
    if (isUnixHidden(src)) {
        return false;
    }
    const extensionsLower = path.extname(src).toLowerCase();
    if (noCopyExtensions.indexOf(extensionsLower) >= 0 || extensionsLower.substring(0, 2) === '._') {
        return false
    }
    const basename = path.basename(src).toLowerCase();
    if (extensionsLower === '.map') {
        if (basename.length > 5 && basename.slice(-6) === 'js.map') {
            return true
        }
        return basename === 'style.css.map';
    }
    if (extensionsLower === '.css') {
        return basename === 'style.css';
    }
    return true;
}

/**
 * Copy a directory in project root to ./dist
 * @param {string} src path relative to project's root
 * @param {function(string, string): boolean|null=} filter
 */
const copyDir = (src, filter) => {
    ensureDirectory(src);
    _fs.copySync(path.join(__dirname, src), path.join(dist, src), {
        filter: filter ?? shouldCopyFile,
        recursive: true
    })
}

/**
 * Copy only files with a given extension
 * @param {string} src
 * @param {string[]} extensions
 */
const copyExtensions = (src, extensions) => {
    const extensionsRegex = '(' + extensions.join('|') + ')';
    const pattern = '.*?\.' + extensionsRegex;
    console.log({pattern});
    const regex = new RegExp(pattern);
    copyDir(src, (file) => {
        if (fs.statSync(file).isDirectory()) {
            return true;
        }
        return file.match(regex) && shouldCopyFile(file);
    });
}

/**
 * Copies index.html while replacing the version placeholder with the current version described in the Package.JSON
 * Placeholder is `{{ _version_ }}`
 */
const copyIndexHTMLUpdatingVersion = () => {
    const version = fs.readFileSync(path.join(__dirname, 'version.txt'), 'utf-8')
        .trim();

    const originalIndexHTMLPath = path.join(__dirname, 'index.html');
    const copyIndexHTMLPath = path.join(dist, 'index.html');
  
    let contents = fs.readFileSync(originalIndexHTMLPath, 'utf-8');

    const versionRegex = /\{\{\s*_version_\s*}}/gi;
    contents = contents.replaceAll(versionRegex, version);

    fs.writeFileSync(copyIndexHTMLPath, contents);
}

// Ensure ./src directory
ensureDirectory('src');

// Copy index.html
copyIndexHTMLUpdatingVersion()

// Copy JS
copyExtensions('src', ['js', 'js\.map']);
// Copy Images
copyDir('src/images', shouldCopyFile);
// Copy CSS
copyDir('src/css', shouldCopyFile);
// Copy assets
copyDir('src/assets', (src) => !isUnixHidden(src));