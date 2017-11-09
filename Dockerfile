FROM scratch
MAINTAINER Łukasz Niemier <lukasz.niemier@appunite.com>

ADD geopost /geopost

EXPOSE 5000

ENTRYPOINT ["/geopost"]
CMD ["-h"]
