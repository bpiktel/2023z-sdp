FROM surrealdb/surrealdb:v1.2.1 AS final

ENV SURREAL_AUTH=false
ENV SURREAL_STRICT=false
ENV SURREAL_NO_BANNER=true
ENV SURREAL_LOG=info
ENV SURREAL_PATH=file://data

USER root

VOLUME [ "/data" ]

ENTRYPOINT [ "/surreal", "start" ]
