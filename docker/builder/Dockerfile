FROM amazonlinux:2
WORKDIR /app

# set up rusttoolchain
RUN yum update -y && yum install gcc -y && yum install openssl-devel -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"
