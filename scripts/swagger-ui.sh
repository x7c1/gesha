#!/usr/bin/env bash

set -xue

# Usage:
# $ ./scripts/swagger-ui.sh

collect_yamls() {
  items=""
  yaml_paths=$(find ./schemas -name "*.yaml" | sort)

  for path in $yaml_paths; do
    name=$(basename $path)
    item="{ url: \"local/$path\", name: \"$name\" }"
    items="$item,$items"
  done

  echo "[$items]"
}

docker run --rm \
  -p 80:8080 \
  -v "${PWD}":/usr/share/nginx/html/local \
  -e URLS="$(collect_yamls)" \
  --platform linux/x86_64 \
  swaggerapi/swagger-ui
