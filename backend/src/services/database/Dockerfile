FROM surrealdb/surrealdb:v2.0.2 AS final

ENV SURREAL_UNAUTHENTICATED=true
ENV SURREAL_STRICT=false
ENV SURREAL_NO_BANNER=true
ENV SURREAL_LOG=info
ENV SURREAL_PATH=surrealkv://data

USER root

VOLUME [ "/data" ]

ENTRYPOINT [ "/surreal", "start" ]
