VERSION 0.5

all-docker:
    BUILD ./libs/ingest+docker
    BUILD ./libs/cleanup+docker

all-unit-test:
    BUILD ./libs/filter+test
    BUILD ./services/cleanup+test
    BUILD ./services/ingest+test
