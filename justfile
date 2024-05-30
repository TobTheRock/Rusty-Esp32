ui:
  #!/bin/sh
  cd ui
  elm make src/Main.elm --optimize
  gzip index.html
