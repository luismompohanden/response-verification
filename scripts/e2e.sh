# Download the SDK repo so we can build and test against the latest changes
download_sdk_repo() {
  SDK_REPO_DIR="$(pwd)/tmp/sdk"

  if [ -d "$SDK_REPO_DIR" ]; then
    echo "SDK repo already cloned, updating..."

    pushd "$SDK_REPO_DIR" || clean_exit
    git fetch
    git pull
    popd || clean_exit
  else
    echo "SDK repo not cloned yet, cloning..."

    git clone "https://github.com/dfinity/sdk" "$SDK_REPO_DIR"
  fi

}

build_dfx() {
  echo "Building DFX..."

  if [ -z "$SDK_REPO_DIR" ]; then
    echo "SDK_REPO_DIR must be defined!"
    clean_exit
  fi

  pushd "$SDK_REPO_DIR" || clean_exit
  cargo build -p dfx
  DFX="$(pwd)/target/debug/dfx"
  popd || clean_exit

  echo "DFX built at $DFX."
}

dfx_start() {
  echo "Starting DFX..."

  if [ -z "$DFX" ]; then
    echo "DFX must be defined!"
    clean_exit
  fi

  "$DFX" start --background

  DFX_REPLICA_PORT=$("$DFX" info replica-port)
  DFX_REPLICA_ADDRESS="http://localhost:$DFX_REPLICA_PORT"
  IC_ROOT_KEY=$($DFX ping $DFX_REPLICA_ADDRESS | sed -n 's/.*"root_key": \(.*\)/\1/p' | sed 's/[][,]//g' | xargs printf "%02x")

  echo "DFX local replica running at $DFX_REPLICA_ADDRESS."
}

dfx_stop() {
  echo "Stopping DFX..."

  if [ -z "$DFX" ]; then
    echo "DFX must be defined!"
    clean_exit
  fi

  "$DFX" stop
}

deploy_dfx_project() {
  echo "Deploying DFX project..."

  if [ -z "$DFX" ]; then
    echo "DFX must be defined!"
    clean_exit
  fi

  DFX_PROJECT_DIR="$(pwd)/packages/ic-response-verification-tests/dfx-project"

  pushd "$DFX_PROJECT_DIR" || clean_exit
  npm i
  "$DFX" deploy

  echo "getting canister id..."
  "$DFX" canister id frontend
  DFX_CANISTER_ID=$("$DFX" canister id frontend)
  echo "$DFX_CANISTER_ID"
  popd || clean_exit
}

clean_exit() {
  echo "Performing clean exit..."

  dfx_stop

  exit 1
}

run_e2e_tests() {
  echo "Running e2e tests..."

  if [ -z "$DFX_REPLICA_ADDRESS" ]; then
    echo "$DFX_REPLICA_ADDRESS must be defined!"
    clean_exit
  fi

  if [ -z "$DFX_CANISTER_ID" ]; then
    echo "DFX_CANISTER_ID must be defined!"
    clean_exit
  fi

  if [ -z "$IC_ROOT_KEY" ]; then
    echo "IC_ROOT_KEY must be defined!"
    clean_exit
  fi

  IC_ROOT_KEY=$IC_ROOT_KEY DFX_REPLICA_ADDRESS=$DFX_REPLICA_ADDRESS RUST_BACKTRACE=1 cargo run -p ic-response-verification-tests -- $DFX_CANISTER_ID || clean_exit
}

download_sdk_repo
build_dfx
dfx_start
deploy_dfx_project
run_e2e_tests
dfx_stop
