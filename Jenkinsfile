pipeline {
  agent none
  stages {
    stage('build') {
      agent {
        docker {
          image 'rust:nightly-bullseye-slim'
          registryUrl: 'https://ghcr.io/rust-lang'
        }
      }
      steps {
        echo "building"
        sh "cargo build"
      }
    }
  }
}