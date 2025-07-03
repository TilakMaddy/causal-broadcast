run-nodes:
    @echo "Running nodes.."
    npx concurrently \
        "RUST_LOG=info cargo su 0" \
        "RUST_LOG=info cargo su 1" \
        "RUST_LOG=info cargo su 2" \
        "RUST_LOG=info cargo su 3" \
        "RUST_LOG=info cargo su 4"

concurrent-broadcast:
    @echo "Requesting concurrent broadcast at nodes 2 and 4.."
    @curl --request POST \
      --url http://0.0.0.0:3002/broadcast \
      --header 'Content-Type: application/json' \
      --data '{ "message": "Hello from concurrent request - part 1" }' & \
    curl --request POST \
      --url http://0.0.0.0:3004/broadcast \
      --header 'Content-Type: application/json' \
      --data '{ "message": "Hello from concurrent request - part 2" }'

sequential-broadcast:
    @echo "Requesting broadcast at node 2.."
    @curl --request POST \
      --url http://0.0.0.0:3002/broadcast \
      --header 'Content-Type: application/json' \
      --data '{ "message": "Hello from sequential reuqest - part 1" }' &&  \
    echo "Waiting..." && \
    sleep 5 && \
    echo "Requesting broadcast at node 4.." && \
    curl --request POST \
      --url http://0.0.0.0:3004/broadcast \
      --header 'Content-Type: application/json' \
      --data '{ "message": "Hello from sequential reuqest - part 2" }'

broadcast:
    @echo "Requesting broadcast at node 3 .."
    @curl --request POST \
      --url http://0.0.0.0:3003/broadcast \
      --header 'Content-Type: application/json' \
      --data '{ "message": "Hello from single broadcast request" }'
