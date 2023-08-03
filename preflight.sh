curl \
    -vvvvv \
    -X OPTIONS \
    -H "Access-Control-Request-Method: POST" \
    -H "Access-Control-Request-Headers: x-api-key" \
    -H "Origin: https://foo.bar.org" \
    http://localhost:8080/submit