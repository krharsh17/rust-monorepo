VERSION 0.5

all-docker:
    BUILD ./services/ingest+docker
    BUILD ./services/cleanup+docker

all-unit-test:
    BUILD ./libs/filter+test
    BUILD ./services/cleanup+test
    BUILD ./services/ingest+test
