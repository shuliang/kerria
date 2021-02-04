pipeline {
  agent any
  stages {
    stage('build') {
      agent {
        dockerfile {
          filename 'release.Dockerfile'
        }

      }
      steps {
        echo 'building...'
      }
    }

    // stage('deploy') {
    //   steps {
    //     echo 'deploying...'
    //     sh './jenkins/scripts/deploy.sh'
    //   }
    // }

    stage('clean') {
      steps {
        cleanWs(cleanWhenAborted: true, cleanWhenFailure: true, cleanWhenSuccess: true, cleanWhenNotBuilt: true, disableDeferredWipeout: true, deleteDirs: true, cleanupMatrixParent: true, cleanWhenUnstable: true)
      }
    }

  }
}
