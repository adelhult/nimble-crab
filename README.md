A rough sketch of how Rust bindings for @piot's Nimble networking engine could
look.

So far, it only contains a build script and a potential API for the
`TransmuteVm`. I'm also having some linker issues that I should investigate.

# Temporary setup

```
cd nimble
go run github.com/piot/deps/src/deps@main fetch -t src/deps 
mkdir build
```

(The real bindings would of course check in the dependencies instead of
requiring Go to be installed.)
