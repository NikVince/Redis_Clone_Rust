pipeline {
    agent any

    environment {
        // Add the jenkins user's Rust installation to the PATH
        PATH = "/var/lib/jenkins/.cargo/bin:${env.PATH}"
    }
    
    options {
        // Add timestamps to console logs
        timestamps()
        // Discard old builds, keeping only recent ones
        buildDiscarder(logRotator(numToKeepStr: '10'))
        // Timeout if build takes too long
        timeout(time: 30, unit: 'MINUTES')
    }
    
    stages {
        stage('Checkout') {
            steps {
                // Clean workspace before checkout
                cleanWs()
                checkout scm
                
                // Output git info for debugging
                sh '''
                git branch -v
                git remote -v
                echo "Git commit: ${GIT_COMMIT}"
                echo "Branch name: ${BRANCH_NAME}"
                '''
            }
        }
        
        stage('Check Rust Installation') {
            steps {
                sh '''
                rustc --version
                cargo --version
                '''
            }
        }
        
        stage('Build') {
            steps {
                sh '''
                # Build in debug mode
                cargo build
                
                # Build in release mode
                cargo build --release
                '''
            }
        }
        
        stage('Test') {
            steps {
                sh '''
                # Run tests with verbose output
                cargo test -- --nocapture
                '''
            }
        }
        
        stage('Quality Checks') {
            parallel {
                stage('Clippy') {
                    steps {
                        sh '''
                        # Make sure clippy is installed
                        rustup component add clippy
                        
                        # Run clippy with warnings as errors
                        cargo clippy -- -D warnings
                        '''
                    }
                }
                
                stage('Format Check') {
                    steps {
                        sh '''
                        # Make sure rustfmt is installed
                        rustup component add rustfmt
                        
                        # Check if code is properly formatted
                        cargo fmt -- --check
                        '''
                    }
                }
                
                stage('Security Audit') {
                    steps {
                        sh '''
                        # Install cargo-audit if not already installed
                        cargo install cargo-audit || true
                        
                        # Run security audit on dependencies
                        cargo audit
                        '''
                    }
                }
            }
        }
        
        stage('Documentation') {
            steps {
                sh '''
                # Generate documentation
                cargo doc --no-deps
                '''
            }
        }
        
        stage('Merge to Main') {
            when {
                branch 'test'
            }
            steps {
                echo "Starting merge process..."
                // Use withCredentials to securely handle GitHub credentials
                withCredentials([usernamePassword(credentialsId: 'github-auth', 
                                passwordVariable: 'GIT_PASSWORD', 
                                usernameVariable: 'GIT_USERNAME')]) {
                    sh '''
                    # Configure Git user identity for the merge
                    git config user.name "Jenkins"
                    git config user.email "jenkins@example.com"
                    
                    # Fetch latest changes from both branches
                    git fetch origin main:refs/remotes/origin/main
                    git fetch origin test:refs/remotes/origin/test
                    
                    # Checkout main
                    git checkout -f origin/main
                    
                    # Merge the test branch
                    git merge --no-ff origin/test -m "Auto-merge test to main [Jenkins]"
                    
                    # Push to main with credentials
                    git push https://${GIT_USERNAME}:${GIT_PASSWORD}@github.com/NikVince/Redis_Clone_Rust.git HEAD:main
                    
                    # Return to original branch
                    git checkout ${BRANCH_NAME}
                    '''
                }
            }
        }
    }
    
    post {
        always {
            // Clean workspace after build
            cleanWs(cleanWhenNotBuilt: false,
                    deleteDirs: true,
                    disableDeferredWipeout: true,
                    notFailBuild: true,
                    patterns: [[pattern: 'target/', type: 'INCLUDE']])
        }
        success {
            echo 'Build successful!'
        }
        failure {
            echo 'Build failed!'
        }
    }
}
