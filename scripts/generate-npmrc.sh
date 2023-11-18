if [ -z $GITHUB_PKG_TOKEN ]; then
  echo "GITHUB_PKG_TOKENの環境変数が設定されていません"
  exit -1
fi

export GITHUB_PKG_TOKEN=$GITHUB_PKG_TOKEN
cat ./scripts/.npmrc.template | envsubst > .npmrc