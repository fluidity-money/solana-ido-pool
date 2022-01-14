FROM rust:latest AS solana-tools

RUN sh -c "$(curl -sSfL https://release.solana.com/edge/install)"
ENV PATH "$PATH:/root/.local/share/solana/install/active_release/bin"

COPY docker/test_signer.json /root/.config/solana/id.json


FROM solana-tools AS build-contracts

COPY programs/ programs/
COPY Cargo.* ./

RUN ["cargo", "build-bpf"]


FROM solana-tools AS validator

COPY --from=build-contracts target/ target/
COPY docker/ docker/

EXPOSE 8899/tcp

# for whatever reason, deploying this normally doesn't work - solana waits for blocks and redeploys endlessly
CMD ["solana-test-validator", "--bpf-program", "docker/test_ido_pool_id.json", "target/deploy/ido_pool.so"]


FROM solana-tools AS contracts

ENV REPO ido-pool-solana

ENV DIR /usr/src/${REPO}

RUN ["apt-get", "update"]
RUN ["apt-get", "install", "--assume-yes", "nodejs", "npm"]

WORKDIR ${DIR}

COPY . .

RUN cd scripts && npm i

CMD ["./docker/deploy-contract.sh"]
