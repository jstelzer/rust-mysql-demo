FROM centos:7
RUN yum makecache
RUN yum -y upgrade
RUN yum -y group install "Development Tools"
RUN yum  -y install openssl-devel
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust-inst.sh
RUN bash /tmp/rust-inst.sh -y
RUN echo ". ~/.cargo/env" >> ~/.bashrc
LABEL com.neverlight.version="0.0.1-beta" com.neverlight.release-date="latest"