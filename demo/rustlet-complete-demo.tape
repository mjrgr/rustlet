# VHS Rustlet Complete Demo
# This tape demonstrates the main features of Rustlet – a lightweight init container tool.

Output rustlet-complete-demo.gif

# Ensure dependencies exist
Require rustlet

# Terminal appearance / behavior
Set Shell "zsh"
Set FontSize 16
Set Width 1200
Set Height 720
Set Theme "Dracula"
Set BorderRadius 10
Set WindowBar "Rings"
Set PlaybackSpeed 1.5
Set TypingSpeed 40ms
Set Framerate 60

# Clear screen and show title
Type "clear"
Enter
Sleep 1s

Type "echo 'Rustlet Complete Demo - Lightweight Init Container Tool'"
Enter
Sleep 2s

Type "echo '============================================================'"
Enter
Sleep 1s

############################
# 1. Version & help
############################

Type "echo ''"
Enter
Type "echo '1. First, let''s check the version and see available options:'"
Enter
Sleep 1s

Type "rustlet --version"
Enter
Sleep 2s

Type "rustlet --help"
Enter
Sleep 4s

############################
# 2. HTTP health check (success)
############################

Type "echo ''"
Enter
Type "echo '2. HTTP Health Check Demo - Checking a healthy endpoint:'"
Enter
Sleep 1s

Type "rustlet --url https://httpbin.org/status/200 --timeout 10 --interval 2"
Enter
Sleep 6s

############################
# 3. HTTP health check (timeout / failure)
############################

Type "echo ''"
Enter
Type "echo '3. Failed HTTP Check Demo - This will timeout:'"
Enter
Sleep 1s

Type "rustlet --url https://httpbin.org/delay/15 --timeout 5 --interval 2"
Enter
Sleep 3s
Ctrl+C
Sleep 2s

############################
# 4. TCP check
############################

Type "echo ''"
Enter
Type "echo '4. TCP Health Check Demo - Checking Google DNS (UDP/TCP port 53):'"
Enter
Sleep 1s

Type "rustlet --tcp 8.8.8.8:53 --timeout 5 --interval 2"
Enter
Sleep 4s

############################
# 5. Multiple checks combined
############################

Type "echo ''"
Enter
Type "echo '5. Multiple Checks Demo - HTTP + TCP combined:'"
Enter
Sleep 1s

Type "rustlet --url https://httpbin.org/status/200 --tcp 8.8.8.8:53 --timeout 5 --interval 3"
Enter
Sleep 6s

############################
# 6. Logging levels
############################

Type "echo ''"
Enter
Type "echo '6. Debug Logging Demo - More verbose output:'"
Enter
Sleep 1s

Type "rustlet --url https://httpbin.org/status/200 --level debug --timeout 5"
Enter
Sleep 5s

############################
# 7. Failed check with retry / Ctrl+C
############################

Type "echo ''"
Enter
Type "echo '7. Failed Check with Retry Demo - Will keep trying until Ctrl+C:'"
Enter
Sleep 1s

Type "rustlet --tcp 192.0.2.1:9999 --interval 2 --timeout 3"
Enter
Sleep 4s
Ctrl+C
Sleep 2s

############################
# 8. Kubernetes init container example
############################

Type "echo ''"
Enter
Type "echo '8. Kubernetes Init Container Usage Example:'"
Enter
Sleep 1s

Type "cat << 'EOF'"
Enter
Type "# Example Kubernetes Pod with Rustlet init container"
Enter
Type "apiVersion: v1"
Enter
Type "kind: Pod"
Enter
Type "metadata:"
Enter
Type "  name: my-app"
Enter
Type "spec:"
Enter
Type "  initContainers:"
Enter
Type "  - name: rustlet"
Enter
Type "    image: mehdijrgr/rustlet:latest"
Enter
Type "    args:"
Enter
Type "    - '--url'"
Enter
Type "    - 'http://database:5432/health'"
Enter
Type "    - '--tcp'"
Enter
Type "    - 'redis:6379'"
Enter
Type "    - '--timeout'"
Enter
Type "    - '30'"
Enter
Type "  containers:"
Enter
Type "  - name: app"
Enter
Type "    image: my-app:latest"
Enter
Type "    ports:"
Enter
Type "    - containerPort: 8080"
Enter
Type "EOF"
Enter
Sleep 4s

############################
# 9. Docker usage example
############################

Type "echo ''"
Enter
Type "echo '9. Docker Usage Example:'"
Enter
Sleep 1s

Type "echo 'docker run --rm mehdijrgr/rustlet:latest \\"
Enter
Type "      --url https://api.service.com/health \\"
Enter
Type "      --tcp db:5432 \\"
Enter
Type "      --timeout 30 --interval 5 --level info'"
Enter
Sleep 3s

############################
# 10. Real-world init container scenario
############################

Type "echo ''"
Enter
Type "echo '10. Real-world Init Container Scenario:'"
Enter
Type "echo '    Waiting for database, cache and API gateway to be ready...'"
Enter
Sleep 1s

Type "rustlet --tcp postgres:5432 --tcp redis:6379 --url http://api-gateway:8080/health --interval 3 --timeout 10 --level info"
Enter
Sleep 5s
Ctrl+C
Sleep 2s

############################
# Summary
############################

Type "echo ''"
Enter
Type "echo 'Rustlet Features Demonstrated:'"
Enter
Sleep 1s

Type "echo '  - Fast HTTP and TCP health checks'"
Enter
Sleep 500ms

Type "echo '  - Configurable retry intervals and timeouts'"
Enter
Sleep 500ms

Type "echo '  - Multiple logging levels (debug, info, warn, error)'"
Enter
Sleep 500ms

Type "echo '  - Graceful shutdown with Ctrl+C'"
Enter
Sleep 500ms

Type "echo '  - Multiple endpoint checks simultaneously'"
Enter
Sleep 500ms

Type "echo '  - Ideal for Kubernetes init containers and Dockerized services'"
Enter
Sleep 500ms

Type "echo '  - Built in Rust for performance and reliability'"
Enter
Sleep 2s

############################
# End of demo
############################

Type "echo ''"
Enter
Type "echo 'Demo Complete! Get Rustlet at: https://github.com/mjrgr/rustlet'"
Enter
Sleep 3s

Type "echo 'Thanks for watching!'"
Enter
Sleep 2s