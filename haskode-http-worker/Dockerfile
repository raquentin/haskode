FROM haskell:8

WORKDIR /opt/example

RUN cabal update

# Add just the .cabal file to capture dependencies
COPY ./haskode-http-worker.cabal /opt/haskode-http-worker/haskode-http-worker.cabal

# Docker will cache this command as a layer, freeing us up to
# modify source code without re-installing dependencies
# (unless the .cabal file changes!)
RUN cabal build --only-dependencies -j4

# Add and Install Application Code
COPY . /opt/haskode-http-worker
RUN cabal install

CMD ["haskode-http-worker"]
