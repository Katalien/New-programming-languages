# Multithread Go application that processes multiple HTTP requests

Fetching Data from Multiple URLs Concurrently:

1. The code reads a list of URLs from a text file named `urls.txt`. Each URL is assumed to be on a separate line in the file.

2. It then launches a separate goroutine for each URL to fetch data from that URL concurrently.

3. The `fetchData` function is responsible for making an HTTP GET request to a given URL and handling the response. It checks for errors during the request and response phases, logging errors and capturing the results.

## Deployment

```bash
docker build -t threads_go_image .
docker run -p 8080:8080 threads_go_image
```
