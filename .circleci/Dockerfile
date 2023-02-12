FROM cimg/rust:1.67.0

# Set root user as apt does not work otherwise
USER root
RUN apt-get update \
  && apt-get install libasound2-dev libudev-dev \
  && apt-get autoremove \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

# Return to CI user after done
USER circleci
