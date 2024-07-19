FROM rockylinux:8
LABEL author=admin@veilid.org
ENV IS_NIGHTLY false

COPY repobuild.sh repobuild.sh
RUN dnf install -y createrepo rpm-build rpm-sign yum-utils
RUN chmod +x repobuild.sh


ENTRYPOINT ["./repobuild.sh"]