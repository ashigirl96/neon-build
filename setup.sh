if [ -f .env ]; then
  source .env
fi

source ./scripts/generate-npmrc.sh