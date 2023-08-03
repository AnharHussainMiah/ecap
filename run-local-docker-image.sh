ip=$(ip -o route get to 8.8.8.8 | sed -n 's/.*src \([0-9.]\+\).*/\1/p')
docker run \
    --rm \
    -it \
    --name ecap \
    -e DATABASE_URL="postgres://dev:password@$ip/ecap" \
    -e API_KEY="63cad126-7373-45dd-a075-8687b148aeeb" \
    -p 8080:8080 \
    ecap