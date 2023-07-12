FROM python:3.9-bookworm

WORKDIR /app

COPY config ./config
COPY scripts ./scripts
COPY src ./src
COPY requirements.txt ./

RUN scripts/install.sh && pip cache purge

CMD scripts/run.sh
