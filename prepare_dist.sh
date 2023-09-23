cd $(dirname "$0") || exit 1;

if [ ! -d "dist/" ]; then
  mkdir -p dist/src/images;
fi
cp index.html dist/;
rsync -avm src/ dist/src/ --include \*/ --include \*.js --exclude \*;

if [ ! -d "dist/src/css" ]; then
  mkdir -p ./dist/src/css/;
fi
cp src/css/style.css ./dist/src/css/

if [ ! -d "dist/src/images" ]; then
  mkdir -p dist/src/images;
fi
rsync -avm src/images/ dist/src/images/ --exclude \.DS_Store;


if [ ! -d "dist/src/assets" ]; then
  mkdir -p dist/src/assets;
fi
rsync -avm src/images/ dist/src/images/ --exclude \.DS_Store;