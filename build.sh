ip=$(ip -o route get to 8.8.8.8 | sed -n 's/.*src \([0-9.]\+\).*/\1/p')
echo "==> using ip $ip"
cross build --release --target x86_64-unknown-linux-musl
echo "==> 👷 lets build the image..."
    DOCKER_BUILDKIT=1 docker build -t ecap .
