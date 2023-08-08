build-frontend:
    cd apps/frontend && npm install
    cd apps/frontend && npm run build 

build-backend:
    cd apps/backend/backend-server && cargo build --release

package: build-frontend build-backend
    rm -rf build-app
    mkdir build-app

    # Get backend binary
    cp apps/backend/backend-server/target/release/backend-server build-app

    # Get frontend site
    cp -r apps/frontend/dist/digital-craftsman build-app

run-app: package
    cd build-app && RUST_LOG=debug ./backend-server

deploy: 
    flyctl deploy --remote-only

